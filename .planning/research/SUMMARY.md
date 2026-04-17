# Research Summary: 蛐蛐大作战 (Cricket Fighting Game)

**Domain:** 3D turn-based cricket fighting game  
**Researched:** 2026-04-17  
**Overall confidence:** MEDIUM

## Executive Summary

The cricket fighting game is a 3D turn-based combat game where players control anthropomorphic crickets in tactical battles against AI opponents. Research reveals a well-established pattern for turn-based combat architecture in Godot, centered around a **Battle Manager** coordinating turn order, AP tracking, and action resolution. The recommended stack is **Godot 4.6.2 with GDScript**, using resource files (.tres) for cricket stats and skills. Key differentiators include anthropomorphic cricket characters with clothing/hats, environment interactions, and cricket-specific abilities.

Critical pitfalls include coupling animation timing with game logic, hardcoded formulas, and global state abuse—all of which can cause rewrites. The architecture should separate presentation (UI, camera, animations) from gameplay logic, using signals for communication.

## Key Findings

**Stack:** Godot 4.6.2 + GDScript + Blender for 3D models, using resource-based configuration for balancing.  
**Architecture:** Centralized Battle Manager with event delegation, Battler components for crickets, separate AI and Environment managers.  
**Critical pitfall:** Embedding game logic in animation timelines causes desync and limits iteration; must use signal-based damage application.

## Implications for Roadmap

Based on research, suggested phase structure:

1. **Phase 1: Core Combat Loop** - Establish turn order, basic attack/defend actions, health/AP systems
   - Addresses: Table stakes (clear turn indicator, health bars, basic actions)
   - Avoids: Overly complex AI early
   - Build order: BattleManager → Battler → simple UI → camera control

2. **Phase 2: Action Point & Skill System** - Implement AP tracking, skill resources, multiple actions
   - Addresses: Core AP system, special skills, attribute system
   - Avoids: Hardcoded formulas
   - Build order: AP system → Skill resources → Formulas class → expanded UI

3. **Phase 3: AI Opponent & Environment** - Add intelligent AI, interactive arena elements
   - Addresses: AI opponent, environment interactions, terrain effects
   - Avoids: Global state abuse
   - Build order: AIManager → EnvironmentManager → terrain modifiers

4. **Phase 4: Polish & Differentiation** - Anthropomorphic cricket models, animations, visual/audio feedback
   - Addresses: Anthropomorphic characters, humorous animations, multiple environments
   - Avoids: Animation-logic coupling
   - Build order: 3D models → AnimationTree → VFX/Audio → camera polish

**Phase ordering rationale:**
- Core combat must work before adding AP complexity.
- AI needs actions to evaluate, so skill system must precede AI.
- Environment interactions enhance existing combat, not foundational.
- Visual polish depends on functional gameplay to avoid reworking assets.

**Research flags for phases:**
- Phase 2 (AP & Skill System): Likely needs deeper research on balancing AP costs and skill formulas.
- Phase 3 (AI & Environment): AI decision algorithms may need iterative testing.
- Phase 4 (Polish): Animation integration patterns need careful implementation.

## Confidence Assessment

| Area | Confidence | Reason |
|------|------------|--------|
| Stack | MEDIUM | Based on GitHub template analysis and recent web verification (Godot 4.6.2, Blender 5.1, template exists). |
| Features | HIGH | Derived from project requirements and standard patterns. |
| Architecture | MEDIUM | Validated against existing Godot template. |
| Pitfalls | MEDIUM | Based on common project issues in Godot community. |

## Gaps to Address

- **Godot-specific performance considerations** for 3D turn-based games.
- **Multi-platform export nuances**.
- **Advanced AI techniques** for turn-based games.
- **Network architecture** (if future multiplayer considered).

These gaps can be addressed during phase-specific research.

---
*Research summary for: 3D turn-based cricket fighting game*
*Researched: 2026-04-17*
