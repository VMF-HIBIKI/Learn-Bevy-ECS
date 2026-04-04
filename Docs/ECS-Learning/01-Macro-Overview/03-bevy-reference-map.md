# Bevy 参考映射

导航：
- [第一层索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/01-Macro-Overview/README.md)
- [第二层模块设计索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/README.md)

## 我们为什么要做映射

直接按 Bevy 源码目录学习，容易“看见很多模块，但不知道学习顺序”。这份映射的目标是把 Bevy 的工程化结构，压成适合自研 ECS 的学习路径。

## Bevy 一级模块到学习模块的映射

| Bevy 模块 | 主要职责 | 映射到我们的模块 |
| --- | --- | --- |
| `entity` | 实体 ID、分配、生命周期 | 模块一 |
| `component` | 组件声明、注册、存储类型、required components | 模块一 |
| `bundle` | 组件组合插入协议 | 模块一 / 模块二 |
| `storage` | Table、SparseSet、Resource storage | 模块二 / 模块三 |
| `archetype` | 组件集合布局与迁移目标 | 模块二 |
| `world` | ECS 外观、资源接口、实体访问入口 | 模块三 |
| `query` | 访问分析、过滤器、迭代器、缓存 | 模块四 |
| `change_detection` | tick、Ref、Mut、变化判定 | 模块四 |
| `lifecycle` | add / insert / remove / despawn 跟踪 | 模块四 / 模块五 |
| `system` | 系统抽象、SystemParam、Commands | 模块五 |
| `event` | 事件类型与事件触发模型 | 模块五 |
| `message` | 更底层消息能力 | 模块五 |
| `observer` | 观察者响应式扩展 | 模块五 |
| `schedule` | 图构建、执行器、集合、条件 | 模块六 |
| `resource` | 资源标记与语义 | 模块三 |
| `spawn` | 批量 spawn 辅助与关系生成 | 模块二 / 模块三 |
| `relationship` | 关系建模 | 模块五的扩展能力 |
| `hierarchy` | 父子层级关系 | 模块五的扩展能力 |
| `entity_disabling` | 查询默认过滤能力 | 模块四 / 模块六 |
| `reflect` | 反射适配 | 后续扩展，不作为首批必做 |
| `name` | 调试友好命名 | 模块六的开发体验扩展 |

## 首批必做能力

我们第一轮实现必须覆盖：

- Entity 与 generation
- Component 注册与 Bundle 插入
- Table / SparseSet / Archetype
- World 与 Resource
- Query / Filter / Change Detection
- Function System / SystemParam / Commands
- Schedule / Executor / Ordering / Conditions

## 第二轮扩展能力

这些不是首批阻塞项，但应当在文档中预留接口：

- Events / Messages / Observers
- Relationship / Hierarchy
- Diagnostic naming / tracing
- Reflection / dynamic world tooling

## 重点源码入口

- ECS 总入口：[`lib.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/lib.rs)
- 实体：[`entity/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/entity/mod.rs)
- 组件：[`component/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/component/mod.rs)
- 存储：[`storage/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/storage/mod.rs)
- World：[`world/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/world/mod.rs)
- Query：[`query/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/query/mod.rs)
- System：[`system/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/system/mod.rs)
- Schedule：[`schedule/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/schedule/mod.rs)
- Event：[`event/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/event/mod.rs)

## 下一步

- 看 [学习与实现路线图](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/01-Macro-Overview/04-learning-roadmap.md)，把这些模块放回真正的工程顺序。
