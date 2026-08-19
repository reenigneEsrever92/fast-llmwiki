# Directory Update Log

## 2026-08-19
* **Creation**: Added an `okf install` subcommand that writes the agent skills embedded in the binary (via an `include_str!` macro) into `.agents/skills`.
* **Update**: Brought the top-level documentation and README in line with the current code — the single-socket `okf` launcher, the six-crate layout, the `/api/tree` endpoint, and semantic search.
* **Update**: Renamed the `tasks` concept to `plans` (directory, `type: Plan`, and all references), scoped `okf-spec` to producing only specs, and added an `okf-plan` skill for the technical approach and implementation steps.

## 2026-08-18
* **Creation**: Established the project documentation as an OKF bundle, covering the format, the REST API, the trust model, and the web UI.
* **Creation**: Added the development section (roadmap, specs, tasks, decisions) and the `okf-dev` / `okf-spec` agent skills.
