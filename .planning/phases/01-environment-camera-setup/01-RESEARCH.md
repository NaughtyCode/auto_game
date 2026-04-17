# Phase 1: Environment & Camera Setup - Research

**Researched:** 2026-04-17
**Domain:** Godot 4 3D environment creation and camera controls
**Confidence:** MEDIUM (Existing research available but some claims need verification with official docs)

## Summary

Phase 1 establishes the foundational 3D environment and camera system for the insect observation game. The player must be able to navigate a grassland/soil environment with natural boundaries using smooth, responsive camera controls (pan, rotate, zoom). This phase addresses requirements ENV-01, ENV-03, CAM-01, and CAM-03.

The Godot 4 engine provides built‑in nodes for terrain (GridMap), vegetation (MultiMeshInstance), and camera control (Camera3D + SpringArm3D). The environment should be a bounded but plausibly natural space where insects can later move and interact. Camera movement must feel fluid and intuitive, with collision detection to prevent passing through terrain.

**Primary recommendation:** Use Godot 4.6.2 with GridMap for tile‑based terrain, MultiMeshInstance for grass/foliage, and a custom CameraController script attached to a SpringArm3D for six‑degree‑of‑freedom camera movement with smoothing and collision avoidance.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Terrain rendering | 3D rendering pipeline (Godot) | — | Godot's GridMap and MeshInstance handle geometry and material rendering. |
| Vegetation placement | 3D rendering pipeline (Godot) | Game logic (placement algorithm) | MultiMeshInstance efficiently renders many identical meshes; logic decides positions. |
| Camera movement | Frontend (CameraController script) | 3D physics (collision detection) | CameraController reads input and transforms the camera node; SpringArm3D handles collision with environment. |
| Environment boundaries | Game logic (terrain design) | 3D physics (collision shapes) | Terrain mesh edges and/or invisible CollisionShape3D nodes define navigable area. |
| Smooth camera interpolation | Frontend (CameraController script) | — | Interpolation (lerp) and input filtering are purely script‑side operations. |

## User Constraints (from CONTEXT.md)

No CONTEXT.md exists for this phase — all decisions are open for discussion.

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ENV-01 | User can navigate a basic grassland/soil environment scene | GridMap tile‑based terrain with grass/soil materials; bounded play area. |
| ENV-03 | Environment provides natural boundaries and space for insect movement | Terrain edges and/or invisible walls; sufficient flat area for insect AI navigation. |
| CAM-01 | User can freely pan, rotate, and zoom the camera in 3D space | CameraController script with WASD translation, mouse‑drag rotation, scroll‑wheel zoom, all attached to SpringArm3D. |
| CAM-03 | User experiences smooth camera movement and responsive controls | Input smoothing (lerp), inertia, adjustable sensitivity, and collision‑aware SpringArm3D. |

## Standard Stack

### Core
| Library/Component | Version | Purpose | Why Standard |
|------------------|---------|---------|--------------|
| Godot Engine | 4.6.2‑stable | Game engine for 3D rendering, physics, scripting | Latest stable version with modern Vulkan renderer, improved 3D workflow, and active community. [VERIFIED: GitHub repository] |
| GridMap | Built‑in | Tile‑based terrain creation | Provides a grid‑oriented placement system for terrain tiles with automatic collision and LOD. [ASSUMED] |
| MultiMeshInstance | Built‑in | Efficient rendering of many identical meshes (grass, rocks) | GPU‑instanced rendering allows thousands of vegetation instances with minimal performance cost. [ASSUMED] |
| Camera3D + SpringArm3D | Built‑in | Observer camera with collision detection | SpringArm3D automatically shortens when colliding with geometry, preventing camera clipping. [ASSUMED] |
| GDScript | Built‑in | Gameplay logic scripting | Native Godot language with tight editor integration, fast iteration, and beginner‑friendly syntax. [ASSUMED] |

