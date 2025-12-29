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

