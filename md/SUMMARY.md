<!--
    STYLE GUIDELINES:

    - Avoid promotional language: no "rich", "powerful", "easy", etc.
    - No "Benefits" sections - they're promotional by nature
    - No duplicate table of contents at chapter ends
    - Be factual and technical, not persuasive
    - Describe what the system does, not why it's good
-->

# Summary

- [Introduction](./introduction.md)
- [What is Symposium?](./about.md)

# User's guide

- [Installing Symposium](./install.md)
- [Usage patterns](./usage-patterns.md)

# For crate authors

- [Supporting your crate](./crate-authors/supporting-your-crate.md)
- [Publishing skills](./crate-authors/publishing-skills.md)
- [Creating a plugin](./crate-authors/creating-a-plugin.md)
- [Publishing hooks](./crate-authors/publishing-hooks.md)

# Reference

- [The `cargo agents` command](./reference/cargo-agents.md)
  - [`cargo agents init`](./reference/cargo-agents-init.md)
  - [`cargo agents sync`](./reference/cargo-agents-sync.md)
  - [`cargo agents hook`](./reference/cargo-agents-hook.md)
- [Configuration](./reference/configuration.md)
- [Plugin definition](./reference/plugin-definition.md)
- [Skill definition](./reference/skill-definition.md)
- [Skill matching](./reference/skill-matching.md)

# Contribution guide

- [Welcome](./design/welcome.md)
- [Key repositories](./design/repositories.md)
- [Key modules](./design/module-structure.md)
- [Configuration loading](./design/configuration-loading.md)
- [State](./design/state.md)
  - [Session state](./design/session-state.md)
- [Important flows](./design/important-flows.md)
  - [Symposium start](./design/symposium-start-flow.md)
  - [Skill nudges](./design/skill-nudge-flow.md)
  - [Cargo workflow](./design/cargo-workflow-flow.md)
- [Integration test harness](./design/test-harness.md)
- [Governance](./design/governance.md)
- [Common issues](./design/common-issues.md)
