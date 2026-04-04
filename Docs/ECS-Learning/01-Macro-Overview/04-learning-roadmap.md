# 学习与实现路线图

导航：
- [第一层索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/01-Macro-Overview/README.md)
- [第三层实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/README.md)

## 总体阶段

### 阶段 0：术语和边界先统一

产出：

- 明确 Entity / Component / Resource / Bundle / Archetype / Query / System / Schedule 的语义
- 建立六大模块边界
- 明确第一版不做什么

### 阶段 1：最小可运行内核

目标是让 ECS 先“活起来”：

- Entity 分配与回收
- Component 注册
- World 存储资源
- 最简单的 spawn / get / get_mut
- 单线程 Schedule 跑函数系统

### 阶段 2：Archetype 化

目标是让 ECS 不再只是泛型容器，而是进入真正的数据布局阶段：

- 引入 Table 与 SparseSet
- 引入 Archetype
- 支持 Bundle 插入和组件迁移
- 让 Query 能批量遍历

### 阶段 3：访问分析与系统参数

目标是让 API 进入“能写出正常 ECS 系统”的状态：

- QueryState
- Access 集合
- SystemParam
- Commands 与 deferred apply

### 阶段 4：调度与并行

目标是让系统执行具备工程化能力：

- before / after
- set / condition
- executor
- 冲突分析与歧义检测

### 阶段 5：高级机制

目标是让框架逐渐接近真实引擎级 ECS：

- Change Detection
- Events / Messages / Observers
- Hooks / lifecycle tracking
- Diagnostics / tracing / naming

## 每一阶段的学习原则

- 先实现最小闭环，再扩展能力。
- 每增加一个概念，都要补测试与文档。
- 任何“看起来能用”的临时代码，都要问清楚是否会阻塞后面的 Archetype、Query 和 Schedule。

## 推荐实践节奏

1. 先从第三层选一个大任务。
2. 到第四层确认它被拆成哪些可执行动作。
3. 到第五层按子任务开发计划推进。
4. 写完后回到第二层，确认设计是否需要修正。

## 下一步

- 进入 [第二层模块设计索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/README.md)。
