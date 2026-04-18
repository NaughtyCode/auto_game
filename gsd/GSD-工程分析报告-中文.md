# GSD (Get Shit Done) 工程分析报告

## 概述

**GSD (Get Shit Done)** 是一个轻量级但功能强大的元提示 (meta-prompting)、上下文工程 (context engineering) 和规格驱动开发 (spec-driven development) 系统，专为 AI 辅助编程工具设计。它解决了 **context rot**（上下文窗口填满导致输出质量劣化）问题，通过系统化的上下文管理、多代理协作和原子化任务执行，使 AI 辅助编程变得可靠且高效。

### 核心价值主张
- **解决上下文劣化**: 通过任务分片、新鲜上下文执行和上下文预算管理，保持 AI 输出质量
- **规格驱动开发**: 从需求描述到代码实现的系统化流程
- **多代理协作**: 专业代理分工协作，各司其职
- **原子化 Git 提交**: 每个任务都有可追溯的提交记录
- **支持多种运行时**: Claude Code、OpenCode、Gemini CLI、Codex、Copilot、Cursor、Windsurf、Antigravity、Augment、Trae、Qwen Code、CodeBuddy、Cline 等

## 核心功能和用法总结

| 类别 | 关键功能 | 主要命令/用法 |
|------|----------|----------------|
| **上下文管理** | 防止上下文劣化，预算管理，新鲜上下文执行 | 自动集成到所有工作流中 |
| **多代理协作** | 20+ 专业代理分工（规划、执行、研究、验证等） | 通过 `/gsd-plan-phase`, `/gsd-execute-phase` 等自动调用 |
| **规格驱动开发** | 从需求到代码的系统化流程，XML 提示格式化 | `/gsd-new-project`, `/gsd-discuss-phase`, `/gsd-plan-phase` |
| **原子化执行** | 每个任务独立提交，Git 历史清晰 | `/gsd-execute-phase` 波浪式执行 |
| **工作流管理** | 项目状态跟踪，决策记录，进度监控 | `STATE.md`, `ROADMAP.md`, `/gsd-progress`, `/gsd-next` |
| **快速任务** | 临时任务处理，跳过完整规划 | `/gsd-quick <任务描述>` |
| **代码质量** | 代码审查，安全加固，UI 审计 | `/gsd-review`, `/gsd-secure-phase`, `/gsd-ui-review` |
| **配置与设置** | 模型配置，工作流开关，Git 分支策略 | `/gsd-settings`, `/gsd-set-profile`, `.planning/config.json` |
| **故障排查** | 健康检查，调试，会话恢复 | `/gsd-health`, `/gsd-debug`, `/gsd-resume-work` |
| **多项目支持** | 工作区隔离，工作流并行 | `/gsd-new-workspace`, `/gsd-workstreams` |

> **典型工作流**:  
> 1. **初始化**: `/gsd-new-project` → 项目愿景、需求、路线图  
> 2. **阶段循环**: 对每个阶段执行:  
>    - **讨论**: `/gsd-discuss-phase N` → 锁定实现偏好  
>    - **规划**: `/gsd-plan-phase N` → 研究 + 计划 + 验证  
>    - **执行**: `/gsd-execute-phase N` → 并行波浪执行  
>    - **验证**: `/gsd-verify-work N` → 用户验收测试  
> 3. **发布**: `/gsd-ship N` → 创建 PR  
> 4. **里程碑管理**: `/gsd-complete-milestone`, `/gsd-new-milestone`

## 核心功能

### 1. 上下文工程系统
- **上下文预算管理**: 根据模型上下文窗口大小（200k/1M）动态调整读取深度
- **上下文退化监控**: 分四级监控（PEAK/GOOD/DEGRADING/POOR），适时警告
- **新鲜上下文执行**: 每个任务在全新上下文中执行，避免累积垃圾

### 2. 多代理架构
GSD 使用专业代理分工协作，每个代理有明确职责：

| 代理类型 | 主要职责 |
|---------|----------|
| `gsd-planner` | 创建可执行的阶段计划，任务分解，依赖分析 |
| `gsd-executor` | 执行计划，生成代码，创建原子提交 |
| `gsd-phase-researcher` | 阶段范围的研究（技术栈、特性、架构、陷阱） |
| `gsd-project-researcher` | 项目范围的研究 |
| `gsd-plan-checker` | 验证计划是否达到阶段目标 |
| `gsd-research-synthesizer` | 多研究结果合成 |
| `gsd-roadmapper` | 创建和修订路线图 |
| `gsd-verifier` | 执行后验证，检查交付成果 |
| `gsd-codebase-mapper` | 代码库分析（现有项目） |
| `gsd-debugger` | 调试调查，根因分析 |
| `gsd-ui-researcher` | UI 规格创建 |
| `gsd-ui-auditor` | UI 审查 |
| 等 20+ 种专业代理 |

