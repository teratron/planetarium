---
description: Memorize a new project rule or guideline.
---

1. **Analyze the Request**:
    - Extract the `<rule for memorization>` from the user's input.
    - Check if the user specified a specific destination file (e.g., "save to `rules/my-rule.md`").

2. **Determine Target Path**:
    - **If a file path is specified**: Use that path (ensure it is absolute or relative to the workspace root).
    - **If NO file path is specified**:
        - Analyze the rule content to generate a short, descriptive, kebab-case filename (e.g., `coding-standards.md`, `git-policy.md`).
        - The target directory is `.agent/rules/`.
        - The full path will be `.agent/rules/<generated-name>.md`.

3. **Check for Existence**:
    - If the user did not specify a file and the generated filename already exists, append a number or refine the name to avoid accidental overwrites, OR ask the user if they want to append/overwrite. (Default to creating a new unique file if unsure).

4. **Create/Update the Rule**:
    - Use `write_to_file` to create the new markdown file.
    - **File Content**:
        - **Language**: Ensure all rules are generated and written in **English**, even if the user request is in another language.
        - Add a H1 header reflecting the topic.
        - Place the rule content clearly below.
    - **Note**: If the user asked to *append* to an existing file (like `user_rules.md`), use `replace_file_content` or `run_command` (to append text), but the default behavior for this workflow is creating a *new* file as per instructions.

5. **Confirmation**:
    - Respond to the user confirming that the rule has been memorized.
    - Mention the full path of the file where the rule was saved.
