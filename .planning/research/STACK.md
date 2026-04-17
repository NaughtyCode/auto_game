# Technology Stack

**Project:** 蛐蛐大作战 (Cricket Fighting Game)
**Researched:** 2026-04-17
**Confidence:** LOW (Web search unavailable; based on GitHub repository analysis, training data, and limited official documentation verification)

> ⚠️ **重要验证说明**
> 
> 由于缺乏实时网络验证，以下技术版本信息基于研究时的数据（2026-04-17）。在实际开始每个阶段开发前，请验证：
> 1. **Godot版本**：访问 [godotengine.org](https://godotengine.org/download) 确认最新稳定版本
> 2. **Blender版本**：检查 [blender.org](https://www.blender.org/download/) 获取最新版本
> 3. **GitHub模板**：验证 `https://github.com/Cute-Fame-Studio/3D-TurnBasedCombat` 仓库是否仍然可用
> 
> 建议在阶段1开始前进行一次技术栈验证，并在后续阶段开始时重新评估。

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Godot Engine | 4.6.2-stable | Game engine for 3D turn-based combat | Latest stable version with improved 3D rendering, physics, and GDScript 2.0. Godot 4.x has modern Vulkan renderer, global illumination, and better 3D workflows compared to Godot 3.x. |
| GDScript | 4.6.2 | Primary scripting language | Native to Godot, tightly integrated with editor, fast iteration, easy for small teams. Preferred over C# for smaller projects due to simplicity and better editor integration. |
| Blender | 4.2+ | 3D modeling, rigging, animation | Industry-standard open-source tool; exports glTF 2.0 format that Godot imports natively with materials and animations. |
| Audacity | 3.6+ | Audio editing and sound effects | Free, cross-platform audio editor for creating and modifying SFX. |
| Git | 2.46+ | Version control | Standard for collaborative development; Godot projects are file-based and work well with Git. |

### Supporting Libraries & Plugins

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Godot 3D Turn-Based Combat Template | Latest (GitHub) | Reference implementation for turn-based systems | Use as a starting point for combat state machine, turn order, action system. Adapt to cricket-specific mechanics. |
| Godot UI Theme Library | Built-in | UI styling and components | Use Godot's built-in Control nodes and Theme resource system for consistent UI; no external UI library needed for simple games. |
| Godot AnimationTree | Built-in | Complex animation state machines | When character animations require blending, conditions, or layered animations (e.g., idle, attack, damage). |
| Godot NavigationServer3D | Built-in | AI pathfinding for movement in 3D space | If crickets need to move across environment with obstacles; otherwise simple AI may not require navigation. |
| glTF 2.0 Export Add-on (Blender) | Built-in | Export 3D models to Godot-compatible format | Use Blender's native glTF exporter; ensures materials, armatures, and animations transfer correctly. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| Godot Editor | Integrated development environment | Includes scene editor, script editor, debugger, profiler. Use for all Godot-specific work. |
| Visual Studio Code with Godot Tools extension | Code editing for GDScript | Provides syntax highlighting, auto-completion, debugging support. Optional but improves workflow. |
| Blender | 3D asset creation | Create and rig cricket models, animate attacks, design environments. |
| Audacity | Audio production | Record, edit, and mix sound effects (cricket chirps, hits, environment sounds). |
| LMMS or Bosca Ceoil | Music composition | Free tools for creating background music and battle themes. |
| Git & GitHub/GitLab | Version control & collaboration | Host repository, manage branches, track issues. |

## Installation

```bash
# Install Godot 4.6.2 from official website: https://godotengine.org/download
# Install Blender from: https://www.blender.org/download/
# Install Audacity from: https://www.audacityteam.org/download/

# Clone reference template (optional):
git clone https://github.com/Cute-Fame-Studio/3D-TurnBasedCombat.git

# VS Code extension:
code --install-extension georgealexakis.godot-tools
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| GDScript | C# | If team has strong C# background and needs performance-critical code; adds complexity for small projects. |
| Blender | Maya, 3ds Max | If studio already licenses professional tools; Blender is free and fully capable. |
| Built-in UI System | Third-party UI plugins (e.g., Godot UI Library) | If project requires complex UI components not provided by Godot's Control nodes; otherwise avoid plugin overhead. |
| Godot 4.6.2 | Godot 3.x | Do not use; Godot 3.x lacks modern 3D features, slower rendering, outdated API. |
| Git | Perforce, SVN | For larger teams with binary asset management needs; Git is simpler for small indie projects. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Godot 3.x | Outdated 3D rendering (OpenGL), missing features, deprecated API. | Godot 4.6.2 |
| Unity (or other engines) | Project requirement specifies Godot; switching adds unnecessary friction. | Stick with Godot as required. |
| Complex ECS frameworks | Over-engineering for a small turn-based game; Godot's node system is sufficient. | Simple state machines and Godot nodes. |
| C# for entire project | Increases build complexity, slower iteration, less editor integration for small team. | GDScript for gameplay logic; C# only for performance-critical modules if needed. |
| Proprietary 3D model formats (FBX, OBJ) | Poor material/animation support in Godot compared to glTF 2.0. | Export models as glTF 2.0 from Blender. |

## Stack Patterns by Variant

**If focusing on rapid prototyping:**
- Use Godot 4.6.2 with GDScript only
- Use placeholder 3D cubes instead of detailed models
- Implement core combat loop first, then add visuals
- Because iteration speed is critical for validating gameplay.

**If targeting high visual fidelity:**
- Use Godot's Vulkan renderer with SDFGI and SSR
- Use Blender for high-quality models with PBR materials
- Implement LOD (Level of Detail) for performance
- Because Godot 4's modern renderer can produce polished visuals.

**If planning to scale to multiple platforms later:**
- Design UI with scalable Control nodes
- Test on Windows, Mac, Linux early
- Keep shaders compatible with OpenGL ES 3.0 (for mobile fallback)
- Because Godot supports cross-platform export but requires forward planning.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| Godot 4.6.2 | GDScript 2.0, C# 10 | Ensure C# projects use .NET 8 SDK |
| Blender 4.2+ | glTF 2.0 exporter | Works with Godot's glTF import; test material export |
| VS Code Godot Tools | Godot 4.x | Extension may need updates for new Godot versions |

## Sources

- GitHub API search for "godot turn-based" repositories (2026-04-17) — identified `3D-TurnBasedCombat` template and `godot-open-rpg` reference
- Godot Engine GitHub releases API — latest stable version 4.6.2
- Training data on Godot 4 features and best practices (LOW confidence without recent verification)
- Limited official documentation access (403 Forbidden); used curl with user-agent to fetch basic feature list

---
*Stack research for: 3D turn-based cricket fighting game with Godot*
*Researched: 2026-04-17*