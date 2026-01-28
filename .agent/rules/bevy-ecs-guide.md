---
trigger: always_on
---

# Bevy ECS (Entity Component System) Development Guidelines

## Core Principles

When working with Bevy game engine, always follow the Entity Component System (ECS) paradigm. ECS separates game logic into three core concepts:

1. **Entities** - Unique identifiers (simple integers) representing "things" in your game
2. **Components** - Data structures attached to entities
3. **Systems** - Logic functions that process entities with specific component combinations

## Component Design Rules

**Rule 1:** Components are Rust structs with `#[derive(Component)]`

- Always derive the `Component` trait
- Keep components focused on single responsibilities
- Prefer small, reusable components over large monolithic ones

```rust
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;
```

**Rule 2:** Break data into reusable pieces

- Instead of adding all data to one component, create separate components for reusable concepts
- Example: Both `Person` and `Dog` entities can share a `Name` component

## System Design Rules

**Rule 3:** Systems are normal Rust functions

- Use plain function syntax - no complex traits or macros required
- Systems define what data they operate on through their parameters

```rust
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}
```

**Rule 4:** Use appropriate system schedules

- `Startup` systems: Run once at app initialization - use for setup
- `Update` systems: Run every frame - use for game logic
- Register systems with `add_systems(Schedule, system_function)`

```rust
App::new()
    .add_systems(Startup, add_people)
    .add_systems(Update, greet_people)
    .run();
```

## Query Guidelines

**Rule 5:** Queries define which entities a system processes

- `Query<&Component>` - Read-only access to components
- `Query<&mut Component>` - Mutable access to modify components
- `With<Component>` - Filter to include only entities with specific components

```rust
// Read-only query
fn read_positions(query: Query<&Position>) {
    for position in &query {
        // Read position data
    }
}

// Mutable query
fn update_positions(mut query: Query<&mut Position>) {
    for mut position in &mut query {
        position.x += 1.0;
    }
}

// Filtered query
fn greet_people(query: Query<&Name, With<Person>>) {
    // Only processes entities with both Name and Person components
}
```

**Rule 6:** When mutating data, use `mut` keyword appropriately

- Query parameter must be mutable: `mut query`
- Iterator must be mutable: `&mut query`
- Component reference must be mutable: `mut component`

## Entity Spawning Rules

**Rule 7:** Use Commands to spawn entities

- `Commands` parameter allows deferred entity creation
- Use `spawn()` with component tuples to create entities

```rust
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
}
```

## System Ordering and Parallelism

**Rule 8:** Systems run in parallel by default

- Bevy automatically parallelizes systems when safe
- Don't assume execution order unless explicitly specified

**Rule 9:** Use `.chain()` for explicit ordering

- When one system must run before another, use `.chain()`
- Only chain systems that have dependencies on each other

```rust
App::new()
    .add_systems(Update, (
        hello_world,  // Runs in parallel with the chain
        (update_people, greet_people).chain()  // Runs sequentially
    ))
    .run();
```

**Rule 10:** Group multiple systems efficiently

- Pass multiple systems as tuples to `add_systems`
- Only chain systems with actual dependencies to maximize parallelism

## Best Practices for AI Agent

1. **Always check component requirements** - Before writing a system, identify which components it needs
2. **Favor composition over inheritance** - Use multiple small components rather than complex hierarchies
3. **Consider parallelism** - Only enforce ordering when necessary
4. **Use clear, descriptive names** - Component and system names should indicate their purpose
5. **Keep systems focused** - Each system should have a single, well-defined responsibility
6. **Leverage Bevy's type system** - Use Rust's type safety to prevent errors at compile time

## Memory and Performance Considerations

- ECS optimizes memory access patterns through component grouping
- Parallel execution is automatically optimized by Bevy's scheduler
- Queries are efficient - they only iterate over entities with matching components

When generating Bevy code, always structure it following ECS principles: define components as simple data structs, implement logic in systems, and use queries to access entity data. This approach ensures clean, performant, and maintainable game code.