### 3. XML 提示格式化
所有计划使用结构化 XML 格式，为 Claude 优化：
```xml
<task type="auto">
  <name>创建登录端点</name>
  <files>src/app/api/auth/login/route.ts</files>
  <action>
    使用 jose 处理 JWT（避免 jsonwebtoken 的 CommonJS 问题）。
    验证用户凭据。
    成功时返回 httpOnly cookie。
  </action>
  <verify>curl -X POST localhost:3000/api/auth/login 返回 200 + Set-Cookie</verify>
  <done>有效凭据返回 cookie，无效返回 401</done>
</task>
```

### 4. 原子 Git 提交
- 每个任务完成后立即提交
- 清晰的提交信息格式：`类型(阶段-计划): 描述`
- 支持 Git bisect 和独立回退

### 5. 工作流状态管理
- **PROJECT.md**: 项目愿景，始终加载
- **REQUIREMENTS.md**: 范围化的 v1/v2 需求，阶段可追溯性
- **ROADMAP.md**: 路线图，阶段分解
- **STATE.md**: 项目状态、决策、阻塞项、会话连续性
- **CONTEXT.md**: 用户实现决策（每个阶段）
- **RESEARCH.md**: 研究成果
- **PLAN.md**: 原子任务计划
- **SUMMARY.md**: 执行摘要，提交历史
- **VERIFICATION.md**: 验证报告

## 工作流程

### 完整项目生命周期

```
┌──────────────────────────────────────────────────┐
│                   新项目初始化                   │
│  /gsd-new-project                                │
│  问题 → 研究 → 需求 → 路线图                    │
└─────────────────────────┬────────────────────────┘
                          │
           ┌──────────────▼─────────────┐
           │      对每个阶段循环:       │
           │                            │
           │  ┌────────────────────┐    │
           │  │ /gsd-discuss-phase │    │  ← 锁定实现偏好
           │  └──────────┬─────────┘    │
           │             │              │
           │  ┌──────────▼─────────┐    │
           │  │ /gsd-ui-phase      │    │  ← UI 设计合约（前端）
           │  └──────────┬─────────┘    │
           │             │              │
           │  ┌──────────▼─────────┐    │
           │  │ /gsd-plan-phase    │    │  ← 研究 + 计划 + 验证
           │  └──────────┬─────────┘    │
           │             │              │
           │  ┌──────────▼─────────┐    │
           │  │ /gsd-execute-phase │    │  ← 并行执行
           │  └──────────┬─────────┘    │
           │             │              │
           │  ┌──────────▼─────────┐    │
           │  │ /gsd-verify-work   │    │  ← 手动用户验收测试
           │  └──────────┬─────────┘    │
           │             │              │
           │  ┌──────────▼─────────┐    │
           │  │ /gsd-ship          │    │  ← 创建 PR（可选）
           │  └──────────┬─────────┘    │
           │             │              │
           │     下一阶段？─────────────┘
           │             │ 否
           └─────────────┼──────────────┘
                          │
          ┌───────────────▼──────────────┐
          │  /gsd-audit-milestone        │
          │  /gsd-complete-milestone     │
          └───────────────┬──────────────┘
                          │
                 还有里程碑吗？
                     │          │
                    是          否 → 完成！
                     │
             ┌───────▼──────────────┐
             │  /gsd-new-milestone  │
             └──────────────────────┘
```

### 关键阶段详解

#### 1. 新项目初始化 (`/gsd-new-project`)
- **深度提问**: 理解项目目标、约束、技术偏好、边界情况
- **研究**: 并行代理研究领域（可选但推荐）
- **需求提取**: 区分 v1、v2 和范围外
- **路线图创建**: 创建阶段映射到需求

**产出**: `PROJECT.md`, `REQUIREMENTS.md`, `ROADMAP.md`, `STATE.md`, `.planning/research/`

#### 2. 阶段讨论 (`/gsd-discuss-phase N`)
- **捕获实现决策**: 在计划前锁定用户偏好
- **分析灰色区域**: 根据构建内容识别决策点
  - 视觉特性 → 布局、密度、交互、空状态
  - API/CLI → 响应格式、标志、错误处理、详细程度
  - 内容系统 → 结构、语气、深度、流程
  - 组织任务 → 分组标准、命名、重复项、例外
