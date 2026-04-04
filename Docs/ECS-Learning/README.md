# ECS Learning

这套文档的目标不是“介绍一下 ECS”，而是把“参考 Bevy 的 ECS 内核，逐步实现一套属于自己的 ECS 框架”拆成可学习、可执行、可验证的长期工程。

## 导航索引

- 第一层：宏观理解与术语地图
  - [总览索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/01-Macro-Overview/README.md)
- 第二层：模块设计
  - [模块设计索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/README.md)
- 第三层：模块实现大任务
  - [实现大任务索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/README.md)
- 第四层：可执行子任务
  - [可执行子任务索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/README.md)
- 第五层：子任务开发计划
  - [开发计划索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/README.md)

## 六个核心模块

- 模块一：实体、组件与元数据
- 模块二：存储、Archetype 与布局迁移
- 模块三：World、资源与访问外观
- 模块四：Query、访问分析与变更检测
- 模块五：System、SystemParam、命令与事件
- 模块六：Schedule、执行器、并行与工程化

## 如何使用这套文档

1. 先读第一层，建立 ECS 的完整心智模型。
2. 再读第二层，明确我们自己的 ECS 将由哪些模块组成。
3. 做实现时只看第三层和第四层，把大问题压成单次可完成的任务。
4. 开工前对照第五层，先写测试与验收标准，再开始编码。
5. 每次只推进一个子任务，保持“一次只啃一个核心机制”的节奏。

## 参考基线

- Bevy ECS 入口：[`bevy_ecs/lib.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/lib.rs)
- Bevy ECS 概览：[`bevy_ecs/README.md`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/README.md)

## 目标产出

- 形成一套完整的 ECS 学习图谱，而不是零散笔记。
- 为后续 `src/` 中的实现提供稳定的设计约束。
- 让每个 Issue、每个 Commit 都能明确对应到一个具体的学习与实现目标。
