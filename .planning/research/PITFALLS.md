# Domain Pitfalls

**Domain:** 3D turn-based cricket fighting game  
**Researched:** 2026-04-17  
**Confidence:** MEDIUM (Based on analysis of common Godot project issues, turn-based game design patterns, and template code review)

## Critical Pitfalls

Mistakes that cause rewrites or major issues.

### Pitfall 1: Coupling Animation Timing with Game Logic

**What goes wrong:** Damage calculation or state changes are triggered directly from AnimationPlayer keyframes or `animation_finished` signals, causing game logic to depend on specific animation lengths. Changing animations breaks combat balance; pausing/skipping animations causes desync.

**Why it happens:** Natural tendency to "just make it work" by embedding logic in visual timelines.

**Consequences:** Unreliable damage application, inability to implement fast-forward, debugging nightmares, limits animation iteration.

**Prevention:** Use a two-phase approach:  
1. BattleManager executes action → triggers animation.  
2. Animation emits `anim_damage_frame` signal (via dedicated keyframe or function call).  
3. BattleManager applies damage/effects upon signal, independent of animation length.  
Keep game logic in scripts, not animation timelines.

**Detection:** Search for `AnimationPlayer` connections to game logic methods; look for `yield(animation_player, "animation_finished")` in combat code.

### Pitfall 2: Hardcoded Attribute Formulas

**What goes wrong:** Damage formula `attacker.power - target.defense` written directly in BattleManager. Becomes impossible to adjust per skill, add critical hits, or factor in terrain without modifying core code.

**Why it happens:** Quick prototyping without anticipating future balancing needs.

**Consequences:** Endless code tweaks for balancing, inability to create unique skills, formula changes risk breaking unrelated systems.

**Prevention:** Create a `Formulas` static class with methods like `physical_damage(attacker, target, skill)`, `magic_damage(...)`, `calculate_hit_chance(...)`. Skill resources can specify which formula to use. This centralizes balancing.

**Detection:** Look for inline arithmetic in `take_damage` or battle manager methods; absence of formula abstraction.

### Pitfall 3: Global State for Battle Data

**What goes wrong:** Using an autoload singleton to store current turn, battlers, or action points. Prevents multiple battle instances (e.g., tutorial + main battle), causes state corruption on scene reload, complicates testing.

**Why it happens:** Convenient access from anywhere; seems simpler than passing references.

**Consequences:** Cannot have separate battles; state persists incorrectly after battle ends; hard to isolate for unit tests.

**Prevention:** Keep battle state within BattleManager instance. Pass BattleManager reference to components that need it (dependency injection). Use signals for cross-component communication rather than global access.

**Detection:** Autoload scripts with battle-related variables; direct `GlobalBattle.current_turn` references.

### Pitfall 4: Missing Turn Timeout Safety

**What goes wrong:** Player or AI gets stuck in a state (e.g., target selection, animation wait) and turn never advances, soft-locking the game.

**Why it happens:** Assuming all code paths will complete correctly; not accounting for edge cases like missing animations, buggy AI.

**Consequences:** Players need to force-quit; negative reviews; hard to debug live.

**Prevention:** Implement turn timer in BattleManager: if `current_turn` exceeds N seconds without progressing, force-advance. Include movement timeouts for battlers stuck advancing.

**Detection:** No timeout logic in turn loops; absence of `force_turn_advance` method.

## Moderate Pitfalls

### Pitfall 1: Overly Complex AI Early

**What goes wrong:** Building sophisticated AI with decision trees, utility scoring, and learning before core combat is fun. Wastes time on AI that will need rebalancing anyway.

**Why it happens:** Desire to make "smart" opponents; underestimating gameplay iteration.

**Consequences:** AI becomes coupled to unstable game mechanics; requires constant rework; delays playtesting.

**Prevention:** Start with random action selection weighted by intelligence. Add simple heuristics (e.g., low HP → heal). Polish AI after core gameplay is locked.

**Detection:** AI script > 300 lines in MVP; complex scoring algorithms before basic combat works.

### Pitfall 2: 3D Camera Clipping and Disorientation

**What goes wrong:** Camera zooms through terrain, loses characters behind obstacles, or rotates in unintuitive ways, frustrating players.

**Why it happens:** Using basic Camera3D without collision detection; not limiting rotation/zoom; assuming empty arena.

**Consequences:** Poor player experience; negative visual feedback; may make game unplayable.

**Prevention:** Implement camera collision (raycast from camera to target), clamp zoom distances, smooth damping, and optional camera reset button. Test with all arena geometry.

**Detection:** No collision detection on camera; free unrestricted rotation.

### Pitfall 3: Not Using Resource System for Balancing

