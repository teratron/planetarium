---
trigger: always_on
---

# Agent Lifecycle Workflow

This rule defines the mandatory logical sequence of operations the AI agent must follow when interacting with the project. It ensures consistency, quality, and adherence to project standards.

## 1. Phase: Analysis & Preparation

Before any code generation or modification:

- **Task Identification**: Locate the current task in `tasks.md` and ensure it follows `task_management_standards.md`.
- **Architectural Alignment**: Verify the planned changes adhere to the Bevy ECS paradigm defined in `bevy-ecs-guide.md`.
- **Context Loading**: Read relevant source files and documentation to fully understand dependencies.

## 2. Phase: Implementation

During the coding process:

- **Clean Code**: Implement logic following Rust best practices.
- **Documentation**:
  - Add **docstrings** to all public functions, structs, enums, and traits.
  - Add **explanatory comments** for complex logic blocks.
- **ECS Patterns**: Use small, reusable components and systems as per Bevy standards.
- **Test Creation**:
  - Every new feature or fix MUST have associated tests.
  - Place integration tests in the `tests/` directory.
  - Place unit tests in a `tests` module within the source file if small.

## 3. Phase: Verification & Quality Control

Immediately after code generation:

- **Static Analysis**:
  - Run `cargo check` for compilation errors.
  - Run `cargo clippy -- -D warnings` to address all lints and warnings.
- **Dynamic Analysis**: Run `cargo test` to ensure all tests pass.
- **Formatting**: Run `cargo fmt` to maintain consistent style.

## 4. Phase: Versioning & SemVer

If the changes affect the public API:

- **SemVer Analysis**: Apply the `rust-semver` skill to evaluate the change type (MAJOR, MINOR, PATCH).
- **Metadata Update**:
  - Increment the version in `Cargo.toml` as required.
  - Update the documentation and `CHANGELOG.md` (if exists) reflecting the changes.

## 5. Phase: Synchronization & Completion

At the end of the cycle:

- **Task Management**: Mark the completed task in `tasks.md` with a checkmark `- [x]` according to `task_management_standards.md`.
- **Final Validation**: Perform a quick sanity check of the workspace for any artifacts left over or missing files.
- **Reporting**: Provide a summary of work done, including results of the verification steps in the chat.

---
**CRITICAL**: This lifecycle is non-negotiable. Any deviation must be justified to the user.
