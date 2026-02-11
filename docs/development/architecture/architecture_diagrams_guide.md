# Bevy Game Engine Architecture - Visual Diagrams Collection

This document contains comprehensive Mermaid diagrams visualizing the complete Bevy game engine architecture with Framework/Game separation pattern.

---

## 📊 Diagram Index

1. **Main Architecture Diagram** - Complete system overview
2. **State Flow Diagram** - Application state transitions
3. **ECS Interaction Diagram** - Entity-Component-System patterns
4. **Lifecycle Sequence Diagram** - Application execution flow
5. **File Structure Diagram** - Project directory organization
6. **Plugin Architecture Diagram** - Plugin hierarchy and dependencies

---

## 1. Main Architecture Diagram

### Purpose

This diagram provides a complete bird's-eye view of the entire Bevy project architecture, showing:

- **Framework Layer** (reusable infrastructure in blue)
- **Game Layer** (project-specific logic in green)
- **Configuration** (settings and config in orange)
- **Utilities** (shared helper functions in purple)

### Key Concepts Visualized

- **Separation of Concerns**: Framework code is completely independent from game code
- **Plugin Pattern**: Each module is encapsulated as a Bevy plugin
- **ECS Organization**: Game layer strictly follows Components/Systems/Resources pattern
- **Module Hierarchy**: Clear parent-child relationships between modules

### File Location

`bevy_architecture_diagram.mermaid`

### How to Read

- **Blue nodes** = Framework layer (copy to new projects)
- **Green nodes** = Game layer (project-specific)
- **Orange nodes** = Configuration
- **Purple nodes** = Utilities
- **Red nodes** = Plugin bundles
- **Solid arrows** = "contains" relationships
- **Dotted arrows** = "uses" or "depends on" relationships

---

## 2. State Flow Diagram

### Purpose

Visualizes the complete application state machine, showing:

- All possible application states
- Valid transitions between states
- Trigger conditions for transitions (timer, user input, game events)

### States Explained

| State | Color | Purpose | Entry Condition | Exit Condition |
|-------|-------|---------|----------------|----------------|
| **Splash** | Purple | Show company logo | App startup | Timer expires (2s) |
| **Loading** | Orange | Load game assets | After splash | All assets loaded |
| **Main Menu** | Blue | Title screen, options | After loading | User selects option |
| **In Game** | Green | Active gameplay | User starts game | Pause, game over |
| **Paused** | Yellow | Pause menu | ESC key pressed | Resume or quit |
| **Settings** | Cyan | Configuration screen | Settings button | Back button |
| **Game Over** | Red | End screen | Win/lose condition | Retry or quit |

### File Location

`state_flow_diagram.mermaid`

### How to Read

- **Circular nodes** = State entry/exit points
- **Rectangular nodes** = Active states
- **Arrows** = Valid state transitions
- **Arrow labels** = Trigger conditions

---

## 3. ECS Interaction Diagram

### Purpose

Demonstrates the Entity-Component-System pattern in action, showing:

- How entities are composed of components
- How systems query and process components
- How resources provide global state
- Data flow and system dependencies

### ECS Pattern Breakdown

#### Entities (Pink)

- Unique identifiers (just an integer)
- Container for components
- Examples: Player Entity (ID: 42), Enemy Entity (ID: 108)

#### Components (Blue)

- Pure data structures
- Attached to entities
- Examples: Position, Velocity, Health, Player (marker)

#### Systems (Green)

- Functions that process components
- Query entities with specific component combinations
- Examples: Movement System, Combat System, Input System

#### Resources (Orange)

- Global state accessible to all systems
- Not tied to specific entities
- Examples: Score, Time, GameConfig

### System Execution Flow

```
Input System → Movement System → Collision System → Combat System → Rendering System
```

### File Location

`ecs_interaction_diagram.mermaid`

### How to Read

- **Dotted lines** = Entity "has" Component relationship
- **Solid arrows** = System "reads" or "writes" Component/Resource
- **Gray dotted arrows** = System execution order dependencies

---

## 4. Lifecycle Sequence Diagram

### Purpose

Shows the chronological execution flow from application startup through gameplay, including:

- Initialization sequence
- State transition mechanics
- Frame-by-frame update loop
- Plugin registration order

### Execution Phases

#### Phase 1: Startup (One-time)

1. `main()` creates Bevy App
2. Add DefaultPlugins (window, renderer, input)
3. Add FrameworkPlugin (splash, loading, menus)
4. Add GamePlugin (game systems, components)
5. Call `app.run()`

#### Phase 2: Splash State

- **OnEnter**: Setup splash screen, initialize timer
- **Update Loop**: Tick timer every frame
- **OnExit**: Cleanup splash screen, transition to Loading

#### Phase 3: Loading State