**What goes wrong:** Cricket stats, skill values, and AP costs hardcoded in scripts. Requires programmer involvement for every balance tweak.

**Why it happens:** Perceived complexity of Godot's Resource system; quick iteration in code.

**Consequences:** Lengthy compile-run-test cycles for designers; inability to create data mods; version control diffs in code instead of data.

**Prevention:** Create `BattlerStats`, `SkillData`, `ArenaModifier` resources. Export them in inspector. Use tool scripts to allow in-editor balancing.

**Detection:** Numeric literals in gameplay scripts; lack of `.tres` files.

### Pitfall 4: Ignoring UI Scale and Input Flexibility

**What goes wrong:** UI designed for one screen resolution; assumes mouse-only input; no controller support.

**Why it happens:** Targeting PC but not considering varying monitor sizes; prototyping with mouse only.

**Consequences:** UI breaks on different resolutions; excludes players preferring keyboard/controller.

**Prevention:** Use Godot's Container nodes and anchors for responsive UI. Implement both mouse and keyboard navigation for battle menus. Test at 1080p, 1440p, 4K.

**Detection:** Absolute positioning in UI; no keyboard navigation code.

## Minor Pitfalls

### Pitfall 1: Inefficient Animation Loading

**What goes wrong:** Loading individual animation files per cricket type causing long load times and memory bloat.

**Why it happens:** Creating separate AnimationPlayer for each cricket scene.

**Consequences:** Slow scene transitions; increased memory usage.

**Prevention:** Share animation libraries via inherited scenes or AnimationTree states. Use `AnimationLibrary` to pack animations.

**Detection:** Multiple identical animation files across characters.

### Pitfall 2: Poor Error Handling for Missing Resources

**What goes wrong:** Assuming `@export` resource will always be set; game crashes with "null instance" when designer forgets to assign skill.

**Why it happens:** Not validating resource references at runtime.

**Consequences:** Crashes in production; difficult for non-programmers to use editor.

**Prevention:** Add `assert(skill != null, "Skill not assigned")` in `_ready()` or use `get_or_default()` fallback. Provide clear error messages.

**Detection:** No null checks before using `@export` resources.

### Pitfall 3: Not Planning for Save/Load

**What goes wrong:** Battle state scattered across nodes with no serialization plan, making save/load impossible without rewrite.

**Why it happens:** "We'll add it later" mentality.

**Consequences:** Major refactoring required; may limit game features (e.g., mid-battle save).

**Prevention:** Design Battler and BattleManager with serialization in mind: keep state in dictionaries that can be `serialize()`/`deserialize()`. Use Godot's `Resource` for saved games.

**Detection:** State variables mixed with presentation logic; no serialization methods.

### Pitfall 4: Over-Optimizing Too Early

**What goes wrong:** Implementing object pooling, LOD, culling for a turn-based game with 2 crickets.

**Why it happens:** Premature optimization based on assumptions.

**Consequences:** Wasted development time; increased code complexity; harder to modify.

**Prevention:** Profile first. Turn-based 3D with few characters is unlikely to be GPU/CPU bound. Optimize only after measuring bottlenecks.

**Detection:** Complex pooling systems before performance testing.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| **Core Combat Loop** | Hardcoded turn order that doesn't support speed attribute later. | Implement turn order as array sorted by speed; start with simple alternation but keep sorting structure. |
| **Action Point System** | AP costs that make some actions never used (too expensive) or always used (too cheap). | Use spreadsheet balancing; make AP costs configurable per skill via resources. |
| **AI Implementation** | AI that feels predictable or cheats (has perfect knowledge). | Add randomness weighted by intelligence; limit AI to information player would have (e.g., visible health bars). |
| **Environment Interactions** | Terrain effects that are forgettable or overwhelming. | Start with simple +/- stat modifiers; make UI show active terrain effects. |
| **Animation Integration** | Animations too long, slowing combat pace. | Allow animation skipping or speed-up; keep attack animations under 1 second. |
| **Camera Control** | Camera that doesn't show action clearly. | Implement camera presets (side, top, close) plus free orbit; playtest with non-devs. |
| **UI/UX** | Action buttons not clear; players don't understand AP costs. | Use tooltips, color coding, and disabled states for insufficient AP. |

## Sources

- Analysis of `3D-TurnBasedCombat` template code (noted bugs and workarounds)
- Common Godot project issues from community forums (training data)
- Turn-based game design patterns (XCOM, Final Fantasy Tactics post-mortems)
- Project requirements and constraints from `.planning/PROJECT.md`

---
*Domain pitfalls for: 3D turn-based cricket fighting game*
*Researched: 2026-04-17*