### Supporting
| Library/Component | Version | Purpose | When to Use |
|------------------|---------|---------|-------------|
| FastNoiseLite | Built‑in (Godot 4) | Procedural height‑map generation | When terrain needs organic elevation variation (optional for Phase 1). [ASSUMED] |
| WorldEnvironment | Built‑in | Sky, ambient light, fog | To set the scene's lighting and atmospheric backdrop. [ASSUMED] |
| CollisionShape3D | Built‑in | Invisible boundaries | When terrain edges need precise collision for camera/insects. [ASSUMED] |
| InputMap | Built‑in | Configurable input actions | To map keyboard/mouse inputs to camera functions (customizable by player). [ASSUMED] |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| GridMap | Voxel‑based plugin (e.g., VoxelTools) | More detailed/editable terrain but steeper learning curve and performance overhead. |
| MultiMeshInstance | Manual MeshInstance placement | Impossible for dense vegetation; manual placement is tedious and performance‑heavy. |
| SpringArm3D | Custom ray‑cast collision | Requires more code and tuning; SpringArm3D is battle‑tested and simple. |
| GDScript | C# | Better performance for intensive calculations but adds build complexity and slower iteration. |

**Installation:**
```bash
# Download Godot 4.6.2 from https://godotengine.org/download
# No additional npm packages required for this phase.
```

**Version verification:** Godot 4.6.2 confirmed via GitHub repository (2026‑04‑17). [VERIFIED: GitHub repository]

## Architecture Patterns

### System Architecture Diagram

```
Player Input (WASD, Mouse, Scroll)
    ↓
Camera Controller (GDScript)
    ├── Translates input into movement/rotation/zoom commands
    ├── Applies smoothing and inertia filters
    └── Updates SpringArm3D target length and position
        ↓
SpringArm3D (Godot node)
    ├── Checks collision along its length
    ├── Shortens if collision detected (prevents clipping)
    └── Positions child Camera3D accordingly
        ↓
Camera3D (Godot node)
    ├── Renders view to screen
    └── Can be toggled between perspective/orthographic
        ↓
Rendered View (Player sees)
    ↓
Environment (GridMap + MultiMeshInstance)
    ├── Terrain tiles (grass, soil) with collision
    ├── Vegetation instances (grass clumps, small rocks)
    └── Invisible boundary colliders (edges of play area)
```

### Recommended Project Structure
```
src/
├── environments/
│   ├── terrain/           # GridMap tile scenes and materials
│   └── vegetation/        # Grass/rock meshes for MultiMeshInstance
├── scripts/
│   ├── camera_controller.gd   # Main camera input handling
│   └── environment_builder.gd # Procedural terrain generation (optional)
└── scenes/
    ├── main.tscn          # Root scene with world, camera, lighting
    └── debug/             # Debug visualization scenes
```

### Pattern 1: SpringArm‑based Camera with Collision Avoidance
**What:** Attach the Camera3D as a child of a SpringArm3D node. The spring arm automatically detects collisions along its length and shortens to keep the camera from passing through geometry.
**When to use:** Any 3D game where the camera can move freely and should not clip into walls, terrain, or other objects.
**Example:**
```gdscript
# Source: Godot community best practice [ASSUMED]
extends SpringArm3D

@export var camera_sensitivity: float = 0.005
@export var zoom_speed: float = 2.0
@export var move_speed: float = 5.0

var target_length: float = 10.0

func _ready():
    target_length = spring_length

func _process(delta):
    # Zoom with scroll wheel
    target_length -= Input.get_axis("zoom_in", "zoom_out") * zoom_speed * delta
    target_length = clamp(target_length, 2.0, 20.0)
    spring_length = lerp(spring_length, target_length, 0.1)
    
    # Rotation with mouse drag
    if Input.is_action_pressed("rotate_camera"):
        rotation.y -= Input.get_last_mouse_velocity().x * camera_sensitivity
        rotation.x = clamp(
            rotation.x + Input.get_last_mouse_velocity().y * camera_sensitivity,
            -PI/4, PI/4
        )
    
    # Translation with WASD
    var move_dir = Input.get_vector("move_left", "move_right", "move_forward", "move_back")
    global_translate(Vector3(move_dir.x, 0, move_dir.y) * move_speed * delta)
```

### Pattern 2: GridMap Tile‑based Terrain
**What:** Use Godot's GridMap node to place pre‑modeled tile meshes (grass, soil, etc.) on a 3D grid. Each tile can have its own collision shape and material.
**When to use:** When the environment is composed of repeating elements (e.g., grassland patches, soil patches) and needs consistent collision.
**Example:**
```gdscript
# Source: Godot community best practice [ASSUMED]
extends GridMap

func generate_basic_terrain(width: int, depth: int):
    var grass_tile = preload("res://environments/terrain/grass_tile.tres")
    var soil_tile = preload("res://environments/terrain/soil_tile.tres")
    
    for x in range(width):
        for z in range(depth):
            # Simple pattern: grass on outer edges, soil inside
            if x == 0 or z == 0 or x == width-1 or z == depth-1:
                set_cell_item(Vector3i(x, 0, z), grass_tile)
            else:
                set_cell_item(Vector3i(x, 0, z), soil_tile)
```

