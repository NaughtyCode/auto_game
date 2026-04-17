# ququ - 昆虫观察游戏

## What This Is

一个使用Godot引擎开发的小游戏，玩家可以观察不同昆虫之间的互动和打架行为，享受昆虫世界的乐趣。游戏侧重于观察和模拟昆虫行为，而非传统的战斗游戏。目标用户是喜欢放松、观察和探索自然行为的玩家。

## Core Value

提供放松有趣的昆虫观察体验，玩家可以观察不同昆虫的自然行为和互动，包括打架行为。游戏重点是模拟昆虫世界，让玩家享受观察和发现的乐趣。

## Current Milestone: v1.0 Core Simulation

**Goal:** Build the core insect simulation system for observing insect behaviors in a natural 3D environment.

**Target features:**
- 昆虫模拟系统 — 不同昆虫的行为模拟，包括移动、觅食、互动等
- 多种昆虫类型 — 至少3-5种不同昆虫，如蛐蛐、蚂蚁、蜘蛛、甲虫等
- 自然环境场景 — 草地、泥土、石头等自然环境，昆虫可以在其中活动
- 观察者视角 — 自由移动的相机，玩家可以观察昆虫行为
- 简单用户界面 — 昆虫信息显示、观察提示等基础界面

## Requirements

### Validated

(None yet — ship to validate)

### Active

**Core Simulation (Milestone v1.0):**
- [ ] 昆虫模拟系统 — 不同昆虫的行为模拟，包括移动、觅食、互动等
- [ ] 多种昆虫类型 — 至少3-5种不同昆虫，如蛐蛐、蚂蚁、蜘蛛、甲虫等
- [ ] 自然环境场景 — 草地、泥土、石头等自然环境，昆虫可以在其中活动
- [ ] 观察者视角 — 自由移动的相机，玩家可以观察昆虫行为
- [ ] 用户界面 — 昆虫信息显示、观察提示等基础界面

**Future Milestones:**
- [ ] 昆虫打架行为 — 模拟昆虫之间的打架行为，有简单规则和动画
- [ ] 3D昆虫模型 — 真实或卡通风格的昆虫3D模型和动画
- [ ] 互动系统 — 玩家可以投放食物或轻微干扰昆虫行为
- [ ] 环境控制 — 高级环境交互和控制界面

### Out of Scope

- 在线多人观察 — 仅支持单人观察体验，简化网络复杂度
- 复杂培养系统 — 无昆虫升级、进化、装备系统
- 故事模式 — 无剧情任务或角色发展
- 移动平台支持 — 仅针对PC平台开发
- 复杂战斗系统 — 无回合制、行动点等传统战斗机制
- 玩家控制昆虫 — 玩家不能直接控制昆虫，只能观察和轻微影响

## Context

- 技术栈：Godot 4引擎，使用GDScript或C#进行开发
- 图形：3D全视角，可旋转摄像头观察昆虫行为
- 目标平台：PC（Windows/Mac/Linux）
- 项目规模：小型独立游戏，聚焦昆虫观察和模拟

## Constraints

- **技术栈**：Godot 4引擎 — 用户明确要求使用Godot
- **平台**：PC平台 — 用户选择PC作为目标平台
- **美术资源**：需要多种昆虫3D模型和动画 — 增加开发复杂度
- **时间**：小型项目范围 — 需要控制功能范围，避免过度复杂化
- **游戏类型**：观察式游戏，非传统战斗游戏 — 需要设计合适的观察机制

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Godot引擎 | 用户明确要求使用Godot引擎进行开发 | — Pending |
| 3D全视角 | 用户选择3D全视角图形，提供更好的观察体验 | — Pending |
| PC平台 | 用户选择PC作为目标发布平台 | — Pending |
| 观察式游戏玩法 | 用户选择观察昆虫行为而非控制昆虫战斗 | — Pending |
| 昆虫模拟系统 | 需要模拟多种昆虫行为和互动，包括打架 | — Pending |
| 环境互动 | 用户希望环境是昆虫活动的自然场景 | — Pending |
| 轻度玩家交互 | 玩家可以观察和轻微影响，但不能直接控制昆虫 | — Pending |

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
*Last updated: 2026-04-17 — Milestone v1.0 started*