# Skill Matching Reference

Skill matching is based on crate predicates.

## Atom forms

An atom is a crate name with an optional version requirement.

Examples:

- `serde`
- `serde>=1.0`
- `tokio^1.40`
- `regex<2.0`
- `serde=1.0`
- `serde==1.0.219`

Semantics:

- bare crate name: any version
- `>=`, `<=`, `>`, `<`, `^`, `~`: standard semver operators
- `=1.0`: compatible-version matching, equivalent to `^1.0`
- `==1.0.219`: exact-version matching

## Usage in matching fields

The `crates` and `applies-when` fields in both `SKILL.md` frontmatter and plugin `[[skills]]` groups accept simple atom lists.

In TOML plugin manifests, `crates` accepts a string or array:

- `crates = "serde"`
- `crates = ["serde", "tokio>=1.40"]`

In SKILL.md frontmatter, `crates` uses comma-separated values:

- `crates: serde`
- `crates: serde, tokio>=1.40`

`applies-when` follows the same atom syntax:

- `applies-when: regex`
- `applies-when = ["tokio", "tokio-stream"]`
