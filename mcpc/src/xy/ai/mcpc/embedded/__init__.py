"""Run an MCPC server inline from a Python script and drive agents against it.

One MCPC shared by parallel agents (each agent gets its own session and toolset)::

    class MyTools(ToolProvider):
        def tools(self, environment):
            return [FunctionTool("lookup", "Look up a record", {"type": "object", "properties": {}}, lambda ctx: "ok")]

    with EmbeddedMcpc(builtin_tools=["read"], tools=[MyTools()], auto_approve=True) as mcpc:
        results = mcpc.run_batch(prompts, "You process records.", tools={"lookup", "read"}, max_workers=8)
        with mcpc.start_agent("You are a reviewer.") as agent:
            text = agent.ask("Review foo.py")

A dedicated, specially configured MCPC per agent::

    with launch_agent("You are a reviewer.", builtin_tools=["read"], cc_profile="review") as agent:
        text = agent.ask("Review foo.py")
"""
from xy.ai.mcpc.embedded.agent import Agent, CacheMode, CliResult, CliSessionError, Effort, Model
from xy.ai.mcpc.embedded.mcpc import EmbeddedMcpc, launch_agent
from xy.ai.mcpc.embedded.tools import FunctionTool, ToolProvider
__all__ = [
    'Agent',
    'CacheMode',
    'CliResult',
    'CliSessionError',
    'EmbeddedMcpc',
    'Effort',
    'FunctionTool',
    'Model',
    'ToolProvider',
    'launch_agent']