# Task Management Standards

These standards define how tasks should be generated, formatted, and organized within the project. All task lists (e.g., `tasks.md`) and task generation workflows must adhere to these rules.

## Checklist Format (REQUIRED)

Every task MUST strictly follow this format:

```text
- [ ] [TaskID] [P?] [Story?] Description with file path
```

### Format Components

1. **Checkbox**: ALWAYS start with `- [ ]` (markdown checkbox).
2. **Task ID**: Sequential number (T001, T002, T003...) in execution order.
3. **[P] marker**: Include ONLY if task is parallelizable (different files, no dependencies on incomplete tasks).
4. **[Story] label**: REQUIRED for user story phase tasks only.
    - Format: `[US1]`, `[US2]`, `[US3]`, etc. (maps to user stories).
    - **Setup phase**: NO story label.
    - **Foundational phase**: NO story label.
    - **User Story phases**: MUST have story label.
    - **Polish phase**: NO story label.
5. **Description**: Clear action with exact file path.

### Examples

- ✅ CORRECT: `- [ ] T001 Create project structure per implementation plan`
- ✅ CORRECT: `- [ ] T005 [P] Implement authentication middleware in src/middleware/auth.py`
- ✅ CORRECT: `- [ ] T012 [P] [US1] Create User model in src/models/user.py`
- ❌ WRONG: `- [ ] Create User model` (missing ID and Story label)
- ❌ WRONG: `T001 [US1] Create model` (missing checkbox)

## Phase Structure

Tasks should be organized into the following phases:

1. **Phase 1: Setup**: Project initialization and shared infrastructure.
2. **Phase 2: Foundational**: Blocking prerequisites (MUST complete before user stories).
3. **Phase 3+: User Stories**: Dedicated phases for each user story in priority order (P1, P2...).
    - Within each story: Tests (if requested) → Models → Services → Endpoints → Integration.
    - Each phase should be a complete, independently testable increment.
4. **Final Phase**: Polish & Cross-Cutting Concerns.

## Task Organization Strategies

1. **From User Stories**: Map all related components (Models, Services, Endpoints) to their specific story.
2. **From Contracts**: Map each contract/endpoint to the user story it serves.
3. **From Data Model**: Map each entity to the user story(ies) that need it.
4. **From Setup/Infrastructure**: Shared infrastructure goes to Setup phase; blocking tasks go to Foundational phase.
