# Phase 3: Simulation Behaviors - Context

**Gathered:** 2026-04-17
**Status:** Ready for planning

<domain>
## Phase Boundary

Player can observe insects exhibiting natural behaviors: moving with species‑appropriate patterns, seeking and consuming food, interacting with each other based on species, and responding to environmental stimuli (light, terrain, obstacles, temperature). All behaviors are observational — no player‑controlled combat or complex AI.

</domain>

<decisions>
## Implementation Decisions

### Movement & Navigation
- **D-01:** Movement style: Random wandering with steering (Godot steering behaviors)
- **D-02:** Species‑specific movement: Each insect species (cricket, beetle, spider) has distinct locomotion (hops, crawls, scuttles)
- **D-03:** Obstacle handling: Collision detection + random turn (no raycast steering)
- **D-04:** Idle behavior: Insects periodically pause moving (alternate between moving and idle states)
- **D-05:** Speed variation: Individual insects of the same species have small random speed differences
- **D-06:** (Claude's discretion) Steering implementation details (weights, turn rates, wander parameters)

### Food & Consumption
- **D-07:** Food representation: Visible 3D objects (fruit, leaf, etc.) placed in the environment
- **D-08:** Food detection: Smell gradient – food emits a scent gradient that insects follow
- **D-09:** Consumption method: Stationary eating animation (insect stops, plays eat animation, food disappears after)
- **D-10:** Food placement: Randomly scattered across the terrain
- **D-11:** Food preferences: Different insect species prefer different food types (e.g., beetle→fruit, cricket→leaf)
- **D-12:** Food respawn: Food regrows after being consumed (continuous supply)
- **D-13:** (Claude's discretion) Smell‑gradient implementation (range, decay, update frequency)

### Insect Interactions
- **D-14:** Interaction type: Approach/avoid based on species (e.g., cricket avoids spider)
- **D-15:** Detection method: Both proximity radius and line‑of‑sight (raycast)
- **D-16:** Interaction matrix: Defined per species pair (cricket–spider, beetle–cricket, etc.)
- **D-17:** Influence strength: Strong influence – immediate turn toward/away
- **D-18:** Social grouping: Simple flocking – insects of the same species tend to stay close together
- **D-19:** (Claude's discretion) Flocking algorithm parameters (cohesion, separation, alignment weights)

### Environmental Responses
- **D-20:** Active stimuli: Light changes, terrain type, obstacles, temperature
- **D-21:** Light response: Change activity level (move faster in day, slower at night)
- **D-22:** Terrain response: Speed varies per terrain type (e.g., slower on soil, faster on grass)
- **D-23:** Temperature response: Seek warmer/cooler areas (insects move toward preferred temperature zones)
- **D-24:** Temperature variation: Global temperature (single value for entire environment)
- **D-25:** (Claude's discretion) Day/night cycle implementation and temperature simulation details

### Claude's Discretion
Areas where the user deferred to Claude:
- Steering implementation details (weights, turn rates, wander parameters)
- Smell‑gradient implementation (range, decay, update frequency)
- Flocking algorithm parameters (cohesion, separation, alignment weights)
- Day/night cycle implementation and temperature simulation details
- Exact species‑pair interaction matrix values
- Specific food‑type assignments per species
- Exact speed multipliers per terrain type

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project & Requirements
- `.planning/ROADMAP.md` — Phase 3 goal, requirements, success criteria
- `.planning/REQUIREMENTS.md` — SIM-01..SIM-04, INSECT-02, INSECT-03

### Prior Phase Context
- `.planning/phases/01-environment-camera-setup/01-CONTEXT.md` — Terrain tile size (0.5 m), camera style (free‑fly 6DOF), boundaries, collision settings
- `.planning/phases/02-basic-insect-implementation/02-CONTEXT.md` — Insect size (0.1 m), primitive shapes with textures, skeletal animations, species selection, population density, camera auto‑distance

### Research & Patterns
- `.planning/research/ARCHITECTURE.md` — Godot project architecture patterns
- `.planning/research/NATURAL_ENVIRONMENT.md` — Godot terrain, vegetation, and environment patterns
- `.planning/research/OBSERVATION_GAMEPLAY.md` — Camera control patterns for observation games
- `.planning/research/STACK.md` — Godot 4.6.2 and GDScript recommendation
- `.planning/research/FEATURES.md` — Feature research summary
- `.planning/research/PITFALLS.md` — Common pitfalls and how to avoid them

### No External Specs
No external specification documents were referenced — requirements are fully captured in the decisions above.

</canonical_refs>

<code_context>
## Existing Code Insights

No existing codebase — this is a fresh Godot project. Phase 1 will establish the foundational scene (`main.tscn`) containing terrain, camera, lighting, and basic input mapping; Phase 2 will add insect entities with primitive shapes and skeletal animations.

### Reusable Assets
None yet.

### Established Patterns
None yet.

### Integration Points
Simulation behaviors will be added as scripts/components on the insect scenes created in Phase 2. Food objects will be added as child nodes of the terrain or a dedicated food‑manager node. Environmental stimuli (light, temperature) will be managed by a global simulation manager that insects query.

</code_context>

<specifics>
## Specific Ideas

- Movement: random wandering with steering, collision‑based obstacle avoidance, species‑specific locomotion styles
- Food: visible 3D objects, smell‑gradient detection, stationary eating animation, random placement, species‑specific preferences, respawning
- Interactions: approach/avoid per species pair, detection via proximity + line‑of‑sight, strong influence, simple flocking
- Environment: light‑based activity change, terrain‑dependent speed, temperature‑seeking behavior, global temperature

No specific “like X” examples were given — standard Godot 4 patterns apply.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 03-simulation-behaviors*  
*Context gathered: 2026-04-17*