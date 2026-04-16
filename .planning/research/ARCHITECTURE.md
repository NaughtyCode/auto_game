# Architecture Patterns

**Domain:** 3D turn-based cricket fighting game
**Researched:** 2026-04-17
**Confidence:** MEDIUM (Based on analysis of existing Godot turn-based combat template, project requirements, and general game architecture knowledge)

## Recommended Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      Presentation Layer                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │
│  │   UI    │  │ Camera  │  │  VFX    │  │  Audio  │        │
│  │  HUD    │  │ Control │  │ System  │  │ System  │        │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘        │
│       │            │            │            │              │
├───────┴────────────┴────────────┴────────────┴──────────────┤
│                      Gameplay Layer                           │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐    │
│  │                 Battle Manager                       │    │
│  │  • Turn order & state machine                       │    │
│  │  • Action point (AP) system                         │    │
│  │  • Win/lose condition checks                        │    │
│  │  • Battle flow coordination                         │    │
│  └──────┬──────────────────────────────────────────────┘    │
│         │                                                   │
│  ┌──────┴──────────────────────────────────────────────┐    │
│  │                 Environment Manager                   │    │
│  │  • Interactive arena elements                        │    │
│  │  • Terrain effects on attributes                     │    │
│  └──────┬──────────────────────────────────────────────┘    │
│         │                                                   │
│  ┌──────┴──────────────────────────────────────────────┐    │
│  │                    AI System                         │    │
│  │  • Opponent decision-making                         │    │
│  │  • Skill selection & targeting                      │    │
│  └──────┬──────────────────────────────────────────────┘    │
│         │                                                   │
├─────────┼───────────────────────────────────────────────────┤
│         │                 Data Layer                         │
├─────────┼───────────────────────────────────────────────────┤
│  ┌──────▼──────┐  ┌──────────┐  ┌──────────┐               │
│  │   Battler   │  │  Skill   │  │  Arena   │               │
│  │ (Cricket)   │  │  System  │  │  Data    │               │
│  └─────────────┘  └──────────┘  └──────────┘               │
│         │                                                   │
│  ┌──────▼──────┐  ┌──────────┐  ┌──────────┐               │
│  │   Animation │  │Attribute │  │  Action  │               │
│  │   System    │  │  System  │  │  System  │               │
│  └─────────────┘  └──────────┘  └──────────┘               │
└─────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| **Battle Manager** | Central coordinator for battle flow. Manages turn order, action points, win/lose conditions, delegates to AI/player turns, processes actions. | Godot Node3D with state machine. References arrays of Battlers, tracks current turn, AP, queued actions. Connects UI signals. |
| **Battler (Cricket)** | Represents a cricket combatant. Holds attributes (power, speed, defense, HP), current AP, actions, animation state. Responds to battle commands (attack, defend, skill). | Godot CharacterBody3D with exported stats resource. Contains methods for take_damage, use_skill, battle_idle. Signals for animation events. |
| **Action Point System** | Tracks AP per battler per turn. Defines AP costs for actions, refresh logic, UI updates. | Integrated into Battle Manager and Battler. Each battler has `current_ap` and `max_ap`. BattleManager refreshes AP at start of turn or per round. |
| **Action/Skill System** | Defines available actions (normal attack, heavy attack, defend, special skill) with AP costs, targeting rules, effects. | Resource-based (.tres) Skill classes with target type, AP cost, effect function. BattleManager references skills. |
| **AI System** | Selects actions for opponent crickets based on intelligence, AI type (aggressive/defensive), current state. | Separate AIManager node with choose_action method. Evaluates targets, skill effectiveness, uses randomness weighted by intelligence. |
| **Environment Manager** | Manages interactive arena elements (grass, sand, rocks) that affect gameplay. Applies terrain modifiers to battlers. | Node3D with area detection. Listens to battler position changes, applies stat modifiers. May trigger events. |
| **UI/HUD** | Displays battle info: health bars, AP counters, action buttons, turn indicator, character attributes. | Godot Control nodes with BattleHUD script. Connects signals to BattleManager for action_selected, updates visuals from battler data. |
| **Camera Controller** | 3D camera orbiting battlefield, zoom, rotation. Allows player to view action from any angle. | Godot Camera3D with spring arm, input handling for mouse drag/scroll. Optional cinematic shots during special moves. |
| **Animation System** | Handles cricket animations (idle, attack, defend, damage, death). Blends animations based on state. | Godot AnimationTree with state machine. Battler calls animation methods that trigger AnimationPlayer blends. |
| **Data/Resource System** | Stores cricket stats, skills, arena data as reusable resources. Enables easy balancing via inspector. | .tres resource files (BattlerStats, Skill, ArenaData). Loaded by scenes via @export references. |