- **OnEnter**: Queue assets for loading
- **Update Loop**: Check asset loading progress
- **OnExit**: Transition to Main Menu when complete

#### Phase 4: Game Loop (InGame State)

- **OnEnter**: Spawn player, enemies, level
- **Update Loop** (60 FPS):
  1. Handle player input
  2. Update physics
  3. Check collisions
  4. Update game logic
  5. Render entities
- **OnExit**: Cleanup game entities

### File Location

`lifecycle_sequence_diagram.mermaid`

### How to Read

- **Vertical axis** = Time progression (top to bottom)
- **Horizontal participants** = System components
- **Arrows** = Function calls or messages
- **Boxes** = Execution contexts or loops
- **Notes** = Phase descriptions

---

## 5. File Structure Diagram

### Purpose

Visualizes the complete project directory tree, showing:

- Folder hierarchy
- File organization
- Module structure
- Asset organization

### Directory Structure Breakdown

```
planetarium/
├── assets/              # Binary assets (textures, audio, models)
├── src/
│   ├── framework/       # Reusable infrastructure (BLUE)
│   ├── game/           # Game-specific logic (GREEN)
│   ├── config/         # Configuration (ORANGE)
│   └── utils/          # Shared utilities (PURPLE)
├── tests/              # Integration tests
├── benches/            # Performance benchmarks
└── docs/               # Documentation (RED)
```

### Framework Layer Contents

```
framework/
├── states/         # State management
├── splash/         # Splash screen system
├── loading/        # Asset loading system
├── menu/           # Menu systems (main, pause)
├── settings/       # Settings management
├── audio/          # Audio system
├── camera/         # Camera controllers
└── ui/             # UI utilities
```

### Game Layer Contents

```
game/
├── components/     # ECS components (data)
├── systems/        # ECS systems (logic)
├── resources/      # ECS resources (global state)
├── entities/       # Entity spawner functions
└── constants.rs    # Game constants
```

### File Location

`file_structure_diagram.mermaid`

### How to Read

- **Navy blue nodes** = Root directories
- **Blue nodes** = Framework layer
- **Green nodes** = Game layer
- **Orange nodes** = Configuration
- **Purple nodes** = Utilities
- **Red nodes** = Documentation
- **Gray nodes** = Individual files

---

## 6. Plugin Architecture Diagram

### Purpose

Shows the plugin system hierarchy, including:

- How plugins are bundled together
- Dependencies between plugins
- Plugin registration order
- System execution schedules

### Plugin Hierarchy

#### Framework Plugin Bundle

```
FrameworkPlugin (orchestrator)
├── StatesPlugin (state management)
├── SplashPlugin (depends on States)
├── LoadingPlugin (depends on States)
├── MenuPlugin (depends on States)
├── SettingsPlugin (depends on Audio)
├── AudioPlugin
└── CameraPlugin
```

#### Game Plugin Bundle

```
GamePlugin (orchestrator)
├── SetupPlugin
├── PhysicsPlugin
├── GameplayPlugin (depends on States)
├── InputPlugin (uses Camera)
├── CombatPlugin (depends on Physics)
└── SpawnPlugin (uses Physics)
```

### System Schedules

| Schedule | Runs | Purpose | Examples |
|----------|------|---------|----------|
| **Startup** | Once at app start | Initialize resources, setup | Load config, spawn camera |
| **OnEnter(State)** | When entering a state | State-specific setup | Spawn menu UI, load level |
| **Update** | Every frame | Game logic | Physics, input, rendering |
| **OnExit(State)** | When leaving a state | State cleanup | Despawn entities, save data |

### Plugin Implementation Pattern

```rust
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MyResource>()
            .add_systems(Startup, setup)
            .add_systems(Update, update_system);
    }
}
```

### File Location

`plugin_architecture_diagram.mermaid`

### How to Read

- **Blue nodes** = Core Bevy systems
- **Pink nodes** = Plugin bundles
- **Purple box** = Code pattern example
- **Orange nodes** = System schedules
- **Solid arrows** = "contains" relationships
- **Dotted arrows** = "depends on" relationships

---

## 🎯 How to Use These Diagrams

### For AI Agents

1. **Start with Main Architecture** - Understand overall structure
2. **Study State Flow** - Learn application lifecycle
3. **Master ECS Interaction** - Understand data flow
4. **Review Plugin Architecture** - Learn module organization
5. **Reference File Structure** - Know where to create files

### For Developers

1. **Planning Phase** - Use State Flow and Architecture diagrams
2. **Implementation Phase** - Reference File Structure and Plugin Architecture
3. **Debugging Phase** - Check Lifecycle Sequence and ECS Interaction
4. **Documentation Phase** - Include these diagrams in project docs

### For Team Onboarding

