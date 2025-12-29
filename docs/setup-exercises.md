# Setup Exercises in mdBook (mdbook-exercises)

This document explains how to author “Setup” exercises — environment configuration tasks presented with the same interactive UI as regular exercises — and how course servers (e.g., pmcp-run) can add setup‑specific behavior.

## Why a Setup Exercise Type?

- One authoring format: Use the same mdbook-exercises directives you already know.
- Reusable and visible: Setup appears as a first‑class exercise in the book.
- AI‑aware: Course servers can detect setup exercises and provide specialized guidance to students.

## Naming Convention

A setup exercise is identified by its exercise ID. Use a suffix of `-setup` and (recommended) the `00` position within a chapter:

- `ch02-00-environment-setup` (chapter 2, setup before other exercises)
- `ch05-00-docker-setup` (chapter 5, docker setup)

This makes prerequisites clear and lets tools detect setup easily.

## Folder Layout (include‑only pattern)

Keep setup exercises alongside regular exercises under `src/exercises/` and include them from a thin chapter page:

```
src/
├─ exercises/
│  └─ ch02/
│     ├─ environment-setup.md    # ID: ch02-00-environment-setup
│     ├─ hello-mcp.md            # ID: ch02-01-hello-mcp
│     └─ calculator.md           # ID: ch02-02-calculator
└─ part1-foundations/
   ├─ ch02-ex01-hello-mcp.md     # includes exercises/ch02/hello-mcp.md
   ├─ ch02-ex02-calculator.md    # includes exercises/ch02/calculator.md
   └─ ch02-ex00-environment.md   # includes exercises/ch02/environment-setup.md
```

On a chapter page, include the setup exercise:

````markdown
# Environment Setup

{{#exercise ../exercises/ch02/environment-setup.md}}
````

## Authoring a Setup Exercise

Author a setup exercise using the same directives as any other exercise; the “setup” semantics come from the ID suffix `-setup`.

````markdown
# Environment Setup

::: exercise
id: ch02-00-environment-setup
difficulty: beginner
time: 15
:::

Configure your development environment for this course.

::: objectives
doing:
  - Install Rust toolchain
  - Install cargo-pmcp CLI
  - Configure Claude Desktop
:::

::: hint level=1 title="Installing Rust"
Visit https://rustup.rs and run:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
:::

::: hint level=2 title="Installing cargo-pmcp"
```bash
cargo install cargo-pmcp
```
:::

::: hint level=3 title="Claude Desktop Configuration"
Add the MCP server to your Claude Desktop config at:
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\\Claude\\claude_desktop_config.json`
:::

::: tests mode=local
```bash
rustc --version && cargo pmcp --version && echo "PASS" || echo "FAIL"
```
:::

::: reflection
- What version of Rust did you install?
- Did you encounter any platform-specific issues?
:::
````

Notes:
- Use `mode=local` in tests for platform checks. Consider providing OS‑specific hints.
- For cross‑platform shells, provide alternative commands where needed (PowerShell, Cmd).

## Course Server (AI) Behavior (Optional)

When a course server detects an exercise whose ID ends with `-setup`, it can:

1. Detect the student’s OS (Windows/macOS/Linux).
2. Guide step‑by‑step through setup, adapting commands per platform.
3. Verify setup with the exercise’s tests.
4. Troubleshoot by surfacing relevant hints if verification fails.

This makes setup a first‑class learning experience with automated checks.

## Best Practices

- Put the setup exercise at position `00` in each chapter that needs prerequisites.
- Keep steps small and verifiable; prefer short hints and clear tests.
- Use `reveal=on-demand` on solutions if you include canonical installer scripts.
- Keep includes relative and consistent; avoid mixing include and inline styles on the same page.

## See Also

- Authoring and integration guide: `docs/exercises-integration.md`
- Sample mdBook using includes: `sample-book/`
- Multi‑language examples (Python/JS): `examples/`