### Anti‑Patterns to Avoid
- **Hard‑coding camera sensitivity/limits in script:** Makes tuning difficult. Use `@export` variables so values can be adjusted in the editor.
- **Placing every vegetation mesh individually:** Performance killer. Always use MultiMeshInstance for identical repeated meshes.
- **No camera collision:** Allows camera to clip through terrain, breaking immersion. Always use SpringArm3D or custom ray‑cast collision.
- **Ignoring input smoothing:** Jerky camera movement feels unprofessional. Apply lerp or inertia to all input‑driven transformations.

## Don't Hand‑Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Terrain collision shapes | Manual collision geometry for each tile | GridMap's built‑in per‑tile collision | GridMap automatically generates accurate collisions for each tile mesh. |
| Camera‑wall collision | Custom ray‑cast system | SpringArm3D node | SpringArm3D is optimized, handles edge cases, and requires minimal code. |
| Vegetation rendering | Individual MeshInstance nodes | MultiMeshInstance | Reduces draw calls from thousands to one, crucial for performance. |
| Input action mapping | Hard‑coded key checks | InputMap resource | Allows player rebinding and cleaner code separation. |
| Smooth movement | Frame‑rate‑dependent translation | `delta`‑scaled movement + lerp | Ensures consistent speed across different framerates and provides smooth interpolation. |

**Key insight:** Godot's built‑in nodes (GridMap, MultiMeshInstance, SpringArm3D) are already optimized for their respective tasks. Custom implementations are unlikely to match their performance or stability and will increase development time.

## Common Pitfalls

### Pitfall 1: Camera Clipping Through Terrain
**What goes wrong:** Player zooms or moves camera into a hill or ground, seeing inside geometry.
**Why it happens:** Camera lacks collision detection or SpringArm3D is not configured properly.
**How to avoid:** Always attach Camera3D as child of a SpringArm3D. Test by moving camera toward walls and slopes.
**Warning signs:** Camera passes through terrain in early testing.

### Pitfall 2: Jerky Camera Rotation
**What goes wrong:** Camera rotation feels sticky, laggy, or jumps inconsistently.
**Why it happens:** Input is processed raw without smoothing, or rotation is applied directly to camera node (causing gimbal lock).
**How to avoid:** Rotate the parent SpringArm3D (not the camera), apply mouse velocity filtering, and lerp rotation values.
**Warning signs:** Camera movement feels “binary” (instant jumps) or framerate‑dependent.

### Pitfall 3: Terrain Tiles Not Aligning
**What goes wrong:** Gaps or overlaps between GridMap tiles, breaking visual continuity.
**Why it happens:** Tile meshes are not designed to snap to grid, or GridMap cell size mismatches tile size.
**How to avoid:** Ensure tile meshes are modeled to exact cell dimensions (e.g., 2m × 2m × 0.2m) and set GridMap cell size accordingly.
**Warning signs:** Visible seams between tiles in editor preview.

### Pitfall 4: Performance Drop with Vegetation
**What goes wrong:** Frame rate plummets when looking at grassy areas.
**Why it happens:** Using individual MeshInstance nodes instead of MultiMeshInstance, or too many high‑poly grass meshes.
**How to avoid:** Use MultiMeshInstance for all repeated vegetation. Use low‑poly placeholder meshes during development.
**Warning signs:** Profiler shows high draw‑call count or high CPU usage in vegetation‑heavy areas.

## Code Examples

### Basic Camera Controller with SpringArm
```gdscript
# camera_controller.gd
extends SpringArm3D

@export var mouse_sensitivity: float = 0.005
@export var move_speed: float = 10.0
@export var zoom_speed: float = 3.0
@export var smooth_factor: float = 0.1

var target_spring_length: float = 8.0
var current_velocity: Vector3 = Vector3.ZERO

func _ready():
    target_spring_length = spring_length
    Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)  # Optional: hide mouse

func _input(event):
    if event is InputEventMouseMotion and Input.is_action_pressed("rotate_camera"):
        rotation.y -= event.relative.x * mouse_sensitivity
        rotation.x = clamp(rotation.x - event.relative.y * mouse_sensitivity, -PI/3, PI/3)
    
    if event is InputEventMouseButton:
        if event.button_index == MOUSE_BUTTON_WHEEL_UP:
            target_spring_length -= zoom_speed
        if event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
            target_spring_length += zoom_speed
        target_spring_length = clamp(target_spring_length, 2.0, 20.0)

func _process(delta):
    # Smooth zoom
    spring_length = lerp(spring_length, target_spring_length, smooth_factor)
    
    # WASD movement
    var move_dir = Input.get_vector("move_left", "move_right", "move_forward", "move_back")
    var forward = -global_transform.basis.z
    var right = global_transform.basis.x
    var move_vector = (forward * move_dir.y + right * move_dir.x) * move_speed * delta
    
    global_translate(move_vector)
```

