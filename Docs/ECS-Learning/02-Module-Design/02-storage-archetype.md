# 模块二：存储、Archetype 与布局迁移

导航：
- [模块设计索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/README.md)
- [本模块实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/02-storage-archetype.md)
- [本模块可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/02-storage-archetype.md)
- [本模块开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/02-storage-archetype.md)

## 模块职责

这个模块负责“数据实际放在哪里，以及实体改变组件集合时如何迁移”。它决定 ECS 是否真的具备数据导向的性能基础。

## 核心组成

### Table

适合高密度、批量遍历的组件。典型特点：

- 每个组件一列
- 相同 Archetype 的实体共享同一张表
- 批量扫描 cache 友好

### SparseSet

适合频繁插入/删除或稀疏存在的组件。典型特点：

- 随机访问快
- 插入删除成本低
- 批量顺序遍历一般不如 Table

### Archetype

Archetype 不是存储本身，而是“组件集合签名 + 实体分组”。它负责回答：

- 拥有同样组件集合的实体在哪一组
- 这组实体对应哪张表
- 当新增或移除组件时，目标 Archetype 是谁

## 最关键的设计判断

### 1. Archetype 是否包含 SparseSet 组件

建议是。即使某些组件值实际存放在 SparseSet 中，Archetype 仍应记录“这个实体逻辑上拥有哪些组件”，否则 Query 过滤会非常混乱。

### 2. 添加组件时发生什么

不是“往原地塞一个字段”，而是：

- 找到新组件集合对应的目标 Archetype
- 把 Table 组件迁移到新表行
- 更新 SparseSet 组件索引
- 更新 EntityLocation

### 3. 删除组件时发生什么

删除是一次布局迁移，而不是简单标记。需要明确：

- 被删组件的 drop 时机
- swap-remove 后受影响实体的位置修正
- 生命周期事件的顺序

## 建议数据结构

- `Tables`
- `Table`
  - columns
  - entities
- `SparseSets`
- `SparseSet`
  - sparse indices
  - dense entities
  - dense values
- `Archetype`
  - component set
  - table id
  - entities
  - edges for add/remove transitions
- `Archetypes`
  - archetype registry
  - lookup by sorted component set

## 关键不变量

- Table 中每一行必须和实体位置表一致。
- Archetype 的组件集合必须是排序且去重后的稳定表示。
- 迁移后源位置与目标位置都必须同步修正。
- 对 Table 组件和 SparseSet 组件的 drop 语义不能混乱。

## 与其他模块的关系

- 依赖模块一提供 ComponentId、BundleInfo、EntityLocation。
- 向模块三提供 `spawn/get/remove` 的底层实现。
- 向模块四提供 Query 遍历候选集。

## 参考 Bevy 源码

- [`storage/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/storage/mod.rs)
- [`archetype`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/archetype)
- [`spawn`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/spawn)

## 第一版建议

- 先实现 Table-only 的一条细路径，再补 SparseSet。
- 先保证布局迁移正确，再考虑批量优化。
- 所有移动路径都先写位置一致性测试。
