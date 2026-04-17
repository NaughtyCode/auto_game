# Phase 2: Basic Insect Implementation - Pattern Map

**Mapped:** 2026-04-17
**Files analyzed:** 12 (inferred from decisions)
**Analogs found:** 0 / 12 (no existing codebase)

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `scenes/insects/insect_base.tscn` | scene | event-driven | No analog | none |
| `scripts/insects/insect.gd` | component | event-driven | No analog | none |
| `scripts/insects/insect_animation.gd` | component | streaming | No analog | none |
| `scripts/graphics/procedural_material.gd` | service | transform | No analog | none |
| `scripts/camera/auto_distance.gd` | component | request-response | No analog | none |
| `scripts/managers/insect_manager.gd` | controller | CRUD | No analog | none |
| `scenes/ui/debug_panel.tscn` | scene | request-response | No analog | none |
| `scripts/ui/debug_panel.gd` | component | request-response | No analog | none |
| `scripts/ui/insect_count.gd` | component | event-driven | No analog | none |
| `scripts/ui/camera_distance_display.gd` | component | event-driven | No analog | none |
| `resources/insect_stats.tres` | model | file-I/O | No analog | none |
| `shaders/insect_procedural.gdshader` | shader | transform | No analog | none |

## Pattern Assignments

Since no existing codebase exists, patterns are derived from research documentation (ARCHITECTURE.md, NATURAL_ENVIRONMENT.md, OBSERVATION_GAMEPLAY.md). Use these as reference for implementation patterns.

### `scripts/insects/insect.gd` (component, event-driven)

**Analog:** No analog — use research pattern for battler with signals.

**Resource pattern** (ARCHITECTURE.md lines 145-156):
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

**Signal pattern** (ARCHITECTURE.md lines 118-131):
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

Apply similar pattern: define signals for insect events (spawned, died, animation_state_changed) and connect them in manager.

### `scripts/insects/insect_animation.gd` (component, streaming)

**Analog:** No analog — use research pattern for animation state machine.

**State machine pattern** (ARCHITECTURE.md lines 169-178):
```gdscript
enum BattleState {PLAYER_TURN, ENEMY_TURN, ACTION_EXECUTION, BATTLE_END}
var current_state: BattleState = BattleState.PLAYER_TURN

func process_player_input(action):
    if current_state != BattleState.PLAYER_TURN:
        return
    current_state = BattleState.ACTION_EXECUTION
    execute_action(action)
```

Adapt for insect animation states:
```gdscript
enum InsectAnimationState {IDLE, WALK, TURN, ATTACK}
var current_animation_state: InsectAnimationState = InsectAnimationState.IDLE
```

**Animation blending:** Use Godot's AnimationTree with state machine.

### `scripts/graphics/procedural_material.gd` (service, transform)

**Analog:** No analog — use research pattern for procedural generation.

**Noise pattern** (NATURAL_ENVIRONMENT.md lines 206-225):
```gdscript
# 来源：Godot社区模式 [ASSUMED]
extends GridMap

func generate_terrain(width: int, depth: int, height_scale: float):
    var noise = FastNoiseLite.new()
    noise.seed = randi()
    noise.frequency = 0.05
    
    for x in range(width):
        for z in range(depth):
            # generate height from noise
```

Adapt for procedural insect patterns: Use FastNoiseLite to generate texture patterns, apply to material.

### `scripts/camera/auto_distance.gd` (component, request-response)

**Analog:** No analog — use research pattern for camera follow.

**Camera follow pattern** (OBSERVATION_GAMEPLAY.md lines 170-190):
```gdscript
extends Camera3D

var follow_target: Node3D = null
var spring_arm: SpringArm3D

func _ready():
    spring_arm = $SpringArm3D

func _process(delta):
    if follow_target:
        # smooth interpolation to target position
        global_position = global_position.lerp(follow_target.global_position, delta * 5.0)
```

Adapt for auto-distance: Calculate optimal distance based on insect size, interpolate camera distance smoothly.

### `scripts/managers/insect_manager.gd` (controller, CRUD)

**Analog:** No analog — use research pattern for manager with CRUD operations.