## Recommended Project Structure

```
src/ (or project root)
├── assets/                  # 3D models, textures, sounds
│   ├── models/crickets/     # Cricket FBX/glTF with animations
│   ├── textures/
│   └── audio/
├── scenes/                  # Godot scene files
│   ├── battles/             # Battle scenes (arenas)
│   ├── characters/          # Cricket prefabs
│   └── ui/                  # UI scenes
├── scripts/                 # GDScript files
│   ├── battle/              # Battle Manager, AIManager, EnvironmentManager
│   ├── characters/          # Battler, attributes, skills
│   ├── ui/                  # HUD, buttons, menus
│   ├── camera/              # Camera controller
│   └── utils/               # Helper functions, formulas
├── resources/               # Resource files (.tres)
│   ├── crickets/            # BattlerStats for each cricket type
│   ├── skills/              # Skill resources
│   └── arenas/              # Arena data
└── tests/                   # Unit/integration tests
```

### Structure Rationale

- **Separate assets from code**: Godot's import system works better when assets are in dedicated folders.
- **Scene-per-prefab**: Each cricket type should be a scene with Battler script attached, making instancing easy.
- **Resource-driven design**: Stats and skills as resources allows balancing without code changes.
- **Script organization by domain**: Battle, character, UI scripts separated for clarity.

## Architectural Patterns

### Pattern 1: Centralized Battle Manager with Event Delegation

**What:** Battle Manager acts as central authority, coordinating all subsystems. Components communicate via signals (events) rather than direct calls.

**When to use:** Turn-based games with complex state transitions. Ensures single source of truth for battle state.

**Trade-offs:**
- **Pros:** Clean separation, easier debugging, predictable state flow.
- **Cons:** Battle Manager can become a "god object" if not carefully bounded.

**Example:**
```gdscript
# BattleManager.gd
signal action_selected(action, skill)
signal turn_ended

func _on_action_selected(action: String, skill: Skill) -> void:
    if validate_ap(current_battler, action):
        current_action = action
        current_skill = skill
        emit_signal("action_selected", action, skill)

# UI sends action via signal
hud.action_selected.connect(battle_manager._on_action_selected)
```

### Pattern 2: Resource-Based Configuration

**What:** Game data (stats, skills, arena properties) stored as Godot Resource (.tres) files, editable in inspector.

**When to use:** When you need to balance game without recompiling code, or when multiple entities share similar configurations.

**Trade-offs:**
- **Pros:** Non-programmers can adjust values, version control friendly, reusable across scenes.
- **Cons:** Requires resource loading pipeline, may be overkill for trivial data.

**Example:**
```gdscript
# BattlerStats.gd (extends Resource)
@export var power: int = 10
@export var speed: int = 10
@export var defense: int = 10
@export var max_health: int = 100

# Battler.gd
@export var stats: BattlerStats
func _ready():
    max_health = stats.max_health
    current_health = max_health
```

### Pattern 3: State Machine for Battle Flow

**What:** Battle Manager uses explicit states (PlayerTurn, EnemyTurn, ActionExecution, BattleEnd) to manage flow.

**When to use:** Any turn-based game with distinct phases. Prevents illegal actions (e.g., attacking during enemy turn).

**Trade-offs:**
- **Pros:** Clear state transitions, easy to add new states (e.g., cutscene), debug visualization.
- **Cons:** Additional complexity for simple battles.

**Example:**
```gdscript
enum BattleState {PLAYER_TURN, ENEMY_TURN, ACTION_EXECUTION, BATTLE_END}
var current_state: BattleState = BattleState.PLAYER_TURN

func process_player_input(action):
    if current_state != BattleState.PLAYER_TURN:
        return
    current_state = BattleState.ACTION_EXECUTION
    execute_action(action)
```

## Data Flow

### Action Execution Flow

```
Player clicks UI button
    ↓
UI emits action_selected(action, skill) signal
    ↓
BattleManager receives signal → validates AP, enters target selection
    ↓
Player selects target (click or keyboard)
    ↓
BattleManager executes action → calls battler.use_skill(skill, target)
    ↓
Battler plays animation → emits animation_damage signal
    ↓
BattleManager processes damage calculation → applies to target
    ↓
BattleManager checks win/lose → updates UI → moves to next turn
```

### Attribute Modification Flow

