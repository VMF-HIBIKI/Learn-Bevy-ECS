# 模块一可执行子任务：实体、组件与元数据

导航：
- [第三层实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/01-entity-component-metadata.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/01-entity-component-metadata.md)

## 子任务表

| 子任务 ID | 对应大任务 | 子任务内容 | 完成标志 |
| --- | --- | --- | --- |
| M1-S1 | M1-T1 | 定义 `Entity` 的字段布局、构造与调试输出 | 有实体类型和基础单测 |
| M1-S2 | M1-T1 | 编写 generation 失效测试 | 旧句柄不会误判有效 |
| M1-S3 | M1-T2 | 定义 `EntityMeta` 与 `EntityLocation` 占位结构 | 实体元数据能描述 alive 与位置 |
| M1-S4 | M1-T2 | 实现 free list 分配器 | 可重复分配并回收 index |
| M1-S5 | M1-T2 | 实现 reserve / flush reserve 流程 | 预留实体不会打乱已活跃实体 |
| M1-S6 | M1-T2 | 实现 despawn 与 generation 递增 | 回收后复用 index 时 generation 正确 |
| M1-S7 | M1-T3 | 定义 `StorageType`、`ComponentDescriptor` | 组件元数据结构稳定 |
| M1-S8 | M1-T3 | 实现组件注册表与 `ComponentId` 映射 | 同类型注册结果稳定一致 |
| M1-S9 | M1-T3 | 暴露 World 级组件注册 API | 上层可通过 World 完成注册 |
| M1-S10 | M1-T4 | 定义 `Bundle` 语义与 `BundleInfo` | 可描述组件集合与写入入口 |
| M1-S11 | M1-T4 | 实现 Bundle 组件排序、去重与校验 | 重复组件能被检测 |
| M1-S12 | M1-T5 | 设计 required components 存储结构 | 组件依赖关系可记录 |
| M1-S13 | M1-T5 | 实现 required 展开与冲突测试 | 自动补组件流程可验证 |

## 执行提示

- M1-S1 到 M1-S6 只碰实体生命周期，不要提前混入 Archetype 迁移。
- M1-S7 到 M1-S9 只解决“类型注册”，不要把 Query 需求提前耦合进来。
- M1-S10 到 M1-S13 的重点是“编译元数据”，不是直接写存储值。
