# 模块三可执行子任务：World、资源与访问外观

导航：
- [第三层实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/03-world-resource-access.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/03-world-resource-access.md)

## 子任务表

| 子任务 ID | 对应大任务 | 子任务内容 | 完成标志 |
| --- | --- | --- | --- |
| M3-S1 | M3-T1 | 定义 `World` 字段布局与默认构造 | World 可被创建并持有核心子系统 |
| M3-S2 | M3-T1 | 实现 bootstrap 与默认元数据初始化 | World 启动即具备最小工作状态 |
| M3-S3 | M3-T2 | 定义资源存储容器 | 资源按类型索引存放 |
| M3-S4 | M3-T2 | 实现资源插入、读取、可变读取、删除 | Resource API 闭环完成 |
| M3-S5 | M3-T3 | 暴露 spawn / despawn / contains / entity | 高层实体 API 可用 |
| M3-S6 | M3-T3 | 暴露 get / get_mut 组件读取路径 | 组件高层访问可用 |
| M3-S7 | M3-T4 | 定义 `EntityRef` / `EntityMut` | 局部实体访问视图可用 |
| M3-S8 | M3-T4 | 设计 `UnsafeWorldCell` 或等价能力 | Query / System 的底层访问可承载 |
| M3-S9 | M3-T5 | 实现 change tick 推进 | 世界变更时间线可维护 |
| M3-S10 | M3-T5 | 实现 flush / apply_deferred 钩子 | World 能配合 Commands 与 Schedule |
| M3-S11 | M3-T5 | 编写 World 集成烟雾测试 | 资源、实体、组件 API 能联合工作 |

## 执行提示

- M3-S1 到 M3-S4 优先做“World 作为容器”。
- M3-S7 到 M3-S8 是后续 Query / SystemParam 的桥，一定要留出清晰边界。
