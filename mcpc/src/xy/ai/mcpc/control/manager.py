"""Human-in-the-loop tool control manager.

Implements the Manager Pattern for intercepting tool calls at two points:

1. *Before* execution (``phase="request"``): the interceptor may approve,
   modify arguments, or reject the call entirely.
2. *After* execution (``phase="result"``): the interceptor may approve,
   replace the tool output, or inject instructions as the result.

The control endpoint at ``/control/tool`` lets an external client poll for
pending items and post approval decisions.  The intercepting threads block on
per-item ``threading.Event`` objects until a decision arrives.
"""

from __future__ import annotations

import logging
import threading
import uuid
from dataclasses import dataclass, field
from typing import Any
from xy.ai.mcpc.session import Session

logger = logging.getLogger("xy.ai.mcpc.control")

# How long (seconds) an intercepted call waits before timing out and auto-approving.
_DEFAULT_TIMEOUT = 24 * 60 * 60.0  # 24 h — matches agent MCP timeout


# ---------------------------------------------------------------------------
# Decision DTO
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class ControlDecision:
    """The outcome of a human review, produced by :meth:`ToolControlManager.process_approvals`."""

    approved: bool
    """True when the call should proceed (possibly with modified data)."""

    rejection_reason: str | None = None
    """Human-readable hint for the agent when ``approved`` is False."""

    modified_arguments: dict[str, Any] | None = None
    """Replacement arguments for the ``request`` phase (``None`` → keep originals)."""

    modified_result: dict[str, Any] | None = None
    """Replacement result dict for the ``result`` phase (``None`` → keep original)."""


# ---------------------------------------------------------------------------
# Internal pending item
# ---------------------------------------------------------------------------

@dataclass
class _PendingItem:
    id: str
    phase: str          # "request" | "result"
    tool_name: str
    arguments: dict[str, Any] | None       # populated in request phase
    result: dict[str, Any] | None          # populated in result phase
    _event: threading.Event = field(default_factory=threading.Event, repr=False)
    _decision: ControlDecision | None = field(default=None, repr=False)

    def to_dict(self) -> dict[str, Any]:
        item: dict[str, Any] = {"id": self.id}
        if self.arguments is not None:
            item["toolName"] = self.tool_name
            item["arguments"] = self.arguments
        if self.result is not None:
            item["result"] = self.result
        return item


# ---------------------------------------------------------------------------
# Manager
# ---------------------------------------------------------------------------

