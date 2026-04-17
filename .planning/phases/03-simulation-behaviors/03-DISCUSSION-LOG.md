# Phase 3: Simulation Behaviors - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-17
**Phase:** 3-simulation-behaviors
**Areas discussed:** Movement patterns, Food seeking & consumption, Insect interactions, Environmental stimuli & responses

---

## Movement patterns

| Option | Description | Selected |
|--------|-------------|----------|
| Random wandering with steering | Natural smooth movement using Godot's steering behaviors, avoids obstacles with raycasts (recommended for organic look) | ✓ |
| Grid‑based pathfinding | Moves tile‑by‑tile (0.5 m grid) using A* pathfinding — simpler but may look robotic | |
| Navigation mesh pathfinding | Uses Godot's NavigationAgent3D with baked navigation mesh — more complex setup but handles obstacles naturally | |

**User's choice:** Random wandering with steering
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — distinct movement per species | Cricket hops, beetle crawls, spider scuttles — each has unique speed, turn rate, animation style | ✓ |
| No — all move similarly | All insects share the same movement logic, differentiated only by appearance and size | |

**User's choice:** Yes — distinct movement per species
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Raycast detection + steering avoidance | Insects detect obstacles ahead with raycasts and steer around them — natural and responsive | |
| Ignore obstacles | Insects walk through obstacles (simpler but unrealistic) | |
| Collision detection + random turn | On collision, insects turn randomly — less predictable | ✓ |

**User's choice:** Collision detection + random turn
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — insects pause occasionally | More realistic — insects alternate between moving and idle states | ✓ |
| No — insects are always moving | Simpler — insects continuously wander | |

**User's choice:** Yes — insects pause occasionally
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — small random variation | Adds realism — each insect has slightly different speed | ✓ |
| No — all same speed | Simpler — consistent movement per species | |

**User's choice:** Yes — small random variation
**Notes:** None

---

## Food seeking & consumption

| Option | Description | Selected |
|--------|-------------|----------|
| Visible 3D objects | Fruit, leaf, or other natural models — players can see food in the environment | ✓ |
| Abstract markers | Invisible collision areas — simpler but less visual | |
| Particle effects | Glowing particles or sparkles — indicates food location | |

**User's choice:** Visible 3D objects
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Proximity radius | Insect senses food within a fixed radius — simple and effective | |
| Line of sight | Insect must see food (raycast) — more realistic but computationally heavier | |
| Smell gradient | Food emits scent gradient, insects follow gradient — complex simulation | ✓ |

**User's choice:** Smell gradient
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Stationary eating animation | Insect stops at food, plays eat animation, food disappears after animation — visually clear | ✓ |
| Continuous consumption | Insect reduces food value over time while moving — more realistic | |
| Instant consumption | Food disappears immediately on contact — simplest | |

**User's choice:** Stationary eating animation
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Randomly scattered | Food appears randomly across terrain — unpredictable | ✓ |
| Fixed locations | Food always appears in same spots — consistent | |
| Both random and fixed | Some fixed, some random — variety | |

**User's choice:** Randomly scattered
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — species have preferences | Adds variety — e.g., beetle prefers fruit, cricket prefers leaves | ✓ |
| No — all eat same food | Simpler — generic food works for all | |

**User's choice:** Yes — species have preferences
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — food regrows | Continuous simulation — food reappears after some time | ✓ |
| No — limited supply | Finite food — once eaten, it's gone | |

**User's choice:** Yes — food regrows
**Notes:** None

---

## Insect interactions

| Option | Description | Selected |
|--------|-------------|----------|
| Approach/avoid based on species | Insects react differently to other species — e.g., cricket avoids spider | ✓ |
| Ignore each other | No interactions — insects act independently | |
| Simple social behaviors | Grouping, following, flocking — adds emergent behavior | |

**User's choice:** Approach/avoid based on species
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Proximity radius | Insect senses others within a fixed radius — simple | |
| Line of sight | Insect must see another (raycast) — more realistic | |
| Both proximity and line of sight | Combines both — more complex | ✓ |

**User's choice:** Both proximity and line of sight
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — species‑specific matrix | Define approach/avoid/ignore for each pair (cricket vs spider, beetle vs cricket, etc.) | ✓ |
| No — all interactions same | All insects treat others the same (e.g., all avoid each other) | |

**User's choice:** Yes — species‑specific matrix
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Strong influence | Immediate turn toward/away — clear reaction | ✓ |
| Moderate influence | Gradual steering — natural adjustment | |
| Weak influence | Subtle bias — barely noticeable | |

**User's choice:** Strong influence
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — simple flocking | Insects of same species tend to stay close together — emergent groups | ✓ |
| No — independent movement | Each insect acts independently — simpler | |

**User's choice:** Yes — simple flocking
**Notes:** None

---

## Environmental stimuli & responses

| Option | Description | Selected |
|--------|-------------|----------|
| Light changes | Day/night cycle influences insect activity | ✓ |
| Terrain type | Insects behave differently on grass, soil, rock | ✓ |
| Obstacles | Rocks, vegetation affect movement and behavior | ✓ |
| Temperature | Simulated temperature affects insect speed/activity | ✓ |

**User's choice:** Light changes, Terrain type, Obstacles, Temperature
**Notes:** All four stimuli selected

| Option | Description | Selected |
|--------|-------------|----------|
| Change activity level | Insects move faster in day, slower at night — simple | ✓ |
| Move towards/away from light | Insects are attracted to or avoid light sources — more complex | |
| No behavioral response | Light changes only visual — simplest | |

**User's choice:** Change activity level
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Speed varies | Movement speed changes per terrain — e.g., slower on soil, faster on grass | ✓ |
| Seek preferred terrain | Insects are attracted to certain terrain types — adds complexity | |
| No effect | Terrain only visual — simplest | |

**User's choice:** Speed varies
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Speed varies | Insects move faster when warmer, slower when cooler | |
| Activity level changes | Warmer → more active, cooler → less active | |
| Seek warmer/cooler areas | Insects move toward preferred temperature zones — complex | ✓ |

**User's choice:** Seek warmer/cooler areas
**Notes:** None

| Option | Description | Selected |
|--------|-------------|----------|
| Global temperature | Single temperature value for entire environment — simpler | ✓ |
| Local variation | Temperature varies across terrain (sunny vs shade) — more realistic | |
| No variation | Temperature constant — simplest | |

**User's choice:** Global temperature
**Notes:** None

---

## Claude's Discretion

Areas where the user said "you decide" or deferred to Claude:
- Steering implementation details (weights, turn rates, wander parameters)
- Smell‑gradient implementation (range, decay, update frequency)
- Flocking algorithm parameters (cohesion, separation, alignment weights)
- Day/night cycle implementation and temperature simulation details
- Exact species‑pair interaction matrix values
- Specific food‑type assignments per species
- Exact speed multipliers per terrain type

## Deferred Ideas

None — discussion stayed within phase scope.