- **产出**: `{phase_num}-CONTEXT.md`

#### 3. 阶段计划 (`/gsd-plan-phase N`)
- **研究**: 研究如何实现此阶段，由 CONTEXT.md 决策指导
- **计划**: 创建 2-3 个原子任务计划，XML 结构
- **验证**: 检查计划是否符合需求，循环直到通过

**产出**: `{phase_num}-RESEARCH.md`, `{phase_num}-{N}-PLAN.md`

#### 4. 阶段执行 (`/gsd-execute-phase N`)
- **波浪式执行**: 基于依赖关系分组，组内并行，组间顺序
- **新鲜上下文**: 每个计划在全新 200k token 上下文中执行
- **原子提交**: 每个任务独立提交
- **目标验证**: 检查代码库是否交付阶段承诺

**波浪执行示例**:
```
WAVE 1 (并行)          WAVE 2 (并行)          WAVE 3
┌─────────┐ ┌─────────┐    ┌─────────┐ ┌─────────┐    ┌─────────┐
│ Plan 01 │ │ Plan 02 │ →  │ Plan 03 │ │ Plan 04 │ →  │ Plan 05 │
│         │ │         │    │         │ │         │    │         │
│ 用户    │ │ 产品    │    │ 订单    │ │ 购物车  │    │ 结账    │
│ 模型    │ │ 模型    │    │ API     │ │ API     │    │ UI      │
└─────────┘ └─────────┘    └─────────┘ └─────────┘    └─────────┘
```

**产出**: `{phase_num}-{N}-SUMMARY.md`, `{phase_num}-VERIFICATION.md`

#### 5. 工作验证 (`/gsd-verify-work N`)
- **提取可测试交付物**: 你现在应该能做什么
- **逐个引导测试**: "你能用邮箱登录吗？" 是/否，或描述问题
- **自动诊断失败**: 生成调试代理查找根本原因
- **创建验证修复计划**: 准备立即重新执行

**产出**: `{phase_num}-UAT.md`，如有问题则创建修复计划

### 快速模式 (`/gsd-quick`)
适用于不需要完整规划的临时任务：
- **相同代理**: 规划器 + 执行器，相同质量
- **跳过可选步骤**: 默认无研究、无计划检查、无验证
- **独立跟踪**: 位于 `.planning/quick/`
- **标志组合**:
  - `--discuss`: 轻量级讨论
  - `--research`: 计划前聚焦研究
  - `--full`: 启用所有阶段（讨论+研究+计划检查+验证）
  - `--validate`: 启用计划检查+执行后验证

## 系统架构

### 目录结构
```
get-shit-done/
├── agents/                    # 代理定义文件
│   ├── gsd-planner.md
│   ├── gsd-executor.md
│   └── ... (20+ agents)
├── commands/gsd/              # 命令定义
│   ├── new-project.md
│   ├── discuss-phase.md
│   └── ... (60+ commands)
├── get-shit-done/
│   ├── workflows/            # 工作流程定义
│   ├── references/           # 参考文档
│   ├── templates/            # 模板文件
│   └── contexts/             # 上下文配置文件
├── hooks/                    # 钩子脚本
├── scripts/                  # 构建和测试脚本
├── bin/install.js            # 安装器
└── docs/                     # 用户文档
```

### 代理合同系统
- **完成标记**: 每个代理有标准完成标记（如 `## PLANNING COMPLETE`）
- **交接合同**: 定义代理间数据传递格式
- **错误处理**: 标准化错误标记和异常处理

### 上下文预算规则
根据上下文窗口大小限制读取深度：

| 上下文窗口 | 子代理输出读取 | SUMMARY.md | VERIFICATION.md | PLAN.md（其他阶段） |
|-----------|--------------|------------|-----------------|-------------------|
| < 500k (200k 模型) | 仅前言 | 仅前言 | 仅前言 | 仅当前阶段 |
| >= 500k (1M 模型) | 允许完整正文 | 允许完整正文 | 允许完整正文 | 仅当前阶段 |

### 安全性设计
- **路径遍历防护**: 所有用户提供的文件路径验证在项目目录内解析
- **提示注入检测**: 集中安全模块扫描用户文本中的注入模式
- **安全 JSON 解析**: 捕获格式错误的参数
- **Shell 参数验证**: 用户文本在 Shell 插值前清理
- **敏感文件保护**: 支持通过 Claude Code deny list 保护机密文件

