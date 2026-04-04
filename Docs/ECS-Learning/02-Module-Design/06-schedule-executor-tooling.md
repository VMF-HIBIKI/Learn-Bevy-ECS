# 模块六：Schedule、执行器、并行与工程化

导航：
- [模块设计索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/README.md)
- [本模块实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/06-schedule-executor-tooling.md)
- [本模块可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/06-schedule-executor-tooling.md)
- [本模块开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/06-schedule-executor-tooling.md)

## 模块职责

前五个模块解决“系统是什么、数据是什么、怎么访问”。这个模块解决“它们如何按规则运行起来”。

## Schedule 应该承担什么

- 收集系统
- 记录显式顺序
- 管理 SystemSet
- 处理 run condition
- 生成执行图
- 选择执行器
- 在命令刷新点插入 apply deferred
- 产出冲突、歧义和执行诊断

## 执行器分层建议

### 第一版：串行执行器

目的不是偷懒，而是先验证：

- System 初始化是否正确
- 条件判断是否稳定
- 命令刷新边界是否清晰

### 第二版：并行规划器

在获得访问集合后，再做：

- 读写冲突判定
- 批次分组
- worker 提交
- 依赖边协调

## 工程化能力为什么属于这一层

因为很多“体验层能力”本质上依赖调度层输出：

- 系统名与调试信息
- 歧义报告
- 性能基准与 tracing
- schedule build pass
- 测试环境下的 deterministic 执行

## 关键不变量

- 显式顺序必须强于默认并行。
- 不兼容访问不能在同一并行批次内执行。
- 条件系统不能偷偷修改不允许修改的数据。
- 一个 schedule 的构建结果在组件注册稳定后应可缓存。

## 与其他模块关系

- 依赖模块四提供访问集合
- 依赖模块五提供 System 抽象
- 依赖模块三提供 World
- 反过来决定 Commands / Events / Change Detection 的刷新节奏

## 参考 Bevy 源码

- [`schedule/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/schedule/mod.rs)
- [`schedule/graph.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/schedule/graph.rs)
- [`schedule/executor`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/schedule/executor)

## 第一版建议

- 先做 schedule graph 的正确性，再做 executor 的性能。
- 先把歧义检测做出来，它会反过来帮助你验证 Query 和 SystemParam 的访问声明是否正确。
