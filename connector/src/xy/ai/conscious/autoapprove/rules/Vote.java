package xy.ai.conscious.autoapprove.rules;

import java.util.ArrayList;
import java.util.List;

/**
 * Result of a single rule. Rules only decide; the approver executes. Besides the
 * verdict a rule may attach metadata: a {@code reason} (relevant for Deny), a
 * {@code hint} (relevant for Allow), {@code warnings} that enrich the answer and
 * an {@code instruction} that is sent to the model as a follow-up prompt.
 */
public record Vote(Verdict verdict, String reason, String hint, List<String> warnings, String instruction) {

	private static final Vote ABSTAIN = new Vote(Verdict.Abstain, null, null, null, null);

	public Vote {
		verdict = verdict == null ? Verdict.Abstain : verdict;
		reason = reason == null ? "" : reason;
		hint = hint == null ? "" : hint;
		warnings = warnings == null ? List.of() : List.copyOf(warnings);
		instruction = instruction == null ? "" : instruction;
	}

	public static Vote abstain() {
		return ABSTAIN;
	}

	public static Vote allow() {
		return allow(null);
	}

	public static Vote allow(String hint) {
		return new Vote(Verdict.Allow, null, hint, null, null);
	}

	public static Vote deny(String reason) {
		return new Vote(Verdict.Deny, reason, null, null, null);
	}

	/** Abstains but adds a warning to the answer. */
	public static Vote warning(String text) {
		return new Vote(Verdict.Abstain, null, null, List.of(text), null);
	}

	/** Abstains but requests a follow-up prompt with the given instruction. */
	public static Vote instruction(String text) {
		return new Vote(Verdict.Abstain, null, null, null, text);
	}

	public Vote withWarning(String text) {
		List<String> all = new ArrayList<>(warnings);
		all.add(text);
		return new Vote(verdict, reason, hint, all, instruction);
	}
}
