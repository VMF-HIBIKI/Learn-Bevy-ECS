# 模块五可执行子任务：System、SystemParam、命令与事件

导航：
- [第三层实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/05-system-command-event.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/05-system-command-event.md)

## 子任务表

| 子任务 ID | 对应大任务 | 子任务内容 | 完成标志 |
| --- | --- | --- | --- |
| M5-S1 | M5-T1 | 定义 `System` trait 与系统生命周期接口 | System 抽象稳定 |
| M5-S2 | M5-T1 | 实现函数到系统的包装器 | 普通函数可转为系统 |
| M5-S3 | M5-T2 | 定义 `SystemParam` 状态与抓取协议 | 参数系统有统一协议 |
| M5-S4 | M5-T2 | 实现 `Res` / `ResMut` 参数 | 资源参数可工作 |
| M5-S5 | M5-T2 | 实现 `Query` 参数 | 查询参数可在系统中使用 |
| M5-S6 | M5-T3 | 定义 `Command` 与 `CommandQueue` | 命令缓冲结构稳定 |
| M5-S7 | M5-T3 | 实现 `Commands` API | 系统可提交结构变更命令 |
| M5-S8 | M5-T3 | 集成 `apply_deferred` 刷新路径 | 命令能在边界被执行 |
| M5-S9 | M5-T4 | 实现事件存储与 writer/reader | 系统间事件传递可用 |
| M5-S10 | M5-T4 | 预留 observer / message 扩展缝隙 | 高级事件模型有演进路径 |
| M5-S11 | M5-T5 | 实现 exclusive system 与 `Local<T>` | 独占系统与局部状态可用 |

## 执行提示

- 在 M5-S5 完成前，不要急着做复杂 Schedule。
- 命令队列必须和 Query 迭代解耦，不要允许系统在迭代过程中直接破坏存储结构。
