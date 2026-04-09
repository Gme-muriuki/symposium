# Important flows

This page traces two key flows: how skills reach the agent, and how hooks react to agent behavior.

* [Symposium start flow](./symposium-start-flow.md) — how `symposium start` loads plugins, matches skills to workspace dependencies, and returns guidance to the agent.
* [Skill nudge flow](./skill-nudge-flow.md) — how Symposium monitors prompts and tool use to nudge the agent toward loading relevant crate skills.
* [Cargo fmt reminder flow](./cargo-fmt-reminder-flow.md) — how Symposium detects Rust file changes and reminds the agent to run `cargo fmt`.
