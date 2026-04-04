# 模块四开发计划：Query、访问分析与变更检测

导航：
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/04-query-access-change-detection.md)
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/04-query-access-change-detection.md)

## 推荐开发顺序

| 顺序 | 子任务 | 建议 Issue 标题 | 前置条件 | 测试重点 | 完成定义 |
| --- | --- | --- | --- | --- | --- |
| 1 | M4-S1 | `feat: define ecs access set model` | M1-S8 | 读写冲突判定 | 访问集合可比较 |
| 2 | M4-S2 | `feat: add filtered access model` | M4-S1 | 过滤后访问表达 | FilteredAccess 可用 |
| 3 | M4-S3 | `feat: define query descriptor traits` | M3-S8 | QueryData / Filter 统一描述 | 查询内部协议稳定 |
| 4 | M4-S4 | `feat: compile query state from world metadata` | M4-S3 | QueryState 缓存 | 重复查询不重复编译 |
| 5 | M4-S5 | `feat: refresh query archetype matches` | M4-S4, M2-S7 | 世界布局变化后的刷新 | Query 缓存能更新 |
| 6 | M4-S6 | `feat: fetch shared component references` | M4-S4, M2-S9 | `&T` 查询 | 只读查询可运行 |
| 7 | M4-S7 | `feat: fetch mutable component references` | M4-S6 | `&mut T` 查询 | 写访问路径正确 |
| 8 | M4-S8 | `feat: support entity and tuple query fetch` | M4-S7 | `(Entity, &T)`、tuple | 常见组合查询可用 |
| 9 | M4-S9 | `feat: add with and without query filters` | M4-S8 | Archetype 过滤 | 基础过滤器稳定 |
| 10 | M4-S10 | `feat: support optional component fetch` | M4-S8 | `Option<&T>` | 可选查询路径可用 |
| 11 | M4-S11 | `feat: track component change ticks` | M3-S9 | 写入打 tick | 变化记录存在 |
| 12 | M4-S12 | `feat: add added and changed filters` | M4-S11 | `Added` / `Changed` | 变化过滤器生效 |
| 13 | M4-S13 | `feat: track removed components` | M4-S12 | remove 记录与消费 | removed tracking 初版完成 |

## 实施建议

- Query 的前半段是在做“编译器”，后半段是在做“执行器”，两部分不要混写。
- 变更检测一定要建立在稳定的 tick 模型之上，不要在还没有统一 tick 语义时急着做 `Changed<T>`。
