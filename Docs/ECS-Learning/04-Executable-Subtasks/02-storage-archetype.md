# 模块二可执行子任务：存储、Archetype 与布局迁移

导航：
- [第三层实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/02-storage-archetype.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/02-storage-archetype.md)

## 子任务表

| 子任务 ID | 对应大任务 | 子任务内容 | 完成标志 |
| --- | --- | --- | --- |
| M2-S1 | M2-T1 | 定义 Table、Column、TableRow 基本结构 | 表结构可容纳列式组件 |
| M2-S2 | M2-T1 | 实现列写入、读取、drop 流程 | 单列可安全插入与销毁 |
| M2-S3 | M2-T1 | 实现表行插入与 swap-remove 删除 | 表行迁移后位置可修正 |
| M2-S4 | M2-T2 | 定义 SparseSet 内部布局 | sparse / dense 结构齐全 |
| M2-S5 | M2-T2 | 实现 SparseSet 插入、更新、删除 | 稀疏组件访问正确 |
| M2-S6 | M2-T3 | 定义 Archetype 组件签名表示 | 组件集合可稳定比较 |
| M2-S7 | M2-T3 | 实现 Archetype 注册表与查找 | 相同签名映射同一 Archetype |
| M2-S8 | M2-T3 | 实现 add/remove edge 缓存 | Archetype 迁移可快速命中目标 |
| M2-S9 | M2-T4 | 打通 table-only 的 spawn 流程 | 表存储组件能完整落盘 |
| M2-S10 | M2-T4 | 实现 add bundle 迁移流程 | 新组件插入后进入正确布局 |
| M2-S11 | M2-T4 | 实现 remove component 迁移流程 | 删除组件后旧值被正确清理 |
| M2-S12 | M2-T5 | 实现 despawn 全路径清理 | 所有存储与位置同步回收 |
| M2-S13 | M2-T5 | 实现 batch spawn 初版 | 批量创建实体可用且正确 |

## 执行提示

- 先让 Table-only 路径跑通，再补 SparseSet。
- 每写一个迁移函数，都要补“源 Archetype、目标 Archetype、实体位置、表行、稀疏索引”五件事的一致性测试。
