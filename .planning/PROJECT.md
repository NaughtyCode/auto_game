# 蛐蛐大作战

## What This Is

一个使用Godot引擎开发的3D回合制蛐蛐战斗游戏，玩家控制拟人化蛐蛐与AI对战，包含多种环境和交互元素。目标用户是喜欢轻度策略战斗的玩家。

## Core Value

提供有趣、策略性的蛐蛐战斗体验，玩家需要管理行动点和属性来击败对手。

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] 回合制行动点战斗系统 — 每回合分配行动点，选择攻击/防御/技能
- [ ] 蛐蛐属性系统 — 力量、速度、防御、生命值四种核心属性
- [ ] 四种战斗行动 — 普通攻击、重击、防御、特殊技能
- [ ] 中度拟人化蛐蛐3D模型 — 穿衣服、戴帽子、有肢体动作的蛐蛐
- [ ] 多种战斗环境 — 草地、沙地、室内等环境，包含可交互元素
- [ ] AI对手 — 智能AI系统，提供有挑战性的对战
- [ ] 用户界面 — 战斗UI、行动按钮、属性显示等

### Out of Scope

- 在线多人对战 — 仅支持单人对战AI，简化网络复杂度
- 复杂培养系统 — 无蛐蛐升级、技能树、装备系统
- 故事模式 — 无剧情任务或角色发展
- 移动平台支持 — 仅针对PC平台开发

## Context

- 技术栈：Godot 4引擎，使用GDScript或C#进行开发
- 图形：3D全视角，可旋转摄像头观察战斗
- 目标平台：PC（Windows/Mac/Linux）
- 项目规模：小型独立游戏，聚焦核心战斗玩法

## Constraints

- **技术栈**：Godot 4引擎 — 用户明确要求使用Godot
- **平台**：PC平台 — 用户选择PC作为目标平台
- **美术资源**：需要3D模型和动画 — 增加开发复杂度
- **时间**：小型项目范围 — 需要控制功能范围，避免过度复杂化

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Godot引擎 | 用户明确要求使用Godot引擎进行开发 | — Pending |
| 3D全视角 | 用户选择3D全视角图形，提供更好的视觉体验 | — Pending |
| PC平台 | 用户选择PC作为目标发布平台 | — Pending |
| 回合制行动点系统 | 用户选择行动点系统而非传统回合制 | — Pending |
| 四种核心属性 | 用户选择力量、速度、防御、生命值作为蛐蛐属性 | — Pending |
| 四种战斗行动 | 用户选择普通攻击、重击、防御、特殊技能 | — Pending |
| 中度拟人化 | 用户选择中度拟人化（穿衣服、戴帽子） | — Pending |
| 环境交互元素 | 用户希望环境包含可交互元素，不仅仅是视觉背景 | — Pending |
| 简单战斗模式 | 用户选择简单战斗模式，无复杂培养或剧情系统 | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-17 after initialization*