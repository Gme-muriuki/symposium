# Symposium start flow

When the agent starts a Rust task, a static skill (installed by the Claude Code plugin) tells it to run `symposium start`. The resulting message (a) lists skills that are available for the workspcae and (b) encourages the agent to run `symposium crate` to look a particular crate.

```mermaid
sequenceDiagram
    participant Agent
    participant Symposium
    participant Plugins as Plugin Sources
    participant Workspace as Cargo.toml

    Note over Agent: Agent begins a Rust task
    Agent->>Symposium: symposium start
    Symposium->>Plugins: load plugin manifests (TOML)
    Symposium->>Workspace: read workspace dependencies
    Symposium->>Symposium: match skills to dependencies
    Symposium-->>Agent: Rust guidance + list of available crate skills

    Note over Agent: Agent sees a relevant crate skill
    Agent->>Symposium: symposium crate <name>
    Symposium->>Plugins: resolve skill source (local or git)
    Symposium->>Symposium: parse SKILL.md, evaluate predicates
    Symposium-->>Agent: crate-specific guidance
```

The start command returns general Rust guidance plus a list of skills that match the workspace's dependencies. The agent can then load individual crate skills as needed. Skills marked `always` are inlined in the start output; `optional` skills are listed with metadata so the agent can choose when to load them.

**Skill resolution** works in layers: plugin sources (configured in `config.toml`) provide plugin manifests, each manifest declares skill groups with crate predicates, and each `SKILL.md` can further narrow with its own `crates` frontmatter. Both levels must match (AND logic), which avoids fetching skill directories that can't possibly apply.
