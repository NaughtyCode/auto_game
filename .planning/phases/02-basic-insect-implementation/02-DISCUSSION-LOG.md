# Phase 2: Basic Insect Implementation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.  
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-17  
**Phase:** 02-basic-insect-implementation  
**Areas discussed:** Insect visual representation, Animation approach, Species selection & distinctiveness, Camera auto‑distance

---

## Insect visual representation

| Option | Description | Selected |
|--------|-------------|----------|
| 3D models (purchased assets) | Use pre‑made 3D insect models from asset stores — fastest path to believable insects | |
| 3D models (custom) | Create custom insect models in Blender — most control but time‑consuming | |
| Primitive shapes | Use Godot primitives (cylinders, spheres) with textures — simple but less realistic | ✓ |
| 2D sprites (billboards) | Use 2D images that always face camera — cheap but breaks 3D immersion | |

**User's choice:** Primitive shapes

### Follow‑up questions

**What level of detail for primitive shapes?**

| Option | Selected |
|--------|----------|
| Simple colored primitives | |
| Textured primitives | ✓ |
| Primitive combinations | |

**Where will insect textures come from?**

| Option | Selected |
|--------|----------|
| Custom painted textures | |
| Procedural generation | ✓ |
| Reuse existing textures | |

**How complex should procedural patterns be?**

| Option | Selected |
|--------|----------|
| Simple color variations | |
| Patterned stripes/spots | |
| Detailed insect‑like patterns | ✓ |

**Notes:** User wants procedural detailed insect‑like patterns via shaders.

---

## Animation approach

| Option | Description | Selected |
|--------|-------------|----------|
| Simple translation/rotation | Move/rotate entire insect mesh for walking, bobbing for idle — easiest | |
| Skeletal animations | Use Godot's skeleton system for leg/wing movement — more realistic but complex | ✓ |
| Shader‑based movement | Animate legs via vertex shader — novel but limited to simple motions | |

**User's choice:** Skeletal animations

### Follow‑up questions

**How many bones per insect for skeletal animation?**

| Option | Selected |
|--------|----------|
| Minimal (3‑5 bones) | |
| Moderate (6‑10 bones) | ✓ |
| Detailed (10+ bones) | |

**Which animation states are needed for basic insect animations?**

| Option | Selected |
|--------|----------|
| Walk and idle only | |
| Walk, idle, and turn | |
| Walk, idle, turn, attack | ✓ |

**How should animation states be managed?**

| Option | Selected |
|--------|----------|
| Simple AnimationPlayer | |
| AnimationTree with state machine | |
| Custom script blending | ✓ |

**Notes:** Attack animation included despite fighting being out of scope for this milestone (超前部署). Animation state management deferred to custom script blending.

---

## Species selection & distinctiveness

| Option | Description | Selected |
|--------|-------------|----------|
| Cricket, Ant, Spider | As mentioned in requirements — covers diversity | |
| Cricket, Beetle, Spider | Replace ant with beetle — different body shape | ✓ |
| Ant, Beetle, Grasshopper | No spider — different movement styles | |
| Other (specify) | Choose your own combination | |

**User's choice:** Cricket, beetle, spider

### Follow‑up questions

**What primary visual distinction between species?**

| Option | Selected |
|--------|----------|
| Size and silhouette | |
| Color and pattern | |
| Shape combination | |
| All of the above | ✓ |

**What approximate size for insects (relative to 0.5m terrain tile)?**

| Option | Selected |
|--------|----------|
| 0.1m (10 cm) | ✓ |
| 0.05m (5 cm) | |
| 0.02m (2 cm) | |

**How many insects of each species in the environment?**

| Option | Selected |
|--------|----------|
| 1‑2 per species | |
| 3‑5 per species | |
| 5‑10 per species | ✓ |

**Notes:** All visual distinctions (size, color, pattern, shape) will be used. Insect size 0.1 m (10 cm). Population 5–10 per species.

---

## Camera auto‑distance (CAM‑04)

| Option | Description | Selected |
|--------|-------------|----------|
| Automatic distance adjustment | Camera automatically zooms in/out based on nearest insect — hands‑free | ✓ |
| Manual toggle to follow | Player presses a key to follow selected insect — player control | |
| Fixed distance with zoom | Camera stays at fixed distance, player manually zooms — simple | |

**User's choice:** Automatic distance adjustment

### Follow‑up questions

**What distance adjustment logic?**

| Option | Selected |
|--------|----------|
| Maintain constant distance | |
| Smoothly interpolate | ✓ |
| Snap to predefined distances | |

**How is optimal distance determined?**

| Option | Selected |
|--------|----------|
| Based on insect size | ✓ |
| Based on number of insects | |
| Fixed per insect species | |

**Notes:** Camera smoothly interpolates to optimal distance calculated based on insect size.

---

## Claude's Discretion

Areas where the user deferred to Claude (recorded in CONTEXT.md):

- Specific procedural texture generation algorithm and shader implementation
- Skeletal rigging details (exact bone placement and hierarchy)
- Custom script blending logic for animation transitions
- Exact primitive shape combinations per insect species
- Attack animation design (though fighting behavior is out of scope for this milestone)

## Deferred Ideas

None — discussion stayed within phase scope.

---

*Log compiled: 2026‑04‑17*