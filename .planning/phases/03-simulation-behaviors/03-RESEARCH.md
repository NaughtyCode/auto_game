# Phase 3: Simulation Behaviors - Research

**Researched:** 2026-04-17
**Domain:** Godot 4 insect behavior simulation (movement, feeding, interaction, environmental response)
**Confidence:** MEDIUM (Based on existing project research, Godot documentation verification for core classes, and general game AI patterns; some implementation details assumed due to limited access to specific Godot steering behavior tutorials)

## Summary

This phase implements observational insect behaviors in a 3D Godot simulation. Insects must exhibit species‑appropriate movement patterns, seek and consume food, interact with each other based on species relationships, and respond to environmental stimuli (light, terrain, temperature). All behaviors are driven by simple AI systems using steering, scent‑gradient detection, proximity/raycast sensing, and flocking algorithms. The simulation requires a global environment manager for day/night cycles, temperature, and food respawning, plus per‑insect behavior scripts that query this global state.

**Primary recommendation:** Implement a component‑based AI architecture where each insect runs a behavior tree (or simple state machine) that samples steering, scent, neighbor detection, and environment state each frame, producing movement and animation outputs. Use Godot's built‑in NavigationAgent3D for pathfinding where needed, but for random wandering implement a lightweight steering system directly in GDScript.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
**Movement & Navigation**
- D-01: Movement style: Random wandering with steering (Godot steering behaviors)
- D-02: Species‑specific movement: Each insect species (cricket, beetle, spider) has distinct locomotion (hops, crawls, scuttles)
- D-03: Obstacle handling: Collision detection + random turn (no raycast steering)
- D-04: Idle behavior: Insects periodically pause moving (alternate between moving and idle states)
- D-05: Speed variation: Individual insects of the same species have small random speed differences
- D-06: (Claude's discretion) Steering implementation details (weights, turn rates, wander parameters)

**Food & Consumption**
- D-07: Food representation: Visible 3D objects (fruit, leaf, etc.) placed in the environment
- D-08: Food detection: Smell gradient – food emits a scent gradient that insects follow
- D-09: Consumption method: Stationary eating animation (insect stops, plays eat animation, food disappears after)
- D-10: Food placement: Randomly scattered across the terrain
- D-11: Food preferences: Different insect species prefer different food types (e.g., beetle→fruit, cricket→leaf)
- D-12: Food respawn: Food regrows after being consumed (continuous supply)
- D-13: (Claude's discretion) Smell‑gradient implementation (range, decay, update frequency)

**Insect Interactions**
- D-14: Interaction type: Approach/avoid based on species (e.g., cricket avoids spider)
- D-15: Detection method: Both proximity radius and line‑of‑sight (raycast)
- D-16: Interaction matrix: Defined per species pair (cricket–spider, beetle–cricket, etc.)
- D-17: Influence strength: Strong influence – immediate turn toward/away
- D-18: Social grouping: Simple flocking – insects of the same species tend to stay close together
- D-19: (Claude's discretion) Flocking algorithm parameters (cohesion, separation, alignment weights)

**Environmental Responses**
- D-20: Active stimuli: Light changes, terrain type, obstacles, temperature
- D-21: Light response: Change activity level (move faster in day, slower at night)
- D-22: Terrain response: Speed varies per terrain type (e.g., slower on soil, faster on grass)
- D-23: Temperature response: Seek warmer/cooler areas (insects move toward preferred temperature zones)
- D-24: Temperature variation: Global temperature (single value for entire environment)
- D-25: (Claude's discretion) Day/night cycle implementation and temperature simulation details

### Claude's Discretion
Areas where the user deferred to Claude:
- Steering implementation details (weights, turn rates, wander parameters)
- Smell‑gradient implementation (range, decay, update frequency)
- Flocking algorithm parameters (cohesion, separation, alignment weights)
- Day/night cycle implementation and temperature simulation details
- Exact species‑pair interaction matrix values
- Specific food‑type assignments per species
- Exact speed multipliers per terrain type

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SIM-01 | User can observe insects moving naturally in the environment | Steering behaviors, species‑specific movement patterns, idle/pause cycles, speed variation |
| SIM-02 | User can observe insects seeking and consuming food sources | Scent‑gradient detection, food preferences, consumption animation, food respawn |
| SIM-03 | User can observe basic interactions between different insects | Proximity + raycast detection, species‑pair interaction matrix, approach/avoid behaviors |
| SIM-04 | User can see insects responding to environmental stimuli | Light‑based activity change, terrain‑dependent speed, temperature‑seeking behavior |
| INSECT-02 | User can see different movement patterns for each insect type | Species‑specific locomotion styles (hops, crawls, scuttles) implemented in steering system |
| INSECT-03 | User can observe different feeding behaviors per insect species | Food‑type preferences, different eating animations, species‑specific detection ranges |
</phase_requirements>

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Insect movement (wandering, steering, collision avoidance) | AI system (insect behavior script) | Physics engine (collision detection) | Movement logic belongs to insect AI; physics engine provides collision feedback |
| Food detection via scent gradient | AI system (insect senses) | Simulation manager (scent emission) | Insects decide how to interpret scent; global manager may compute gradient field |
| Food consumption | AI system (insect behavior) | Food node (visual/state) | Insect triggers consumption animation; food node handles disappearance/respawn |
| Insect interactions (approach/avoid) | AI system (insect decision) | Detection system (proximity, raycast) | AI decides response based on detected neighbor; detection is separate sensor |
| Flocking (social grouping) | AI system (group behavior) | — | Flocking is a collective behavior computed per insect using neighbor positions |
| Environmental responses (light, terrain, temperature) | AI system (insect response) | Environment manager (global state) | Insects react to environment; environment manager provides global light/temperature values |
| Day/night cycle | Environment manager (global simulation) | Rendering (lighting) | Global simulation updates time of day; rendering adjusts directional light and ambient |
| Temperature simulation | Environment manager | — | Global temperature value computed from time of day and possibly local heat sources |
| Food respawn | Food manager | — | Central manager tracks consumed food and respawns after delay |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Godot Engine | 4.6.2-stable | Game engine for 3D simulation | Latest stable version with modern 3D rendering, physics, and GDScript 2.0 [VERIFIED: godotengine.org] |
| GDScript | 4.6.2 | Primary scripting language | Native to Godot, tight editor integration, fast iteration [VERIFIED: godotengine.org] |
| NavigationAgent3D | Built‑in | Pathfinding and steering | Provides velocity‑based steering, obstacle avoidance, and navigation target following [CITED: docs.godotengine.org/en/stable/classes/class_navigationagent3d.html] |
| RayCast3D | Built‑in | Line‑of‑sight detection | Essential for insect interaction detection (D‑15) [CITED: docs.godotengine.org/en/stable/classes/class_raycast3d.html] |
| Area3D | Built‑in | Proximity detection | Sphere‑based detection for nearby insects and food [CITED: docs.godotengine.org/en/stable/classes/class_area3d.html] |
| AnimationPlayer | Built‑in | Insect animations | Play eating, idle, movement animations; supports blending [CITED: docs.godotengine.org/en/stable/classes/class_animationplayer.html] |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| WorldEnvironment | Built‑in | Global lighting and fog | Day/night cycle via DirectionalLight rotation and ambient color changes |
| GPUParticles3D | Built‑in | Visual effects (dust, etc.) | Optional: add particle trails behind moving insects |
| MultiMeshInstance | Built‑in | Efficient food instance rendering | If food objects are numerous (>50) and share the same mesh |
| Signal system | Built‑in | Decoupled communication | Notify environment manager when food is consumed, emit insect detection events |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| NavigationAgent3D | Custom steering implementation | Custom gives full control but requires implementing obstacle avoidance and path smoothing |
| RayCast3D for all detection | Only proximity (Area3D) | RayCast adds line‑of‑sight realism but costs performance; proximity is cheaper |
| Global scent grid | Per‑food distance check | Grid allows diffusion and complex gradients but needs 2D/3D texture updates; distance check is simpler for small food counts |

**Installation:**
```bash
# Godot 4.6.2 from https://godotengine.org/download
# No additional npm packages required for this phase
```

**Version verification:** Godot 4.6.2 is the latest stable version as of 2026-04-17 (verified via godotengine.org). GDScript version matches engine version.

## Architecture Patterns

### System Architecture Diagram

```
Player Observation
       ↓
Camera (free‑fly)───────────┐
       ↓                    ↓
Environment Manager     Insect Manager
├── Day/night cycle    ├── Insect instances (scene)
├── Global temperature │   ├── Behavior script
├── Food respawn timer │   │   ├── Steering (wander)
└── Scent gradient     │   │   ├── Scent detection
       ↓               │   │   ├── Interaction detection
       └───────────────┼───│───┼── Flocking
                       │   │   └── Environment response
                       │   ├── AnimationPlayer
                       │   └── CollisionShape3D
                       └── Food instances
                           ├── Area3D (scent emitter)
                           └── MeshInstance3D
```

Data flow:
1. Environment manager updates day/night, temperature, scent gradients each frame.
2. Each insect's behavior script reads environment state, scans for food via scent, detects nearby insects via Area3D + RayCast3D.
3. Steering system computes wander direction, adjusted by obstacle collisions, food attraction, neighbor influence, and flocking forces.
4. Final velocity applied via `move_and_slide()`; animation state updated.
5. Food consumption triggers signal to environment manager, which schedules respawn.

### Recommended Project Structure
```
src/
├── insects/
│   ├── scenes/                 # Insect scene files (.tscn)
│   ├── scripts/
│   │   ├── insect_behavior.gd  # Main AI controller
│   │   ├── steering.gd         # Wander, seek, avoid logic
│   │   ├── detection.gd        # Scent, proximity, raycast
│   │   └── flocking.gd         # Cohesion, separation, alignment
│   └── animations/             # Animation resources
├── environment/
│   ├── managers/
│   │   ├── environment_manager.gd  # Day/night, temperature, food respawn
│   │   └── scent_grid.gd           # Optional global scent diffusion
│   └── food/
│       ├── food_scene.tscn
│       └── food.gd
└── utils/
    ├── constants.gd            # Species stats, interaction matrix
    └── helpers.gd              # Math utilities
```

### Pattern 1: Component‑Based Insect AI
**What:** Each insect behavior is split into independent components (steering, detection, flocking) that are combined in a central behavior script.
**When to use:** When insects need multiple overlapping behaviors that can be toggled or weighted.
**Example:**
```gdscript
# insect_behavior.gd
extends CharacterBody3D

var steering: SteeringComponent
var detection: DetectionComponent
var flocking: FlockingComponent

func _physics_process(delta):
    var food_target = detection.get_nearest_food()
    var neighbors = detection.get_nearby_insects()
    
    var steer_force = steering.wander()
    if food_target:
        steer_force += steering.seek(food_target.global_position)
    if neighbors.size() > 0:
        steer_force += flocking.calculate(neighbors)
    
    velocity = steer_force * move_speed
    move_and_slide()
```

### Pattern 2: Scent Gradient via Distance Falloff
**What:** Each food emits a scent value that decays with distance; insects sample the strongest scent within their detection radius.
**When to use:** When food count is small (<30) and computational cost must be low.
**Example:**
```gdscript
# food.gd
extends Node3D

@export var scent_range: float = 10.0
@export var max_scent: float = 1.0

func get_scent_at_position(pos: Vector3) -> float:
    var distance = global_position.distance_to(pos)
    if distance > scent_range:
        return 0.0
    return max_scent * (1.0 - distance / scent_range)
```

### Anti-Patterns to Avoid
- **Hard‑coded species constants in scripts:** Makes balancing difficult. Store species stats (speed, food preference, interaction weights) in a resource file or constants.gd.
- **Every insect scanning every food each frame:** O(n²) cost. Use spatial partitioning (Area3D groups) or a global scent grid.
- **Mixing AI logic with animation logic:** Leads to tangled code. Keep animation updates in a separate method called after velocity is applied.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Pathfinding across terrain | Custom A* on grid | NavigationRegion3D + NavigationAgent3D | Godot's navigation system handles dynamic obstacles, mesh baking, and smooth path interpolation. |
| Physics‑based collision avoidance | Manual raycast steering | `move_and_slide()` with slope detection and collision feedback | Built‑in method handles sliding, stairs, slopes, and provides collision normal for random turn. |
| Complex animation blending | Manual animation interpolation | AnimationTree with state machine | AnimationTree provides blending, transitions, and parameters; manual blending is error‑prone. |
| Scent diffusion simulation | Custom fluid dynamics solver | Simple distance falloff or 2D texture blur | Full diffusion is overkill for observation; falloff is convincing and cheap. |
| Flocking spatial queries | Brute‑force neighbor search | Area3D with `get_overlapping_bodies()` | Area3D uses Godot's spatial indexing; brute force scales poorly with many insects. |

**Key insight:** Godot's built‑in nodes (NavigationAgent3D, Area3D, AnimationTree) are optimized and handle edge cases (e.g., multi‑threading, memory management). Re‑implementing them is time‑consuming and likely buggy.

## Common Pitfalls

### Pitfall 1: Steering Over‑accumulation
**What goes wrong:** Multiple steering forces (wander, seek, flocking) sum to extreme values, causing insects to vibrate or move unrealistically fast.
**Why it happens:** Forces are added without clamping or weighting.
**How to avoid:** Normalize the total force to a maximum magnitude; apply species‑specific weights to each force component.
**Warning signs:** Insects twitch in place or accelerate to unrealistic speeds.

### Pitfall 2: Scent Detection Performance
**What goes wrong:** Each insect checks distance to every food every frame, causing frame‑rate drops as food count increases.
**Why it happens:** Naïve O(n×m) double loop.
**How to avoid:** Use spatial partitioning: assign food to an Area3D group, insects use `get_overlapping_areas()` to only check nearby food. Alternatively, pre‑compute a scent grid updated infrequently.
**Warning signs:** Profiler shows high time in `distance_to()` calls; frame rate decreases when food count >20.

### Pitfall 3: RayCast Leftover State
**What goes wrong:** RayCast3D retains collision data from previous frame if not cleared, causing detection to “stick” after target moves away.
**Why it happens:** `is_colliding()` returns true until the next `force_raycast_update()`.
**How to avoid:** Call `force_raycast_update()` each frame before checking, or use a new RayCast each query.
**Warning signs:** Insects continue reacting to a neighbor that has left line‑of‑sight.

### Pitfall 4: Day/Night Cycle Lighting Pop
**What goes wrong:** DirectionalLight rotation changes abruptly at dawn/dusk, causing noticeable “pop” in lighting.
**Why it happens:** Linear interpolation of rotation without smoothing.
**How to avoid:** Use `lerp()` on light energy and color over several seconds; rotate light smoothly with eased interpolation.
**Warning signs:** Visible jump in shadow positions or ambient brightness every cycle.

## Code Examples

### Basic Wander Steering
```gdscript
# steering.gd
func wander(current_position: Vector3, current_direction: Vector3, wander_radius: float, wander_distance: float, wander_jitter: float) -> Vector3:
    # Add random displacement to current direction
    var jitter = Vector3(
        randf_range(-1, 1) * wander_jitter,
        0,
        randf_range(-1, 1) * wander_jitter
    )
    var target_local = current_direction * wander_distance + jitter
    var target_global = current_position + target_local.normalized() * wander_radius
    return (target_global - current_position).normalized()
```

### Flocking Cohesion, Separation, Alignment
```gdscript
# flocking.gd
func calculate(neighbors: Array, self_position: Vector3, self_velocity: Vector3) -> Vector3:
    var cohesion = Vector3.ZERO
    var separation = Vector3.ZERO
    var alignment = Vector3.ZERO
    var neighbor_count = 0
    
    for neighbor in neighbors:
        var dist = self_position.distance_to(neighbor.global_position)
        if dist > 0:
            cohesion += neighbor.global_position
            separation -= (neighbor.global_position - self_position).normalized() / dist
            alignment += neighbor.velocity
            neighbor_count += 1
    
    if neighbor_count > 0:
        cohesion = (cohesion / neighbor_count - self_position).normalized()
        separation = separation.normalized()
        alignment = (alignment / neighbor_count).normalized()
    
    return cohesion * cohesion_weight + separation * separation_weight + alignment * alignment_weight
```

### Scent‑Based Food Targeting
```gdscript
# detection.gd
func get_strongest_food_position() -> Vector3:
    var strongest_scent = 0.0
    var target_position = null
    
    for food in get_overlapping_areas():
        if food.is_in_group("food"):
            var scent = food.get_scent_at_position(global_position)
            if scent > strongest_scent:
                strongest_scent = scent
                target_position = food.global_position
    
    return target_position
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Fixed‑function animation systems | AnimationTree with state machine | Godot 4.0 (2023) | Enables complex blending, conditions, and layered animations with visual editor. |
| NavigationMesh baked offline | NavigationRegion3D with runtime updates | Godot 4.0 | Allows dynamic obstacles and terrain changes without full rebake. |
| Manual GDScript 1.0 typing | GDScript 2.0 with static typing | Godot 4.0 | Improves performance, editor autocompletion, and error detection. |

**Deprecated/outdated:**
- **Godot 3.x navigation:** Use NavigationRegion3D instead of NavigationMeshInstance.
- **`KinematicBody`:** Renamed to `CharacterBody3D` in Godot 4.
- **Immediate geometry drawing:** Use `MeshInstance3D` with `ArrayMesh` for performance.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | NavigationAgent3D provides velocity‑based steering suitable for insect wandering | Standard Stack | Might need custom steering implementation;会增加开发时间 |
| A2 | Area3D's `get_overlapping_bodies()` is performant enough for up to 50 insects | Don't Hand-Roll | If performance suffers,需要实现空间分区 |
| A3 | Scent gradient via distance falloff is sufficiently convincing for observation gameplay | Pattern 2 | Players might expect more sophisticated scent trails;需要升级到扩散网格 |
| A4 | Godot's built‑in `move_and_slide()` with collision normals enables “random turn on collision” | Common Pitfalls | Collision normal may not provide usable turn direction;需要额外逻辑 |
| A5 | Day/night cycle can be implemented by rotating a DirectionalLight | Architecture Patterns | Might need skybox shader or ambient color animation for visual quality |

## Open Questions

1. **Scent gradient implementation scope**
   - What we know: Food emits scent decaying with distance; insects sample strongest scent.
   - What's unclear: Should scent diffuse (spread over time) or be static? Should there be a global scent grid for performance?
   - Recommendation: Start with simple distance falloff; if performance becomes issue or scent trails need to be more realistic, implement a 2D texture grid updated every few frames.

2. **Temperature simulation detail**
   - What we know: Global temperature value varies with day/night cycle; insects seek preferred temperature zones.
   - What's unclear: How to define temperature zones? Should temperature be a 2D field (warmer near light source)?
   - Recommendation: Start with single global temperature; insects randomly move toward warmer/cooler areas based on preference. Later can add local heat sources (e.g., under light).

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Godot Engine | Entire phase | ✗ | — | **BLOCKING** – must be installed before implementation |
| Python 3 | Test execution (if using GUT) | ✓ | 3.14.4 | — |
| Git | Version control | ✓ | 2.46+ | — |

**Missing dependencies with no fallback:**
- Godot 4.6.2 – planner must include a setup task to download and install Godot.

**Missing dependencies with fallback:**
- None.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | GUT (Godot Unit Test) v9.1.0 |
| Config file | `gut_config.json` (to be created) |
| Quick run command | `godot -s addons/gut/gut_cmdln.gd -gtest=* -gmaximize` |
| Full suite command | `godot -s addons/gut/gut_cmdln.gd -gtest=* -gprint_gutconfig` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| SIM-01 | Insect moves with wander steering | unit | `godot -s addons/gut/gut_cmdln.gd -gtest=*wander*` | ❌ Wave 0 |
| SIM-02 | Insect seeks food via scent | unit | `godot -s addons/gut/gut_cmdln.gd -gtest=*scent*` | ❌ Wave 0 |
| SIM-03 | Insect approaches/avoids other species | unit | `godot -s addons/gut/gut_cmdln.gd -gtest=*interaction*` | ❌ Wave 0 |
| SIM-04 | Insect speed changes with light/terrain | unit | `godot -s addons/gut/gut_cmdln.gd -gtest=*environment*` | ❌ Wave 0 |
| INSECT-02 | Species have distinct movement patterns | unit | `godot -s addons/gut/gut_cmdln.gd -gtest=*species_movement*` | ❌ Wave 0 |
| INSECT-03 | Species have different food preferences | unit | `godot -s addons/gut/gut_cmdln.gd -gtest=*food_preference*` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** Run quick command for relevant test subset (e.g., only wander tests).
- **Per wave merge:** Run full suite for all phase‑related tests.
- **Phase gate:** Full suite green before `/gsd-verify-work`.

### Wave 0 Gaps
- [ ] `test_steering.gd` – covers SIM-01, INSECT-02
- [ ] `test_detection.gd` – covers SIM-02, INSECT-03
- [ ] `test_interaction.gd` – covers SIM-03
- [ ] `test_environment.gd` – covers SIM-04
- [ ] `gut_config.json` – GUT configuration
- [ ] GUT plugin install: `git submodule add https://github.com/bitwes/Gut.git addons/gut`

*(If no gaps: "None — existing test infrastructure covers all phase requirements")*

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No | Not required for single‑player observation game |
| V3 Session Management | No | No user sessions |
| V4 Access Control | No | No multi‑user or privilege separation |
| V5 Input Validation | Yes | Validate camera control inputs (mouse/keyboard) to avoid injection‑like bugs; use Godot's built‑in input mapping |
| V6 Cryptography | No | No sensitive data storage |

### Known Threat Patterns for Godot 3D Games

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Input injection via UI fields | Tampering | Use Godot's `LineEdit` with input validation; sanitize saved game files |
| Memory corruption via large asset loads | DoS | Implement resource streaming; limit concurrent asset loads |
| Shader‑based GPU crash | DoS | Test shaders on target hardware; provide fallback shaders |

## Sources

### Primary (HIGH confidence)
- Godot Engine 4.6.2 documentation (classes: NavigationAgent3D, RayCast3D, Area3D, AnimationPlayer) – verified via HTTPS fetch
- Existing project research: `.planning/research/NATURAL_ENVIRONMENT.md` – provides environment architecture patterns

### Secondary (MEDIUM confidence)
- General game AI patterns (steering, flocking, scent gradients) – based on training knowledge, not Godot‑specific
- GUT (Godot Unit Test) plugin documentation – assumed current as of 2026-04-17

### Tertiary (LOW confidence)
- Specific performance characteristics of `Area3D.get_overlapping_bodies()` with >50 entities – needs empirical validation
- Day/night cycle implementation details – assumed via DirectionalLight rotation; may need skybox shader adjustments

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH – Godot version and core classes verified via official docs
- Architecture: MEDIUM – patterns derived from existing research and general game AI principles; some implementation details unverified
- Pitfalls: MEDIUM – based on common Godot/game dev pitfalls; some are assumed from training data

**Research date:** 2026-04-17
**Valid until:** 2026-05-17 (30 days – Godot 4.6.x stable branch updates slowly)
