# 模块一开发计划：实体、组件与元数据

导航：
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/01-entity-component-metadata.md)
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/01-entity-component-metadata.md)

## 推荐开发顺序

| 顺序 | 子任务 | 建议 Issue 标题 | 前置条件 | 测试重点 | 完成定义 |
| --- | --- | --- | --- | --- | --- |
| 1 | M1-S1 | `feat: add entity handle type` | 无 | 实体比较、调试输出 | `Entity` 类型可独立编译 |
| 2 | M1-S2 | `test: validate stale entity generations` | M1-S1 | 旧 generation 失效 | 旧句柄测试全部通过 |
| 3 | M1-S3 | `feat: add entity metadata layout` | M1-S1 | alive / location 初值 | 有可扩展元数据结构 |
| 4 | M1-S4 | `feat: implement entity free list allocator` | M1-S3 | 分配、回收、复用 | 分配器可独立测试 |
| 5 | M1-S5 | `feat: add reserved entity flow` | M1-S4 | reserve / flush 场景 | 预留机制不破坏活跃实体 |
| 6 | M1-S6 | `feat: recycle entity generations on despawn` | M1-S4 | despawn 后复用 index | generation 递增行为稳定 |
| 7 | M1-S7 | `feat: define component descriptor metadata` | 无 | 类型信息、存储类型 | 描述结构完整 |
| 8 | M1-S8 | `feat: implement component registry ids` | M1-S7 | 同类型重复注册 | `ComponentId` 稳定返回 |
| 9 | M1-S9 | `feat: expose component registration through world` | M1-S8 | World 包装 API | 上层可调用注册 |
| 10 | M1-S10 | `feat: add bundle metadata compilation` | M1-S8 | Bundle 组件枚举 | 有 `BundleInfo` 基础 |
| 11 | M1-S11 | `test: validate bundle component ordering` | M1-S10 | 排序、去重、重复检测 | Bundle 校验逻辑稳定 |
| 12 | M1-S12 | `feat: add required component metadata graph` | M1-S8 | 组件依赖记录 | required 关系可存储 |
| 13 | M1-S13 | `feat: expand required components during bundle compile` | M1-S12 | 自动补组件、覆盖规则 | required 展开闭环完成 |

## 实施建议

- 前六个子任务先完成“实体生命周期闭环”，不要同时推进组件注册。
- 组件注册和 Bundle 编译建议在同一阶段完成，因为 Bundle 对 `ComponentId` 依赖很重。
- Required Components 可以先用最直接的数据结构做正确版本，循环检测和高级优化后置。

## 每个子任务的固定动作

1. 先补失败测试或最小 smoke test。
2. 再写实现，不要跳过中间的不可变接口设计。
3. 跑 `cargo +nightly fmt --all -- --check`。
4. 跑 `cargo +nightly clippy --all-targets --all-features -- -D warnings`。
5. 更新对应文档中的状态和风险说明。