### Simple GridMap Terrain Setup
```gdscript
# environment_builder.gd
extends GridMap

enum Tiles { GRASS = 0, SOIL = 1, ROCK = 2 }

func _ready():
    var tile_size = cell_size
    var ground_width = 20
    var ground_depth = 20
    
    # Fill ground with soil
    for x in range(ground_width):
        for z in range(ground_depth):
            set_cell_item(Vector3i(x, 0, z), Tiles.SOIL)
    
    # Add grass border
    for x in range(ground_width):
        set_cell_item(Vector3i(x, 0, 0), Tiles.GRASS)
        set_cell_item(Vector3i(x, 0, ground_depth-1), Tiles.GRASS)
    for z in range(ground_depth):
        set_cell_item(Vector3i(0, 0, z), Tiles.GRASS)
        set_cell_item(Vector3i(ground_width-1, 0, z), Tiles.GRASS)
    
    # Add a few rock patches
    for i in range(5):
        var rx = randi_range(3, ground_width-4)
        var rz = randi_range(3, ground_depth-4)
        set_cell_item(Vector3i(rx, 0, rz), Tiles.ROCK)
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Godot 3.x (OpenGL) | Godot 4.x (Vulkan) | Godot 4.0 (2022) | Better 3D rendering, global illumination, SDFGI, improved particle system. |
| Manual camera collision | SpringArm3D node | Godot 3.1 (2019) | Built‑in collision‑aware camera arm reduces boilerplate and edge‑case bugs. |
| Individual MeshInstances | MultiMeshInstance | Godot 3.0 (2018) | Massive performance gain for repeated geometry (grass, trees, etc.). |
| Fixed‑function terrain | GridMap + procedural generation | Godot 3.2 (2020) | More flexible, reusable terrain system that supports large environments. |

**Deprecated/outdated:**
- **Godot 3.x:** Missing modern Vulkan features, slower 3D rendering, outdated API.
- **Custom camera collision scripts:** Prone to bugs and harder to maintain than SpringArm3D.
- **Placing every vegetation mesh by hand:** Not scalable beyond a few dozen instances.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | GridMap is suitable for tile‑based grassland/soil terrain. | Standard Stack | Medium – GridMap may have limitations (max size, performance) that require alternative terrain system. |
| A2 | MultiMeshInstance is the optimal way to render grass/foliage. | Standard Stack | Low – This is a well‑established Godot best practice. |
| A3 | SpringArm3D provides adequate collision detection for camera. | Standard Stack | Low – SpringArm3D is widely used for third‑person cameras; should work for observation camera. |
| A4 | GDScript is sufficient for camera and environment logic. | Standard Stack | Low – GDScript performance is adequate for this phase; can switch to C# later if needed. |
| A5 | The code examples follow correct Godot 4 GDScript syntax. | Code Examples | Medium – Syntax may have changed; examples need testing in actual editor. |
| A6 | InputMap can be configured for camera controls. | Standard Stack | Low – InputMap is a core Godot feature and well‑documented. |

**If this table is empty:** All claims in this research were verified or cited — no user confirmation needed.

## Open Questions

1. **What is the optimal GridMap cell size for insect‑scale environment?**
   - What we know: Insects are small (centimeter scale). Terrain tiles should be large enough to accommodate insect movement but not so large that the environment feels blocky.
   - What's unclear: Exact tile dimensions that balance visual fidelity and performance.
   - Recommendation: Start with 0.5m × 0.5m tiles and adjust after testing with placeholder insect models.

2. **Should terrain have elevation variation (hills) in Phase 1?**
   - What we know: Requirements only specify grassland/soil with natural boundaries. Elevation is not required but could add visual interest.
   - What's unclear: Whether hills would complicate insect AI (future phase) or camera collision.
   - Recommendation: Keep terrain flat for Phase 1; add hills in a later polish phase.

3. **How should “natural boundaries” be implemented?**
   - What we know: Environment must prevent player from leaving play area.
   - What's unclear: Whether to use invisible walls, terrain edges (cliffs), or a combination.
   - Recommendation: Use a combination of terrain mesh edges (where appropriate) and invisible CollisionShape3D walls where needed.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Godot Engine | All development | ✗ | — | Must install before implementation |
| Blender (optional) | Asset creation | ? | — | Use built‑in primitive meshes for placeholder |
| Git | Version control | ✓ | 2.46+ | — |

**Missing dependencies with no fallback:**
- Godot Engine – must be downloaded and installed before any work can begin.

**Missing dependencies with fallback:**
- Blender – not strictly required for Phase 1; can use Godot's built‑in primitive meshes (Cube, Sphere) as placeholder terrain/vegetation.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | GUT (Godot Unit Test) + manual playtesting |
| Config file | `res://test/gut_config.gd` (to be created) |
| Quick run command | `godot --run-tests --test=**/test_camera*.gd` |
| Full suite command | `godot --run-all-tests` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ENV-01 | Terrain renders correctly | Integration | `godot --run-tests --test=test_terrain_rendering` | ❌ Wave 0 |
| ENV-03 | Boundaries prevent camera exit | Integration | `godot --run-tests --test=test_boundaries` | ❌ Wave 0 |
| CAM-01 | Camera responds to input | Unit | `godot --run-tests --test=test_camera_input` | ❌ Wave 0 |
| CAM-03 | Camera movement is smooth | Manual (visual) | — | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** Run relevant unit tests (if any)
- **Per wave merge:** Run full test suite (if implemented)
- **Phase gate:** Full manual playtest to verify smoothness and responsiveness before `/gsd-verify-work`

