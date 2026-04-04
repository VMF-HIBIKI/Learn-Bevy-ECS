# 模块五实现大任务：System、SystemParam、命令与事件

导航：
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/05-system-command-event.md)
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/05-system-command-event.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/05-system-command-event.md)

## 大任务总览

| 任务 ID | 名称 | 目标 |
| --- | --- | --- |
| M5-T1 | System 抽象 | 建立 `System` trait 与函数系统包装 |
| M5-T2 | SystemParam 管线 | 让函数参数能从 World 中抓取数据 |
| M5-T3 | Commands 与延迟应用 | 让结构性修改从系统执行中解耦 |
| M5-T4 | Event / Message / Observer 基线 | 建立事件驱动扩展路径 |
| M5-T5 | 独占系统与局部状态 | 支持 `&mut World` 系统和 `Local` 状态 |

## M5-T1 System 抽象

产出：

- `System` trait
- `IntoSystem`
- function system wrapper
- 系统初始化和运行接口

验收标准：

- 普通函数可以被加入调度并运行

## M5-T2 SystemParam 管线

产出：

- ParamState
- ParamFetch
- Query / Res / ResMut / Commands 基础参数

验收标准：

- 函数签名可以自动映射为参数抓取逻辑

## M5-T3 Commands 与延迟应用

产出：

- `Command`
- `CommandQueue`
- `Commands`
- `apply_deferred`

验收标准：

- 系统中发出的结构修改能在明确边界生效

## M5-T4 Event / Message / Observer 基线

产出：

- 类型化事件缓冲
- 事件读取与写入 API
- 观察者扩展预留接口

验收标准：

- 至少能完成“系统写事件、另一系统读事件”的闭环

## M5-T5 独占系统与局部状态

产出：

- exclusive system
- `Local<T>`
- one-shot / system state 入口

验收标准：

- 能编写直接接收 `&mut World` 的系统

## 参考 Bevy

- [`system/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/system/mod.rs)
- [`event/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/event/mod.rs)
