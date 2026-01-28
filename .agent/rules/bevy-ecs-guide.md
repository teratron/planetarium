---
trigger: always_on
---

# Bevy ECS Principles

This guide outlines the core principles of the Entity Component System (ECS) in the Bevy game engine.

## Core Concepts

### 1. Entities

Entities are simple unique identifiers (integers). They don't contain data or logic themselves but act as containers for components.

- In Bevy, this is represented by the `Entity` struct.
- Spawn entities using `Commands`: `commands.spawn((ComponentA, ComponentB));`.

### 2. Components

Components are normal Rust structs that store data.

- They must implement the `Component` trait, usually via `#[derive(Component)]`.
- **Best Practice**: Keep components small and focused to encourage code reuse (e.g., `Name(String)` instead of a large `Person` struct with many fields).

### 3. Systems

Systems are normal Rust functions that contain the game logic.

- They run on data provided by the Bevy `World`.
- Parameters define what data the system accesses (Queries, Resources, Commands).
- Systems run in parallel by default to maximize performance.

## Working with Systems

### Queries

Queries allow you to iterate over entities that have a specific set of components.

- **Immutable**: `Query<&Name>`
- **With Filters**: `Query<&Name, With<Person>>` (only entities with both `Name` and `Person`).
- **Mutable**: `Query<&mut Name>`. Requires the system parameter itself to be `mut query: Query<...>` and using `&mut` in the query type.

### Commands

Used to modify the `World` from within a system (e.g., spawning/despawning entities, adding/removing components).

- Parameter: `mut commands: Commands`.

### Startup Systems

Systems that run exactly once when the app starts.

- Added via `.add_systems(Startup, system_name)`.

### System Ordering

- By default, systems run in parallel.
- Use `.chain()` to force systems to run in a specific order:

  ```rust
  app.add_systems(Update, (system_a, system_b).chain());
  ```

## App Construction

Register systems in the `App` using `add_systems` with the appropriate `Schedule` (e.g., `Update`, `Startup`).

```rust
fn main() {
    App::new()
        .add_systems(Startup, setup_system)
        .add_systems(Update, (logic_system, render_system).chain())
        .run();
}
```