**Manager pattern** (ARCHITECTURE.md lines 118-131):
```gdscript
signal action_selected(action, skill)
signal turn_ended

func _on_action_selected(action: String, skill: Skill) -> void:
    if validate_ap(current_battler, action):
        current_action = action
        current_skill = skill
        emit_signal("action_selected", action, skill)
```

Implement methods: `spawn_insect(species, position)`, `remove_insect(insect)`, `get_insect_count()`.

### `scripts/ui/debug_panel.gd` (component, request-response)

**Analog:** No analog — use research pattern for UI signals.

**UI signal pattern** (ARCHITECTURE.md lines 118-131):
```gdscript
# UI sends action via signal
hud.action_selected.connect(battle_manager._on_action_selected)
```

Implement button signals for "Spawn Cricket", "Spawn Beetle", "Spawn Spider" that emit signals to insect manager.

**Button creation pattern** (from UI-SPEC): Use Godot Control nodes (Button, VBoxContainer) with accent color #4CAF50 for spawn buttons.

### `resources/insect_stats.tres` (model, file-I/O)

**Analog:** No analog — use research pattern for resource-based configuration.

**Resource pattern** (ARCHITECTURE.md lines 145-156):
```gdscript
# BattlerStats.gd (extends Resource)
@export var power: int = 10
@export var speed: int = 10
@export var defense: int = 10
@export var max_health: int = 100
```

Create InsectStats resource with @export variables for size, color, pattern parameters, movement speed, etc.

---

## Shared Patterns

### Signal-Based Communication
**Source:** ARCHITECTURE.md lines 118-131
**Apply to:** All component scripts (insect, manager, UI)
```gdscript
# Define signals at top of script
signal insect_spawned(insect)
signal insect_removed(insect)
signal animation_state_changed(new_state)

# Emit signals when events occur
func spawn_insect():
    var new_insect = preload("res://scenes/insects/insect_base.tscn").instantiate()
    add_child(new_insect)
    emit_signal("insect_spawned", new_insect)

# Connect signals in other scripts
insect_manager.insect_spawned.connect(_on_insect_spawned)
```

### Resource-Driven Configuration
**Source:** ARCHITECTURE.md lines 145-156
**Apply to:** Insect stats, species parameters
```gdscript
# InsectStats.gd (extends Resource)
@export var size: float = 0.1  # meters
@export var walk_speed: float = 1.0
@export var color: Color = Color.WHITE
@export var pattern_complexity: int = 5

# Insect.gd
@export var stats: InsectStats
func _ready():
    scale = Vector3.ONE * stats.size
```

### State Machine for Animation
**Source:** ARCHITECTURE.md lines 169-178
**Apply to:** Insect animation controller
```gdscript
enum InsectAnimationState {IDLE, WALK, TURN, ATTACK}
var current_state: InsectAnimationState = InsectAnimationState.IDLE

func set_animation_state(new_state: InsectAnimationState):
    if current_state == new_state:
        return
    current_state = new_state
    # trigger animation transition
```

### Smooth Interpolation
**Source:** OBSERVATION_GAMEPLAY.md lines 170-190
**Apply to:** Camera auto-distance, animation blending
```gdscript
# Use lerp for smooth transitions
var target_distance: float = 5.0
var current_distance: float = 10.0

func _process(delta):
    current_distance = lerp(current_distance, target_distance, delta * 2.0)
```

---

## No Analog Found

All files have no existing analog because this is a fresh Godot project. Use research patterns above as implementation guidance.

## Metadata

**Analog search scope:** Entire project root (no Godot files found)
**Files scanned:** 0 (no existing .gd, .tscn, .tres files)
**Pattern extraction date:** 2026-04-17
**Research sources referenced:**
- `.planning/research/ARCHITECTURE.md` — Godot architecture patterns, signal system, resource patterns
- `.planning/research/NATURAL_ENVIRONMENT.md` — Procedural generation, noise patterns
- `.planning/research/OBSERVATION_GAMEPLAY.md` — Camera follow patterns
- `.planning/research/STACK.md` — Technology stack (Godot 4.6.2, GDScript)

**Note to planner:** Since there are no existing code patterns, each plan should include concrete implementation based on research patterns above. Follow Godot 4.6.2 and GDScript conventions as per STACK.md.