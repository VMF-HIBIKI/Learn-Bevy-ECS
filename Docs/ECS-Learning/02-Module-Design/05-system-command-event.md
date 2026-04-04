# 模块五：System、SystemParam、命令与事件

导航：
- [模块设计索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/README.md)
- [本模块实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/05-system-command-event.md)
- [本模块可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/05-system-command-event.md)
- [本模块开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/05-system-command-event.md)

## 模块职责

这个模块负责把“普通 Rust 函数”提升为“可被调度、可声明访问、可复用状态”的运行时行为单元。

## System 层要覆盖的核心能力

### 1. Function System

把函数签名转成一个实现 `System` trait 的对象，并记录：

- 输入输出类型
- 初始化逻辑
- 访问集合
- 上次运行 tick

### 2. SystemParam

SystemParam 是连接 World 与系统函数参数的桥梁。没有它，ECS 只是“数据容器”，很难形成良好的使用体验。

### 3. Commands

当系统遍历 Query 时，往往不能立刻修改 World 的结构性状态，因此需要命令队列：

- spawn
- insert
- remove
- despawn
- insert_resource

### 4. Events / Messages / Observers

这部分是从“纯拉式查询”走向“事件驱动扩展”的关键能力。建议先有统一抽象，再分层落地：

- Events：有类型的广播
- Messages：更底层的消息存储
- Observers：响应式回调机制

## 设计要点

### System 不是函数指针

它必须是有内部状态的：

- 维护局部缓存
- 记录 `last_run_tick`
- 缓存 QueryState 和 ParamState

### Commands 不是便捷 API

它是为了把“结构修改”和“系统执行”解耦。否则 Query 迭代期的结构变化会让存储与借用都变得危险。

### Event 不应破坏 World 一致性

第一版事件系统建议先做“类型化缓冲 + 显式读取”，观察者与即时触发可以后置。

## 建议接口

- `IntoSystem`
- `System`
- `SystemParam`
- `SystemState`
- `Commands`
- `CommandQueue`
- `EventWriter` / `EventReader` 或自定义等价接口

## 参考 Bevy 源码

- [`system/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/system/mod.rs)
- [`event/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/event/mod.rs)
- [`message`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/message)
- [`observer`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/observer)

## 第一版建议

- 先做 Query / Res / ResMut / Commands 这几类高频参数。
- 先做 deferred command，再做 observer。
- 事件系统先追求清晰可测，不先追求最强功能面。
