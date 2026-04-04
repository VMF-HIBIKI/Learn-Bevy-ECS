# 模块三实现大任务：World、资源与访问外观

导航：
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/03-world-resource-access.md)
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/03-world-resource-access.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/03-world-resource-access.md)

## 大任务总览

| 任务 ID | 名称 | 目标 |
| --- | --- | --- |
| M3-T1 | World 内部骨架 | 组织 ECS 内部子系统的总容器 |
| M3-T2 | Resource 存储接口 | 提供按类型唯一的资源能力 |
| M3-T3 | 高层访问 API | 提供 spawn / get / entity 这类门面 |
| M3-T4 | 引用视图与 Unsafe 访问层 | 为 Query / SystemParam 提供底层借用能力 |
| M3-T5 | World 维护机制 | 管理 tick、flush、bootstrap、初始化流程 |

## M3-T1 World 内部骨架

产出：

- `World` 结构
- 内部子模块字段布局
- 默认初始化与 bootstrap 流程

验收标准：

- World 创建后即可注册组件、插入资源、spawn 实体

## M3-T2 Resource 存储接口

产出：

- `insert_resource`
- `get_resource`
- `get_resource_mut`
- `remove_resource`
- `init_resource`

验收标准：

- 一个类型同时只能存在一个资源实例
- 资源也有变更追踪入口

## M3-T3 高层访问 API

产出：

- `spawn`
- `despawn`
- `contains`
- `get`
- `get_mut`
- `entity`

验收标准：

- 使用者无需理解底层 Archetype 也能使用 ECS
- 非法实体访问能得到清晰错误或 `None`

## M3-T4 引用视图与 Unsafe 访问层

产出：

- `EntityRef`
- `EntityMut`
- `EntityWorldMut`
- `UnsafeWorldCell` 或等价能力

验收标准：

- Query / System 层可以在不暴露整个 `&mut World` 的前提下工作

## M3-T5 World 维护机制

产出：

- change tick 更新
- 命令刷新接口
- 生命周期引导逻辑
- 调度运行所需公共入口

验收标准：

- System 连续运行时 tick 正确推进
- 命令应用边界明确

## 参考 Bevy

- [`world/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/world/mod.rs)
