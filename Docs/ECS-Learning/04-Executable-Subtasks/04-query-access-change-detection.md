# 模块四可执行子任务：Query、访问分析与变更检测

导航：
- [第三层实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/04-query-access-change-detection.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/04-query-access-change-detection.md)

## 子任务表

| 子任务 ID | 对应大任务 | 子任务内容 | 完成标志 |
| --- | --- | --- | --- |
| M4-S1 | M4-T1 | 定义组件/资源读写访问集合 | 可表达冲突分析所需访问语义 |
| M4-S2 | M4-T1 | 定义过滤访问模型 | 带过滤器的访问能独立表示 |
| M4-S3 | M4-T2 | 定义 QueryData / QueryFilter 描述结构 | 查询声明有统一内部表示 |
| M4-S4 | M4-T2 | 实现 QueryState 编译与缓存 | Query 可重复使用编译结果 |
| M4-S5 | M4-T2 | 实现 Archetype 匹配缓存刷新 | 世界布局变化后查询可更新 |
| M4-S6 | M4-T3 | 实现 `&T` 读取 fetch | 只读组件查询可工作 |
| M4-S7 | M4-T3 | 实现 `&mut T` 写入 fetch | 可变组件查询可工作 |
| M4-S8 | M4-T3 | 实现 `(Entity, &T)` 与 tuple fetch | 常见组合查询可工作 |
| M4-S9 | M4-T4 | 实现 `With<T>` / `Without<T>` | 基本过滤器生效 |
| M4-S10 | M4-T4 | 实现 `Option<&T>` | 可选组件访问可工作 |
| M4-S11 | M4-T5 | 实现 `Tick` 与 `ComponentTicks` | 组件/资源变化可被记录 |
| M4-S12 | M4-T5 | 实现 `Added<T>` / `Changed<T>` | 变化过滤器可工作 |
| M4-S13 | M4-T5 | 实现 removed tracking 初版 | 移除事件能被后续系统观察 |

## 执行提示

- M4-S1 到 M4-S5 的本质是“查询编译器”。
- M4-S6 到 M4-S10 的本质是“抓取器与过滤器”。
- M4-S11 到 M4-S13 的本质是“时间线与生命周期跟踪”。
