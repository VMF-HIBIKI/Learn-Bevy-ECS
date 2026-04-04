# 模块二开发计划：存储、Archetype 与布局迁移

导航：
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/02-storage-archetype.md)
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/02-storage-archetype.md)

## 推荐开发顺序

| 顺序 | 子任务 | 建议 Issue 标题 | 前置条件 | 测试重点 | 完成定义 |
| --- | --- | --- | --- | --- | --- |
| 1 | M2-S1 | `feat: define table storage primitives` | M1-S8 | 表结构创建 | Table 基础结构可用 |
| 2 | M2-S2 | `feat: implement table column write and drop` | M2-S1 | 写入、读取、drop | 单列生命周期正确 |
| 3 | M2-S3 | `feat: support table row insert and swap remove` | M2-S2 | 行删除影响位置修正 | 表行迁移测试通过 |
| 4 | M2-S4 | `feat: define sparse set storage layout` | M1-S8 | sparse / dense 一致性 | SparseSet 结构完整 |
| 5 | M2-S5 | `feat: implement sparse set mutations` | M2-S4 | 插入、更新、删除 | 稀疏组件闭环完成 |
| 6 | M2-S6 | `feat: define archetype signatures` | M1-S10 | 组件集合排序 | Archetype 签名可比较 |
| 7 | M2-S7 | `feat: implement archetype registry lookup` | M2-S6 | 注册与命中 | Archetype 可复用 |
| 8 | M2-S8 | `feat: cache archetype transition edges` | M2-S7 | add/remove 命中路径 | 迁移边可缓存 |
| 9 | M2-S9 | `feat: wire table only spawn path` | M2-S3, M2-S7 | 从 spawn 到存储落盘 | table-only spawn 正确 |
| 10 | M2-S10 | `feat: migrate entities when adding bundles` | M2-S8, M2-S9 | add component 迁移 | 添加组件进入正确布局 |
| 11 | M2-S11 | `feat: migrate entities when removing components` | M2-S10 | remove component 迁移 | 删除组件流程正确 |
| 12 | M2-S12 | `feat: clean up storages on entity despawn` | M2-S11 | despawn 全路径 | 所有位置同步回收 |
| 13 | M2-S13 | `perf: add initial batch spawn path` | M2-S9 | 批量创建一致性 | batch spawn 正确可用 |

## 实施建议

- 先做 table-only，是为了把 Archetype 的核心迁移流程尽快走通。
- SparseSet 一定要在有 Archetype 签名之后接入，否则查询层会出现“逻辑拥有组件但布局层找不到”的混乱。
- 所有迁移路径都必须写 swap-remove 受影响尾实体修正测试。
