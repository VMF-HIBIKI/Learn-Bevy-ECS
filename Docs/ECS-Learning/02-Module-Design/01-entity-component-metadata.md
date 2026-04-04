# 模块一：实体、组件与元数据

导航：
- [模块设计索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/README.md)
- [本模块实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/01-entity-component-metadata.md)
- [本模块可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/01-entity-component-metadata.md)
- [本模块开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/01-entity-component-metadata.md)

## 模块职责

这个模块负责 ECS 中所有“定义性的东西”：

- Entity 的表示、generation、防悬空语义
- Entity 分配器与回收策略
- Component 的注册、ComponentId、StorageType
- Bundle 的组件集合描述
- Required Components 与插入前扩展逻辑

如果这一层不稳定，后面所有存储、查询和调度都会建立在沙地上。

## 关键设计问题

### 1. Entity 如何编码

建议第一版直接采用“index + generation”的组合。原因：

- 可以检测旧句柄
- 回收实体后能复用 index
- 对 Archetype、SparseSet、命令队列都友好

### 2. 组件如何唯一标识

建议不要在运行时频繁以 `TypeId` 直接访问底层存储，而是通过注册表把 `TypeId` 映射到稳定的 `ComponentId`。这样后续：

- Query 可以缓存组件位集
- Archetype 可以使用紧凑 ID 排序
- Storage 层不必每次做动态查找

### 3. Bundle 是什么

Bundle 不是单纯“组件元组”，而是“一次插入操作的静态描述”。它的职责是：

- 提供组件集合
- 保证组件类型不重复
- 为插入路径准备排序后的组件清单

### 4. Required Components 要不要一开始就做

建议第一版先把“机制接口”设计好，再用最小实现落地。因为它会影响：

- Bundle 展开
- 插入顺序
- 生命周期钩子
- 组件元数据图

## 建议数据结构

- `Entity`
  - `index: u32`
  - `generation: NonZeroU32` 或等价方案
- `EntityLocation`
  - `archetype_id`
  - `table_row`
  - `archetype_row`
- `EntityMeta`
  - `generation`
  - `location`
  - `alive`
- `ComponentDescriptor`
  - `type_id`
  - `type_name`
  - `layout`
  - `drop_fn`
  - `storage_type`
- `ComponentInfo`
  - `component_id`
  - `descriptor`
  - `required_components`
- `BundleInfo`
  - 排序后的组件 ID 列表
  - 组件写入函数表

## 模块接口建议

- `World::spawn(bundle)`
- `World::despawn(entity)`
- `World::register_component::<T>()`
- `World::component_id::<T>() -> Option<ComponentId>`
- `World::contains(entity) -> bool`
- `World::entity(entity) -> EntityRef`

## 关键不变量

- 一个活着的实体必须有且仅有一个有效位置。
- 一个 `ComponentId` 对应一个稳定的组件元信息。
- 一个 Bundle 展开后不允许出现重复组件。
- Required Components 的扩展结果必须是确定性的。

## 与其他模块的耦合点

- 向模块二提供 Archetype 迁移所需的组件集合描述。
- 向模块三提供实体位置索引。
- 向模块四提供 Query 所需的 `ComponentId` 语义。
- 向模块五提供 Commands / Events 处理所需的实体合法性判断。

## 参考 Bevy 源码

- [`entity/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/entity/mod.rs)
- [`component/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/component/mod.rs)
- [`bundle`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/bundle)

## 设计取舍

- 第一版可以先不追求极致压缩编码，优先让生命周期清晰。
- 第一版可以不做跨 World 的 entity mapping，但要预留转换接口。
- 第一版 Required Components 可以不做复杂循环检测，但文档中必须写清风险。
