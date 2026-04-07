# Cargo workflow

The cargo workflow monitors tools invoked to reduce token usage and (perhaps in the future) trigger other actions.


## Hook flow

Hooks let Symposium react to what the agent is doing. The Claude Code plugin registers three hook events:

```mermaid
sequenceDiagram
    participant Agent
    participant Symposium
    participant Plugin as Plugin Hooks
    participant Session as Session State

    Note over Agent: Agent is about to use a tool
    Agent->>Symposium: symposium hook pre-tool-use (JSON on stdin)
    Symposium->>Symposium: dispatch to builtin hook handlers
    Symposium->>Plugin: run plugin hook commands
    Plugin-->>Symposium: plugin output, allow with json (exit 0) or block (exit 2)
    Symposium->>Symposium: merge output structs
    Symposium-->>Agent: merged output
```

**PreToolUse** has no built-in logic — it dispatches to plugin-defined hook commands, which receive the event JSON on stdin and can allow or block the action.
