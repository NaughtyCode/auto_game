# Phase 1: Environment & Camera Setup - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-17
**Phase:** 01-environment-camera-setup
**Areas discussed:** Camera movement style, Terrain construction, Environment boundaries, Terrain elevation

---

## Camera Movement Style

| Option | Description | Selected |
|--------|-------------|----------|
| Orbital (SpringArm3D) | Rotates around a point, zooms in/out - recommended for observation, prevents clipping. | |
| Free-fly (first-person) | Direct movement like flying - more control but may cause disorientation. | ✓ |
| Top-down | Fixed overhead view - simpler but limited observation angles. | |

**User's choice:** Free-fly (first-person)
**Notes:** User wants maximum control for observation.

### Camera Movement Constraints

| Option | Description | Selected |
|--------|-------------|----------|
| Full 6DOF | Move freely in any direction, rotate freely - maximum control. | ✓ |
| Horizontal plane only | Move only on ground level, vertical rotation limited - easier to control. | |
| You decide | Claude can choose appropriate constraints. | |

**User's choice:** Full 6DOF

### Camera Collision with Terrain

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, prevent clipping | Camera stops when approaching terrain - more immersive. | ✓ |
| No, allow passing through | Camera can move through terrain - useful for debugging. | |
| You decide | Claude can implement appropriate collision. | |

**User's choice:** Yes, prevent clipping

### Camera Zoom Method

| Option | Description | Selected |
|--------|-------------|----------|
| Move camera forward/backward | Physical movement - feels natural for free-fly. | ✓ |
| Change field of view | Zoom lens effect - can see wider/narrower view. | |
| Both | Scroll wheel moves camera, modifier changes FOV. | |

**User's choice:** Move camera forward/backward

### Camera Movement Smoothing

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, smooth movement | Adds damping for more polished feel. | ✓ |
| No, instant response | Direct 1:1 input response. | |
| You decide | Claude can implement appropriate smoothing. | |

**User's choice:** Yes, smooth movement

### Camera Sensitivity Configuration

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, via settings | Allow players to adjust mouse sensitivity. | ✓ |
| No, fixed | Use a preset sensitivity. | |
| You decide | Claude can decide appropriate default. | |

**User's choice:** Yes, via settings

---

## Terrain Construction

### Terrain Construction Method

| Option | Description | Selected |
|--------|-------------|----------|
| GridMap tile-based | Pre-made tiles placed on grid - simpler, recommended by research. | |
| Procedural heightmap | Generate organic terrain with noise - more natural but complex. | |
| You decide | Claude can choose appropriate approach. | ✓ |

**User's choice:** You decide

### Terrain Size

| Option | Description | Selected |
|--------|-------------|----------|
| Small (20x20 tiles) | Compact area for focused observation. | |
| Medium (50x50) | Comfortable space for insect exploration. | |
| Large (100x100) | Expansive environment, more performance cost. | |
| You decide | Claude can choose appropriate size. | ✓ |

**User's choice:** You decide

### Tile Cell Size

| Option | Description | Selected |
|--------|-------------|----------|
| 0.5m | Insect scale - recommended by research. | ✓ |
| 1m | Moderate size. | |
| 2m | Large tiles. | |
| You decide | Claude can choose appropriate size. | |

**User's choice:** 0.5m

### Vegetation Placement Method

| Option | Description | Selected |
|--------|-------------|----------|
| Procedural placement | Randomly scatter within terrain - faster. | |
| Manual placement | Place each item manually - more control. | |
| Combination | Procedural with manual adjustments. | |
| You decide | Claude can choose appropriate method. | ✓ |

**User's choice:** You decide

### Terrain Collision

| Option | Description | Selected |
|--------|-------------|----------|
| Yes | Required for camera collision and insect navigation. | ✓ |
| No | No collision - camera can pass through. | |
| You decide | Claude can decide. | |

**User's choice:** Yes

### Terrain Material Variety

| Option | Description | Selected |
|--------|-------------|----------|
| Yes | Visual variety. | ✓ |
| No | Single material for simplicity. | |
| You decide | Claude can decide. | |

**User's choice:** Yes

---

## Environment Boundaries

### Boundary Implementation

| Option | Description | Selected |
|--------|-------------|----------|
| Invisible walls | Simple collision shapes at edges. | ✓ |
| Terrain edges (cliffs) | Visual cliffs that block movement. | |
| Combination | Some edges are cliffs, others invisible walls. | |
| You decide | Claude can choose appropriate method. | |

**User's choice:** Invisible walls

### Boundary Shape

| Option | Description | Selected |
|--------|-------------|----------|
| Rectangular | Simple rectangular area - easier to implement. | |
| Follow terrain shape | Boundaries match terrain edges - more natural. | ✓ |
| You decide | Claude can decide. | |

**User's choice:** Follow terrain shape

### Boundary Blocking

| Option | Description | Selected |
|--------|-------------|----------|
| Yes | Both camera and insects cannot pass. | |
| No | Allow passing through boundaries. | |
| You decide | Claude can decide. | ✓ |

**User's choice:** You decide

### Boundary Visibility

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, debug visualization | Show wireframe or markers for debugging. | ✓ |
| No, completely invisible | No visual indication. | |
| You decide | Claude can decide. | |

**User's choice:** Yes, debug visualization

### Boundary Height Extension

| Option | Description | Selected |
|--------|-------------|----------|
| Yes | Block camera from flying over terrain. | ✓ |
| No, limited height | Allow camera to fly over boundaries. | |
| You decide | Claude can decide. | |

**User's choice:** Yes

---

## Terrain Elevation

### Elevation Variation

| Option | Description | Selected |
|--------|-------------|----------|
| Flat | No elevation - simpler. | ✓ |
| Gentle hills | Subtle elevation for visual interest. | |
| Steep hills | More dramatic terrain. | |
| You decide | Claude can decide. | |

**User's choice:** Flat

---

## Claude's Discretion

Areas where user said "you decide":
- Terrain construction method (GridMap vs procedural)
- Terrain size (approximate tile count)
- Vegetation placement method
- Whether boundaries block insects

## Deferred Ideas

No deferred ideas mentioned during discussion.