## 安装与配置

### 安装
```bash
npx get-shit-done-cc@latest
```

安装器提示选择：
1. **运行时**: Claude Code、OpenCode、Gemini、Kilo、Codex、Copilot 等（多选）
2. **位置**: 全局（所有项目）或本地（当前项目）

### 配置
GSD 将项目设置存储在 `.planning/config.json` 中：

#### 核心设置
| 设置 | 选项 | 默认 | 控制内容 |
|------|------|------|----------|
| `mode` | `yolo`, `interactive` | `interactive` | 自动批准 vs 每步确认 |
| `granularity` | `coarse`, `standard`, `fine` | `standard` | 阶段粒度（阶段 × 计划） |
| `project_code` | 字符串 | `""` | 阶段目录前缀项目代码 |

#### 模型配置文件
控制每个代理使用的 Claude 模型：

| 配置文件 | 规划 | 执行 | 验证 |
|---------|------|------|------|
| `quality` | Opus | Opus | Sonnet |
| `balanced` (默认) | Opus | Sonnet | Sonnet |
| `budget` | Sonnet | Sonnet | Haiku |
| `inherit` | 继承 | 继承 | 继承 |

切换配置文件：
```bash
/gsd-set-profile budget
```

#### 工作流代理
这些代理在计划/执行期间生成额外代理：

| 设置 | 默认 | 作用 |
|------|------|------|
| `workflow.research` | `true` | 每个阶段计划前研究领域 |
| `workflow.plan_check` | `true` | 执行前验证计划达到阶段目标 |
| `workflow.verifier` | `true` | 执行后确认必须交付物已交付 |
| `workflow.auto_advance` | `false` | 自动链式讨论 → 计划 → 执行 |
| `workflow.discuss_mode` | `'discuss'` | 讨论模式：`discuss`（访谈）, `assumptions`（代码库优先） |

## 命令参考

### 核心工作流命令
| 命令 | 作用 |
|------|------|
| `/gsd-new-project [--auto]` | 完整初始化：问题 → 研究 → 需求 → 路线图 |
| `/gsd-discuss-phase [N] [--auto] [--analyze] [--chain]` | 计划前捕获实现决策 |
| `/gsd-plan-phase [N] [--auto] [--reviews]` | 研究 + 计划 + 验证阶段 |
| `/gsd-execute-phase <N>` | 并行波浪执行所有计划，完成时验证 |
| `/gsd-verify-work [N]` | 手动用户验收测试 |
| `/gsd-ship [N] [--draft]` | 从已验证阶段工作创建 PR |
| `/gsd-next` | 自动推进到下一个逻辑工作流步骤 |
| `/gsd-quick <text>` | 临时任务 — 完全跳过规划，立即执行 |
| `/gsd-complete-milestone` | 归档里程碑，标记发布 |
| `/gsd-new-milestone [name]` | 开始下一个版本：问题 → 研究 → 需求 → 路线图 |

### 工作流管理
| 命令 | 作用 |
|------|------|
| `/gsd-workstreams list` | 显示所有工作流及其状态 |
| `/gsd-workstreams create <name>` | 创建命名空间工作流以进行并行里程碑工作 |
| `/gsd-workstreams switch <name>` | 切换活动工作流 |
| `/gsd-workstreams complete <name>` | 完成并合并工作流 |

### 代码质量
| 命令 | 作用 |
|------|------|
| `/gsd-review` | 当前阶段或分支的跨 AI 同行评审 |
| `/gsd-secure-phase [N]` | 安全强化，威胁模型锚定验证 |
| `/gsd-audit-uat` | 审计验证债务 — 查找缺少 UAT 的阶段 |

### 实用工具
| 命令 | 作用 |
|------|------|
| `/gsd-settings` | 配置模型配置文件和工作流代理 |
| `/gsd-help` | 显示所有命令和使用指南 |
| `/gsd-health [--repair]` | 验证 `.planning/` 目录完整性，`--repair` 自动修复 |
| `/gsd-stats` | 显示项目统计 — 阶段、计划、需求、Git 指标 |
| `/gsd-map-codebase [area]` | 新项目前分析现有代码库 |

## 技术特色

### 1. 上下文窗口感知提示精简
- 自动减少子 200K 模型的提示大小
- 动态调整读取深度和内容包含

### 2. 知识图谱集成 (`/gsd-graphify`)
- 为规划代理带来知识图谱
- 在项目工件之间建立丰富的上下文连接

