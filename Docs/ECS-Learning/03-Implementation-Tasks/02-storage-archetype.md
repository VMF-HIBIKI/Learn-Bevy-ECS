# 模块二实现大任务：存储、Archetype 与布局迁移

导航：
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/02-storage-archetype.md)
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/02-storage-archetype.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/02-storage-archetype.md)

## 大任务总览

| 任务 ID | 名称 | 目标 |
| --- | --- | --- |
| M2-T1 | Table 存储 | 建立列式组件存储和表行管理 |
| M2-T2 | SparseSet 存储 | 支持稀疏组件的高效增删查 |
| M2-T3 | Archetype 注册表 | 维护组件集合到 Archetype 的映射 |
| M2-T4 | 布局迁移引擎 | 支持 add / remove / insert bundle 的迁移 |
| M2-T5 | 批量路径与清理 | 支持 batch spawn、swap-remove 与回收 |

## M2-T1 Table 存储

产出：

- 组件列容器
- 表行分配与删除
- 实体到表行的映射协议

验收标准：

- 同 Archetype 实体的表存储组件能连续存放
- 行删除后受影响实体的位置能被修正

## M2-T2 SparseSet 存储

产出：

- sparse index
- dense entity/value 数组
- 插入、更新、删除与查询接口

验收标准：

- 稀疏组件能独立于 Table 存放
- 删除后索引仍然一致

## M2-T3 Archetype 注册表

产出：

- `Archetype`
- 组件集合签名
- `Archetypes` 集合
- add/remove edge 缓存

验收标准：

- 相同组件集合总能命中同一 Archetype
- 组件集合有稳定排序与查找策略

## M2-T4 布局迁移引擎

产出：

- 从源 Archetype 迁移到目标 Archetype 的流程
- Table 组件移动
- SparseSet 组件更新
- entity location 修正

验收标准：

- 添加组件后实体进入正确 Archetype
- 删除组件后旧值正确 drop
- 迁移不产生悬空位置

## M2-T5 批量路径与清理

产出：

- batch spawn
- swap-remove 帮助函数
- despawn 清理流程
- 位置修正与释放策略

验收标准：

- 批量插入比单个插入路径更简单清晰
- despawn 后 Table、SparseSet、Archetype、EntityMeta 全部一致

## 风险与边界

- 第一次实现时最容易错在“移动了值但没修位置”。
- 这一层的测试必须覆盖 swap-remove 影响到的“尾元素被搬家”场景。

## 参考 Bevy

- [`storage/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/storage/mod.rs)
- [`archetype`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/archetype)