```
Environment detects battler entered area
    ↓
EnvironmentManager applies terrain modifier (e.g., speed -2)
    ↓
Modifier added to battler's active_modifiers list
    ↓
Battler's get_effective_speed() calculates base + modifiers
    ↓
UI reads effective speed for display
    ↓
BattleManager uses effective speed for turn order calculation
```

### AI Decision Flow

```
BattleManager calls AIManager.choose_action(enemy, targets)
    ↓
AI evaluates available skills based on:
    - Current HP/AP
    - Target vulnerabilities
    - AI type (aggressive/defensive)
    - Intelligence (randomness weight)
    ↓
AI selects action and target
    ↓
AI returns action to BattleManager via callback
    ↓
BattleManager executes as if player action
```

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| **1-2 cricket types, 1 arena** | Monolithic BattleManager fine; keep all logic in few scripts. |
| **5+ cricket types, multiple skills** | Move skill logic to separate Skill classes; use resource system for balance. |
| **Multiple arena types with complex interactions** | Extract EnvironmentManager to handle terrain effects; use area detection. |
| **Adding multiplayer (hotseat)** | Add PlayerController component that routes input to BattleManager; keep battle logic unchanged. |
| **Adding RPG progression** | Add Experience/Level component to Battler; modify stats via progression curves. |

### Scaling Priorities

1. **First bottleneck:** BattleManager complexity — split into sub-managers (TurnManager, ActionManager) when file exceeds 500 lines.
2. **Second bottleneck:** Animation blending — use AnimationTree with state machine early to avoid hardcoded animation calls.
3. **Third bottleneck:** AI decision time — implement decision caching and limit skill evaluation depth.

## Anti-Patterns

### Anti-Pattern 1: Direct Component Coupling

**What people do:** Battler directly calls UI methods to update health bars, or UI directly modifies battler state.

**Why it's wrong:** Creates tight coupling, makes testing difficult, breaks when UI changes.

**Do this instead:** Use signals. Battler emits health_changed signal; UI listens and updates. BattleManager mediates actions.

### Anti-Pattern 2: Hardcoded Attribute Formulas

**What people do:** Writing damage calculation directly in BattleManager as `damage = attacker.power - target.defense`.

**Why it's wrong:** Difficult to balance, cannot adjust formula per skill, no flexibility for critical hits/elemental strengths.

**Do this instead:** Create Formulas static class with methods like `physical_damage(attacker, target, skill)`. Skill resources can reference formula type.

### Anti-Pattern 3: Mixing Animation and Game Logic

**What people do:** Putting damage application inside animation timeline or AnimationPlayer callback.

**Why it's wrong:** Makes game logic dependent on animation timing, breaks when animations change, hard to test.

**Do this instead:** Use signals. Animation emits `animation_damage_frame` signal; BattleManager applies damage then. Keep game logic frame-rate independent.

### Anti-Pattern 4: Global State Abuse

**What people do:** Using global variables for battle state, like `var current_turn` in autoload.

**Why it's wrong:** Difficult to debug, prevents multiple simultaneous battles (e.g., tutorial), not thread-safe.

**Do this instead:** Keep battle state in BattleManager instance. Pass references where needed (dependency injection).

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| **Godot Animation System** | Signal-based callbacks | Connect AnimationPlayer "animation_finished" to resume game logic. Use AnimationTree for complex states. |
| **Godot Physics Engine** | Area detection for environment | Use Area3D nodes with collision shapes to detect battler position changes. |
| **Godot UI System** | Signal connections | UI buttons emit signals; BattleManager connects. Use Theme resources for consistent styling. |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| **BattleManager ↔ Battler** | Method calls + signals | BattleManager calls `battler.use_skill()`; battler emits `skill_used` signal. |
| **BattleManager ↔ AI System** | Callback functions | BattleManager calls `ai.choose_action(params)`; AI calls back with selected action. |
| **BattleManager ↔ EnvironmentManager** | Signals | EnvironmentManager emits `terrain_entered(battler, terrain_type)`; BattleManager listens. |
| **Battler ↔ UI/HUD** | Signals | Battler emits `health_changed`, `ap_changed`; UI subscribes. |

## Sources

- Analysis of `3D-TurnBasedCombat` Godot template (GitHub: Cute-Fame-Studio/3D-TurnBasedCombat)
- Godot Engine documentation patterns (despite 403 access limitations, inferred from template)
- General game architecture knowledge for turn-based combat systems
- Project requirements from `.planning/PROJECT.md`

---
*Architecture research for: 3D turn-based cricket fighting game*
*Researched: 2026-04-17*