1. Show **Main Architecture** first - big picture
2. Explain **State Flow** - user experience flow
3. Demonstrate **ECS Interaction** - coding patterns
4. Walk through **File Structure** - where to find code
5. Describe **Plugin Architecture** - how everything connects

---

## 📝 Diagram Usage Examples

### Example 1: Adding a New Feature

**Task**: Add a new "Inventory" system to the game

1. **Check Main Architecture**: Decide if it's Framework or Game layer → **Game layer**
2. **Review File Structure**: Create files in `game/components/`, `game/systems/`, `game/resources/`
3. **Study ECS Interaction**: Follow component → system → resource pattern
4. **Check Plugin Architecture**: Add to GamePlugin bundle
5. **Review State Flow**: Determine which states should have inventory access

### Example 2: Debugging State Transitions

**Problem**: Game doesn't transition from Loading to MainMenu

1. **Check State Flow Diagram**: Verify transition is valid (Loading → MainMenu ✓)
2. **Review Lifecycle Sequence**: Check OnExit(Loading) and OnEnter(MainMenu) systems
3. **Examine Plugin Architecture**: Ensure LoadingPlugin properly depends on StatesPlugin
4. **Verify ECS Interaction**: Check if loading completion resource is properly set

### Example 3: Porting Framework to New Project

**Task**: Use this framework for a new racing game

1. **Use File Structure Diagram**: Copy entire `framework/` directory
2. **Reference Plugin Architecture**: Keep FrameworkPlugin intact
3. **Check State Flow**: Reuse Splash → Loading → MainMenu flow
4. **Replace Game Layer**: Delete `game/` and create new racing-specific logic
5. **Verify Main Architecture**: Ensure clean separation maintained

---

## 🔗 Diagram Relationships

```
Main Architecture ←→ File Structure       (Structure mirrors organization)
State Flow ←→ Lifecycle Sequence          (States drive lifecycle)
ECS Interaction ←→ Plugin Architecture    (ECS implemented via plugins)
Plugin Architecture ←→ File Structure     (Plugins map to directories)
```

---

## 📚 Additional Resources

### Viewing Mermaid Diagrams

- **GitHub**: Renders `.mermaid` files automatically
- **VS Code**: Install "Mermaid Preview" extension
- **Online**: Use [mermaid.live](https://mermaid.live) editor
- **Documentation**: Embed in Markdown files

### Updating Diagrams

When the architecture changes:

1. Update the relevant `.mermaid` file
2. Regenerate this documentation
3. Update code to match diagrams
4. Notify team of changes

### Diagram Maintenance Checklist

- [ ] Diagrams match current codebase structure
- [ ] New features are reflected in diagrams
- [ ] Color coding is consistent across diagrams
- [ ] All relationships are accurate
- [ ] Documentation explains all symbols

---

## 🎨 Color Coding Legend

| Color | Hex | Meaning | Used In |
|-------|-----|---------|---------|
| Navy Blue | #1976D2 | Core Bevy / Root | Main, Plugin, File Structure |
| Blue | #4A90E2 | Framework Layer | All diagrams |
| Green | #50C878 | Game Layer | All diagrams |
| Orange | #FF9800 | Configuration / Resources | Main, ECS, File Structure |
| Purple | #9C27B0 | Utilities / Code | Main, Plugin |
| Red | #E91E63 | Plugins / Documentation | Plugin, File Structure |
| Yellow | #FFC107 | Paused State | State Flow |
| Cyan | #00BCD4 | Settings State | State Flow |
| Pink | #E91E63 | Entities | ECS Interaction |

---

## 🚀 Quick Start Guide

### Step 1: Understand the Architecture

```
Read: Main Architecture Diagram → File Structure Diagram
Goal: Understand Framework/Game separation
```

### Step 2: Learn the Flow

```
Read: State Flow Diagram → Lifecycle Sequence Diagram
Goal: Understand application execution
```

### Step 3: Master ECS

```
Read: ECS Interaction Diagram → Plugin Architecture Diagram
Goal: Understand how to write Bevy code
```

### Step 4: Start Coding

```
Reference: All diagrams while implementing
Goal: Build features following the architecture
```

---

## 📖 Conclusion

These diagrams provide a complete visual reference for the Bevy game engine architecture. They are designed to:

✅ **Guide development** - Clear patterns to follow  
✅ **Facilitate learning** - Visual understanding of concepts  
✅ **Enable collaboration** - Shared architectural language  
✅ **Support maintenance** - Easy to navigate codebase  
✅ **Promote consistency** - Standardized structure across projects  

Keep these diagrams updated as the architecture evolves, and reference them frequently during development to maintain architectural integrity.

---

**Document Version**: 1.0  
**Last Updated**: 2026  
**Diagram Format**: Mermaid  
**Total Diagrams**: 6
