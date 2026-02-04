# Post-Task Quality Check

To ensure high code quality and stability, the following steps must be performed **immediately after completing every task**:

1. **Error Check**: Run `cargo check` to verify there are no compilation errors.
2. **Linting**: Address any compiler warnings or clippy lints that appear.
3. **Refactoring**: Review the newly added or modified code for:
    - Readability and clarity.
    - Adherence to Bevy ECS patterns (Components, Systems, Resources).
    - Proper file structure and modularity.
    - Opportunities to simplify or optimize logic.
4. **Guideline Compliance**: Ensure the changes respect the existing project rules (e.g., `bevy-ecs-guide.md`, `rust-semver-guide.md`).

Do not proceed to the next task until the current work is clean, verified, and refactored.

## OS-Specific Execution Variations

Ensure that commands are appropriate for the user's operating system environment.

### 🪟 Windows (PowerShell)

- Use `cargo check` and `cargo clippy` directly.
- Ensure file paths use backslashes `\` or handle them correctly in cross-platform tools.
- Example: `cargo check; if ($?) { cargo clippy }`

### 🐧 Linux (Bash)

- Use standard chained commands.
- Example: `cargo check && cargo clippy`

### 🍎 macOS (Zsh)

- Similar to Linux, ensure environment variables (like `PATH`) are correctly sourced if tools are missing.
- Example: `cargo check && cargo clippy`
