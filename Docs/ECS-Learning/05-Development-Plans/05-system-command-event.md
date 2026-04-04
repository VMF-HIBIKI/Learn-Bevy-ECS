# 模块五开发计划：System、SystemParam、命令与事件

导航：
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/05-system-command-event.md)
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/05-system-command-event.md)

## 推荐开发顺序

| 顺序 | 子任务 | 建议 Issue 标题 | 前置条件 | 测试重点 | 完成定义 |
| --- | --- | --- | --- | --- | --- |
| 1 | M5-S1 | `feat: define core system trait` | M3-S8 | System 初始化与运行协议 | System 抽象可用 |
| 2 | M5-S2 | `feat: wrap functions into systems` | M5-S1 | 普通函数转系统 | 基础系统可运行 |
| 3 | M5-S3 | `feat: define system param state pipeline` | M5-S1 | ParamState / ParamFetch | 参数框架稳定 |
| 4 | M5-S4 | `feat: implement resource system params` | M5-S3, M3-S4 | `Res` / `ResMut` | 资源参数可用 |
| 5 | M5-S5 | `feat: implement query system param` | M5-S3, M4-S8 | Query 作为参数 | 查询系统可运行 |
| 6 | M5-S6 | `feat: define deferred command queue` | M3-S10 | 命令缓冲生命周期 | CommandQueue 可用 |
| 7 | M5-S7 | `feat: add commands api for structural changes` | M5-S6 | spawn / insert / despawn commands | 系统中可提交命令 |
| 8 | M5-S8 | `feat: apply deferred commands in world` | M5-S7 | 刷新边界 | 命令可实际生效 |
| 9 | M5-S9 | `feat: add typed event buffers` | M3-S4 | writer / reader 闭环 | 事件系统可工作 |
| 10 | M5-S10 | `refactor: reserve observer extension seams` | M5-S9 | 扩展接口稳定性 | 事件模型可后续演进 |
| 11 | M5-S11 | `feat: support exclusive systems and local state` | M5-S2 | `&mut World` / `Local<T>` | 独占系统与局部状态可用 |

## 实施建议

- 一定先让 `Res`、`ResMut`、`Query` 这三类参数可用，再推进 Commands 与 Event。
- Commands 的正确性优先级高于 API 花样，因为它会直接影响存储安全性。
