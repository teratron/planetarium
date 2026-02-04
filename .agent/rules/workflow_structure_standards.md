---
trigger: always_on
---

# Workflow Structure Standards

All agent workflows (located in `.agent/workflows/`) must follow this standardized structure to ensure consistency, reliable execution, and proper user input handling.

## File Structure

### 1. YAML Frontmatter

Must be at the very top of the file and include a `description`.

```yaml
---
description: [Short action-oriented description]
---
```

### 2. User Input Section

Must immediately follow the frontmatter to capture and display arguments.

```markdown
## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

```

### 3. Outline Section
A numbered, sequential list of steps for the agent to execute.

```markdown
## Outline

1. **Setup/Analysis**: Initial checks and context loading.
2. **Execution Steps**: The core logic of the workflow.
3. **Validation**: steps to verify the work.
4. **Report**: Final summary to the user.
```

### 4. Detailed Rules & Context

Subsections defining specific constraints, formats, or templates referenced in the Outline.

- Use **CRITICAL**, **MUST**, or **REQUIRED** for mandatory constraints.
- Provide clear examples (✅ CORRECT vs ❌ WRONG).

## Naming Conventions

- Workflow files should use kebab-case (e.g., `feature-spec.md`, `run-tests.md`).
- Stored in `.agent/workflows/`.

## Best Practices

- **Validation**: Always include a step to validate that the output matches the required format.
- **Context Loading**: Explicitly list which files or docs to read at the start.
- **Robustness**: Anticipate missing files or empty inputs.
