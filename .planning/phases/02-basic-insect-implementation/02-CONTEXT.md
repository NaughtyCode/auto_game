# Phase 2: Basic Insect Implementation - Context

**Gathered:** 2026-04-17
**Status:** Ready for planning

<domain>
## Phase Boundary

Player can observe multiple insect species with distinct appearances and basic animations in the environment. At least three different insect species (cricket, beetle, spider) are present with textured primitive shapes and skeletal animations. Camera automatically maintains reasonable viewing distance based on insect size.
</domain>

<decisions>
## Implementation Decisions

### Visual Representation
- **D-01:** Insect visual representation: Primitive shapes with textures
- **D-02:** Primitive detail level: Textured primitives
- **D-03:** Texture source: Procedural generation
- **D-04:** Procedural pattern complexity: Detailed insect‑like patterns

### Animation Approach
- **D-05:** Animation implementation: Skeletal animations
- **D-06:** Bone complexity: Moderate (6–10 bones per insect)
- **D-07:** Animation states needed: Walk, idle, turn, attack
- **D-08:** Animation state management: Custom script blending

### Species Selection & Distinctiveness
- **D-09:** Species selection: Cricket, beetle, spider
- **D-10:** Visual distinction: All of the above (size, color, pattern, shape)
- **D-11:** Insect size: 0.1 m (10 cm) relative to 0.5 m terrain tile
- **D-12:** Population density: 5–10 insects per species

### Camera Auto‑Distance (CAM‑04)
- **D-13:** Camera distance maintenance: Automatic distance adjustment
- **D-14:** Adjustment logic: Smoothly interpolate to optimal distance
- **D-15:** Optimal distance calculation: Based on insect size

### Claude's Discretion
Areas where the user deferred to Claude:
- Specific procedural texture generation algorithm and shader implementation
- Skeletal rigging details (exact bone placement and hierarchy)
- Custom script blending logic for animation transitions
- Exact primitive shape combinations per insect species
- Attack animation design (though fighting behavior is out of scope for this milestone)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project & Requirements
- `.planning/ROADMAP.md` — Phase 2 goal, requirements, success criteria
- `.planning/REQUIREMENTS.md` — INSECT-01, INSECT-04, INSECT-05, ENV-02, CAM-04

### Prior Phase Context
- `.planning/phases/01-environment-camera-setup/01-CONTEXT.md` — Terrain tile size (0.5 m), camera style (free‑fly 6DOF), boundaries, collision settings

### Research & Patterns
- `.planning/research/NATURAL_ENVIRONMENT.md` — Godot terrain, vegetation, and environment patterns
- `.planning/research/OBSERVATION_GAMEPLAY.md` — Camera control patterns for observation games
- `.planning/research/STACK.md` — Godot 4.6.2 and GDScript recommendation
- `.planning/research/ARCHITECTURE.md` — Godot project architecture patterns

### No External Specs
No external specification documents were referenced — requirements are fully captured in the decisions above.

</canonical_refs>

<code_context>
## Existing Code Insights

No existing codebase — this is a fresh Godot project. Phase 1 will establish the foundational scene (`main.tscn`) containing terrain, camera, lighting, and basic input mapping.

### Reusable Assets
None yet.

### Established Patterns
None yet.

### Integration Points
Insect entities (scenes) will be added as children of the terrain or a dedicated insect‑manager node in the main scene created in Phase 1.

</code_context>

<specifics>
## Specific Ideas

- Primitive shapes with procedural detailed insect‑like patterns (shader‑based)
- Skeletal animations with 6–10 bones per insect
- Three species: cricket, beetle, spider — each distinct in size, color, pattern, and shape
- Insect size: 0.1 m (10 cm) relative to 0.5 m terrain tile
- Population: 5–10 insects per species (15–30 total)
- Camera automatically adjusts viewing distance based on insect size, smoothly interpolating to optimal distance

No specific “like X” examples were given — standard Godot 4 patterns apply.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 02-basic-insect-implementation*  
*Context gathered: 2026-04-17*