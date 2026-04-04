# 模块三开发计划：World、资源与访问外观

导航：
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/03-world-resource-access.md)
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/03-world-resource-access.md)

## 推荐开发顺序

| 顺序 | 子任务 | 建议 Issue 标题 | 前置条件 | 测试重点 | 完成定义 |
| --- | --- | --- | --- | --- | --- |
| 1 | M3-S1 | `feat: define world core layout` | M1-S8, M2-S7 | World 创建 | World 可初始化 |
| 2 | M3-S2 | `feat: bootstrap world internal state` | M3-S1 | 默认状态一致性 | 新 World 具备最小可用能力 |
| 3 | M3-S3 | `feat: define resource storage container` | M1-S7 | 类型到资源映射 | 资源容器结构完整 |
| 4 | M3-S4 | `feat: add resource world apis` | M3-S3 | insert/get/remove/init | 资源 API 闭环完成 |
| 5 | M3-S5 | `feat: expose entity lifecycle apis on world` | M3-S1, M2-S9 | spawn / despawn / contains | 高层实体 API 可用 |
| 6 | M3-S6 | `feat: expose component read write apis on world` | M3-S5 | get / get_mut | 高层组件 API 可用 |
| 7 | M3-S7 | `feat: add entity reference views` | M3-S6 | EntityRef / EntityMut | 局部访问视图可用 |
| 8 | M3-S8 | `refactor: introduce unsafe world split access` | M3-S7 | 不安全访问边界 | Query / System 的底层访问基座建立 |
| 9 | M3-S9 | `feat: track world change ticks` | M3-S1 | tick 推进与读取 | 时间线基础稳定 |
| 10 | M3-S10 | `feat: add world flush and deferred hooks` | M3-S9 | 命令刷新边界 | Commands / Schedule 可接入 |
| 11 | M3-S11 | `test: add world integration smoke tests` | M3-S10 | 资源、实体、组件联合场景 | World 基础闭环稳定 |

## 实施建议

- 先把 World 做成“可靠容器”，再把它变成“聪明的 API 门面”。
- `UnsafeWorldCell` 或等价能力一定要有明确的 unsafe 说明，否则后面 Query 层会很难维护。
