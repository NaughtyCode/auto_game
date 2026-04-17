# Phase 1: Environment & Camera Setup - Context

**Gathered:** 2026-04-17
**Status:** Ready for planning

<domain>
## Phase Boundary

Player can navigate a basic 3D environment with smooth, responsive camera controls. The environment is a grassland/soil terrain with natural boundaries, providing a plausible space for insect activity. Camera supports free movement (pan, rotate, zoom) with collision detection.

</domain>

<decisions>
## Implementation Decisions

### Camera Movement
- **D-01:** Camera style: Free‑fly (first‑person) with full 6 degrees of freedom (move in any direction, rotate freely)
- **D-02:** Camera collides with terrain — prevents clipping
- **D-03:** Zoom works by moving camera forward/backward (physical movement)
- **D-04:** Camera movement uses smoothing (gradual acceleration/deceleration) for polished feel
- **D-05:** Camera sensitivity is configurable via settings (mouse sensitivity adjustable)

### Terrain Construction
- **D-06:** Tile cell size: 0.5 m (insect scale)
- **D-07:** Terrain has collision for camera and insects
- **D-08:** Terrain uses multiple material types (grass, soil, rock) for visual variety
- **D-09:** (Claude's discretion) Terrain construction method (GridMap tile‑based vs procedural heightmap)
- **D-10:** (Claude's discretion) Terrain size (approximate tile count)
- **D-11:** (Claude's discretion) Vegetation (grass, rocks) placement method (procedural vs manual)

### Environment Boundaries
- **D-12:** Boundaries implemented as invisible walls
- **D-13:** Boundary shape follows terrain shape (not rectangular)
- **D-14:** Boundaries are visible via debug visualization (wireframe/markers)
- **D-15:** Boundaries extend upward indefinitely (block camera from flying over terrain)
- **D-16:** (Claude's discretion) Whether boundaries block insects (camera blocking is decided)

### Terrain Elevation
- **D-17:** Terrain is flat — no elevation variation (hills)

### Claude's Discretion
Areas where the user deferred to Claude:
- Terrain construction method (GridMap vs procedural)
- Terrain size (approximate tile count)
- Vegetation placement method
- Whether boundaries block insects

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Godot Environment & Camera Patterns
- `.planning/phases/01-environment-camera-setup/01-RESEARCH.md` — Research summary with recommended architecture, standard stack, code examples, and pitfalls
- `.planning/research/NATURAL_ENVIRONMENT.md` — Godot terrain, vegetation, and environment patterns
- `.planning/research/OBSERVATION_GAMEPLAY.md` — Camera control patterns for observation games
- `.planning/research/STACK.md` — Godot 4.6.2 and GDScript recommendation

</canonical_refs>

<code_context>
## Existing Code Insights

No existing codebase — this is a fresh Godot project. All assets and scripts will be created in this phase.

### Reusable Assets
None yet.

### Established Patterns
None yet.

### Integration Points
The phase will establish the foundational scene (`main.tscn`) containing terrain, camera, lighting, and basic input mapping. Future phases will add insect entities and UI onto this base.

</code_context>

<specifics>
## Specific Ideas

- Camera: free‑fly with 6DOF, collision, smooth movement, configurable sensitivity, zoom via forward/backward movement
- Terrain: 0.5 m tile size, multiple materials (grass, soil, rock), flat, collision enabled
- Boundaries: invisible walls that follow terrain shape, debug‑visible, extend upward indefinitely
- Vegetation: to be placed (method at Claude's discretion)

No specific references or “like X” examples were given — standard Godot 4 patterns apply.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 01-environment-camera-setup*
*Context gathered: 2026-04-17*