class ToolControlManager:
    """Thread-safe manager for human-in-the-loop tool interception.

    Usage::

        manager = ToolControlManager()

        # In the interceptor thread (blocks until a decision is received):
        decision = manager.submit_request("bash", {"command": "rm -rf /"})

        # In the control endpoint handler (non-blocking):
        pending = manager.get_pending()
        manager.process_approvals([{"id": "…", "approved": True}])
    """

    def __init__(self, timeout: float = _DEFAULT_TIMEOUT) -> None:
        self._timeout = timeout
        self._pending: dict[str, _PendingItem] = {}
        self._lock = threading.Lock()

    # ------------------------------------------------------------------
    # Interceptor-facing API (blocking)
    # ------------------------------------------------------------------

    def submit_request(
        self,
        session: Session,
        tool_name: str,
        arguments: dict[str, Any],
        *,
        auto_approve: bool = False,
    ) -> ControlDecision:
        """Block until the controller approves/rejects a tool-call *request*.

        Returns a :class:`ControlDecision`.  If the decision includes
        ``modified_arguments``, the caller should substitute them before
        invoking the tool handler.
        """
        if auto_approve:
            logger.info(
                "Auto-approving request for %s [%s] (tool-flagged)",
                tool_name, session.id,
            )
            return ControlDecision(approved=True)

        item = self._enqueue(session, "request", tool_name, arguments=arguments, result=None)
        return self._wait(item)

    def submit_result(
        self,
        session: Session,
        tool_name: str,
        result: dict[str, Any],
        *,
        auto_approve: bool = False,
    ) -> ControlDecision:
        """Block until the controller approves/replaces a tool-call *result*.

        Returns a :class:`ControlDecision`.  If the decision includes
        ``modified_result``, the caller should use that instead of the
        original result.

        ``auto_approve`` is a hint set by the *tool itself* (via
        ``ToolResult.auto_approve``), not derived from the shape of the
        result. A tool sets it when it judges its own result carries
        nothing meaningful for a human to review (e.g. a plain success
        acknowledgement, or an unrestricted full-file read). When set, the
        result is approved without involving the controller.
        """
        if auto_approve:
            logger.info(
                "Auto-approving result for %s [%s] (tool-flagged)",
                tool_name, session.id,
            )
            return ControlDecision(approved=True)

        item = self._enqueue(session, "result", tool_name, arguments=None, result=result)
        return self._wait(item)

    # ------------------------------------------------------------------
    # Control-endpoint-facing API (non-blocking)
    # ------------------------------------------------------------------

    def get_pending(self) -> list[dict[str, Any]]:
        """Return serialisable snapshots of all items still awaiting a decision."""
        with self._lock:
            return [item.to_dict() for item in self._pending.values()]

    def process_approvals(self, approvals: list[dict[str, Any]]) -> None:
        """Apply a batch of approval decisions from the control client.

        Each entry in *approvals* must have at minimum an ``"id"`` key.
        Accepted forms:

        * ``{"id": "…"}`` — simple approval, keep original data.
        * ``{"id": "…", "rejected": true, "reason": "…"}`` — rejection.
        * ``{"id": "…", "arguments": {…}}`` — approve with modified arguments.
        * ``{"id": "…", "result": {…}}`` — approve with replaced result.
        """
        for approval in approvals:
            item_id = approval.get("id")
            if not isinstance(item_id, str):
                logger.warning("Approval entry missing 'id', skipped: %s", approval)
                continue
            with self._lock:
                item = self._pending.get(item_id)
            if item is None:
                logger.warning("Unknown approval id %s, skipped", item_id)
                continue

            if approval.get("rejected"):
                decision = ControlDecision(
                    approved=False,
                    rejection_reason=approval.get("reason") or "Rejected by controller",
                )
            else:
                decision = ControlDecision(
                    approved=True,
                    modified_arguments=approval.get("arguments"),
                    modified_result=approval.get("result"),
                )

            item._decision = decision
            with self._lock:
                self._pending.pop(item_id, None)
            logger.info(
                "Dequeued control item %s [%s/%s]: approved=%s",
                item.tool_name, item.phase, item_id, decision.approved,
            )
            item._event.set()

    # ------------------------------------------------------------------
    # Internal helpers
    # ------------------------------------------------------------------

    def _enqueue(
        self,
        session: Session,
        phase: str,
        tool_name: str,
        arguments: dict[str, Any] | None,
        result: dict[str, Any] | None,
    ) -> _PendingItem:
        item_id = str(uuid.uuid4())
        item = _PendingItem(
            id=item_id,
            phase=phase,
            tool_name=tool_name,
            arguments=arguments,
            result=result,
        )
        with self._lock:
            self._pending[item_id] = item
        logger.info("Enqueued control item %s [%s/%s/%s]", tool_name, phase, session.id, item_id)
        return item

    def _wait(self, item: _PendingItem) -> ControlDecision:
        signalled = item._event.wait(timeout=self._timeout)
        with self._lock:
            self._pending.pop(item.id, None)

        if not signalled or item._decision is None:
            # Timeout — auto-approve to avoid hanging the agent forever.
            logger.warning(
                "Control item %s [%s/%s] timed out, auto-approving",
                item.tool_name, item.phase, item.id,
            )
            return ControlDecision(approved=True)

        return item._decision
