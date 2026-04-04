# 模块三：World、资源与访问外观

导航：
- [模块设计索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/README.md)
- [本模块实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/03-world-resource-access.md)
- [本模块可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/03-world-resource-access.md)
- [本模块开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/03-world-resource-access.md)

## 模块职责

World 是 ECS 的总外观。它不是一个简单容器，而是：

- 存放所有实体、组件、资源和调度相关状态的总入口
- 隔离底层存储复杂度的 API 门面
- 借用规则与运行时状态的集中协调点

## World 应当拥有什么

- 实体元数据与分配器
- 组件注册表
- Archetype 集合
- 存储集合
- Bundle 缓存
- 资源存储
- 变更 tick
- 命令队列
- 事件 / 观察者注册表

## Resource 语义

资源是“按类型唯一”的全局状态。它与组件的区别不在数据结构，而在生命周期与寻址方式：

- 组件附着在 Entity 上
- Resource 直接附着在 World 上

首批实现至少要覆盖：

- `insert_resource`
- `get_resource`
- `get_resource_mut`
- `init_resource`
- `remove_resource`

## 访问外观设计

World 不能把所有内部结构都原样暴露，否则你后面很难维持不变量。建议分三层：

### 1. 高层便捷 API

面向使用者：

- `spawn`
- `despawn`
- `get`
- `get_mut`
- `query`

### 2. 中层引用视图

面向局部访问：

- `EntityRef`
- `EntityMut`
- `EntityWorldMut`

### 3. 底层不安全视图

面向 Query / System 实现：

- `UnsafeWorldCell` 或等价设计

## 为什么要设计 UnsafeWorldCell

因为 Query 和 SystemParam 需要在“逻辑安全”的前提下拆分 World 的多个子借用，而 Rust 的静态借用往往无法直接表达这种运行时已证明安全的访问拆分。

## 关键不变量

- 任何高层 API 都不能绕过底层位置一致性约束。
- Resource 的唯一性必须按类型保证。
- Unsafe 层只能作为底层实现细节存在，不能变成随处可用的逃生门。

## 参考 Bevy 源码

- [`world/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/world/mod.rs)
- [`resource`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/resource)
- [`unsafe_world_cell`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/world/unsafe_world_cell.rs)

## 第一版建议

- 先把 World 做成单线程正确版本。
- 先给 Query / SystemParam 预留访问视图接口，再逐渐引入 unsafe 优化。
- 所有 `get_mut` 路径都先写借用与位置一致性测试。