### Wave 0 Gaps
- [ ] `test/test_terrain_rendering.gd` – covers ENV‑01
- [ ] `test/test_boundaries.gd` – covers ENV‑03  
- [ ] `test/test_camera_input.gd` – covers CAM‑01
- [ ] Framework installation: `git clone https://github.com/bitwes/Gut.git` into `addons/gut` – if not detected
- [ ] No automated test for CAM‑03 (smooth movement) – must rely on manual playtesting.

*(If no gaps: "None — existing test infrastructure covers all phase requirements")*

## Security Domain

> Security enforcement is enabled (default). However, this phase involves no authentication, network communication, or sensitive data storage.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No | No user accounts required. |
| V3 Session Management | No | No online sessions. |
| V4 Access Control | No | No permission system. |
| V5 Input Validation | Yes | Godot's built‑in input validation; sanitize any exported variables that could cause crashes. |
| V6 Cryptography | No | No sensitive data to encrypt. |

### Known Threat Patterns for Godot Games

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Resource exhaustion (memory) | Denial of Service | Implement resource limits; use streaming for large environments. |
| Input injection (crash via invalid input) | Tampering | Validate exported script variables and input actions. |
| Save‑game tampering | Spoofing | Not applicable in Phase 1 (no save system). |

## Sources

### Primary (HIGH confidence)
- GitHub repository verification (2026‑04‑17) – Godot Engine latest stable version is 4.6.2 [VERIFIED: GitHub repository]

### Secondary (MEDIUM confidence)
- `.planning/research/NATURAL_ENVIRONMENT.md` – Godot terrain, vegetation, and environment patterns (2026‑04‑17)
- `.planning/research/OBSERVATION_GAMEPLAY.md` – Camera control patterns for observation games (2026‑04‑17)
- `.planning/research/STACK.md` – Godot 4.6.2 and GDScript recommendation (2026‑04‑17)

### Tertiary (LOW confidence)
- Training knowledge of Godot 4 API details – marked as [ASSUMED] in relevant sections.

## Metadata

**Confidence breakdown:**
- Standard stack: MEDIUM – Godot version verified; built‑in components assumed standard but not verified against official docs.
- Architecture: MEDIUM – Patterns derived from existing project research, but some details may need adjustment.
- Pitfalls: HIGH – Common issues in 3D game development are well‑known.
- Code examples: LOW – Based on training knowledge; need testing in actual Godot editor.

**Research date:** 2026‑04‑17
**Valid until:** 30 days (Godot 4 API relatively stable; verify before implementation)