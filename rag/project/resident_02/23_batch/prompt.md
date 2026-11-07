Write the technical specification for the module "{{ module.Name }}" (build order {{ module.Order }}).

Context: The system is split into modules that are implemented independently by coding agents, each against defined interfaces and responsibilities. A coding agent will implement this module in Rust from your specification alone and will not see any other specification. It must be compact and optimized for salience.

## Responsibilities
{% for item in module.Responsibilities %}
- {{ item }}
{%- endfor %}

## Requirements
{% for item in module.Requirements %}
- {{ item }}
{%- endfor %}

## Dependencies
{% if module.Dependencies %}
{% for dep in module.Dependencies %}
- {{ dep }}
{%- endfor %}
{% else %}
None.
{% endif %}

## Prior specifications
Read the specifications of the dependencies with `read_specifications` before writing. Beyond that, `read_specifications` (parameter `modules`) gives access to all specifications created so far; consult any that are relevant to this module, and continue without those still pending.

Use what you read exactly as specified; do not redefine it. Carry over into your specification whatever this module relies on (signatures, behavior, guarantees, constraints, internals where they matter).

## Rules
- Stay within the responsibilities, requirements and the specifications you read. Invent no responsibilities or behavior beyond them.
- Aim for the solution that fits this module and its context best. Conventions are not a topic of their own: they emerge from how the module fits together with the specifications you read.
- The specification must leave the implementer no open design question that matters for the fit. Resolve it in the specification; leave only details the implementation run decides, such as local names and internal structure.
- Specify the public interface as Rust signatures (types, traits, functions, ownership and mutability, error types, `Send`/`Sync` bounds).
- State observable behavior, invariants, edge cases and concurrency semantics as short, testable statements. Omit internal design unless a requirement prescribes it or a dependent relies on it.

## Output
Only the finished specification as Markdown, with these sections:
1. Task, restrictions and boundaries
2. Condensed relevant Context Information (Role of this module in the system and its relation to neighboring modules)
3. Interface (Rust signatures)
4. Behavior and invariants (including concurrency and errors)
5. Relied-on elements (Concrete elements taken from other specifications (signatures, guarantees) that the implementer needs verbatim; omit if none)