### 3. TDD 管道模式
- 通过 `--tdd` 标志启用测试驱动开发工作流
- 集成到规划-执行-验证循环中

### 4. 项目技能感知
- 9 个 GSD 代理现在发现并使用项目范围的技能
- 技能作为 `<agent_skills>` 块注入到代理提示中

### 5. SDK 类型查询基础
- 基于注册表的 `gsd-sdk query` 命令
- 分类错误和单元测试处理程序
- 状态、路线图、阶段生命周期、配置的查询

### 6. 工作树安全
- 执行期间的工作树隔离
- 检测陈旧和孤立的工作树
- 子模块检测和跳过

## 最佳实践

### 1. 权限模式建议
GSD 设计用于无摩擦自动化，建议运行：
```bash
claude --dangerously-skip-permissions
```

或添加细粒度权限到 `.claude/settings.json`。

### 2. 现有项目工作流
对于现有（brownfield）项目：
1. 首先运行 `/gsd-map-codebase` 分析堆栈、架构、约定和关注点
2. 然后运行 `/gsd-new-project`，系统将了解你的代码库模式

### 3. 讨论模式选择
- **讨论模式** (`discuss`): 标准访谈风格，适合明确知道自己想要什么的用户
- **假设模式** (`assumptions`): 代码库优先，系统读取代码，展示它会做什么及原因，只要求纠正错误

### 4. Git 分支策略
在 `.planning/config.json` 中配置：
- **`none`**: 提交到当前分支（默认）
- **`phase`**: 每阶段创建分支，阶段完成时合并
- **`milestone`**: 整个里程碑创建一个分支，完成时合并

### 5. 性能监控
- **STATE.md** 中的性能指标：完成计划总数、平均持续时间、趋势
- **上下文使用监控**：分级警告系统
- **健康检查**：`.planning/` 目录完整性验证

## 故障排除

### 常见问题
1. **安装后命令未找到**
   - 重启运行时以重新加载命令/技能
   - 验证文件存在于 `~/.claude/skills/gsd-*/SKILL.md`

2. **更新到最新版本**
   ```bash
   npx get-shit-done-cc@latest
   ```

3. **卸载**
   ```bash
   npx get-shit-done-cc --claude --global --uninstall
   ```

### 恢复快速参考
- **会话恢复**: `/gsd-resume-work` 从上个会话恢复
- **状态检查**: `/gsd-progress` 查看当前位置和下一步
- **健康修复**: `/gsd-health --repair` 自动修复 `.planning/` 问题
- **工作流取证**: `/gsd-forensics [desc]` 失败工作流运行的事后调查

## 版本亮点 (v1.36.0)

### 新增功能
- **知识图谱集成** — `/gsd-graphify` 为规划代理带来知识图谱
- **SDK 类型查询基础** — 基于注册表的 `gsd-sdk query` 命令
- **TDD 管道模式** — 通过 `--tdd` 标志启用测试驱动开发工作流
- **上下文窗口感知提示精简** — 自动减少子 200K 模型的提示大小
- **项目技能感知** — 9 个 GSD 代理现在发现并使用项目范围的技能

### 改进
- **规划器上下文成本大小调整** — 用上下文成本大小和多源覆盖审计替换基于时间的推理
- **内联执行小计划** — 默认内联执行，跳过小计划的子代理开销
- **非技术所有者适配** — `discuss-phase` 通过 USER-PROFILE.md 为非技术所有者调整灰色区域语言

## 结论

GSD 是一个成熟的上下文工程和规格驱动开发系统，通过以下方式解决 AI 辅助编程的核心挑战：

1. **系统化上下文管理**：防止上下文劣化，保持输出质量
2. **专业代理分工**：20+ 种专业代理各司其职
3. **原子化工作流**：每个任务独立、可追溯、可验证
4. **灵活配置**：适应不同项目规模、团队偏好和预算
5. **多运行时支持**：覆盖主流 AI 辅助编程工具

系统特别适合：
- **独立开发者**：想要从想法到实现的可靠工作流
- **技术团队**：需要可重复、高质量 AI 辅助开发流程
- **现有项目维护**：通过代码库映射和 brownfield 支持
- **复杂系统构建**：通过阶段化路线图和依赖管理

GSD 的核心哲学是"将复杂性隐藏在系统中，而不是工作流中"，为用户提供简单命令背后强大的上下文工程能力。

---

*报告生成时间: 2026-04-17*  
*基于 GSD v1.36.0 分析*