# 模块一实现大任务：实体、组件与元数据

导航：
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/01-entity-component-metadata.md)
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/01-entity-component-metadata.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/01-entity-component-metadata.md)

## 大任务总览

| 任务 ID | 名称 | 目标 |
| --- | --- | --- |
| M1-T1 | Entity 标识模型 | 建立稳定的 `index + generation` 实体句柄 |
| M1-T2 | 实体分配与回收器 | 支持 spawn、reserve、despawn、reuse |
| M1-T3 | 组件注册表 | 把 `TypeId` 编译为 `ComponentId` 与描述信息 |
| M1-T4 | Bundle 元数据编译 | 把组件组合编译成稳定插入协议 |
| M1-T5 | Required Components 图 | 为自动补组件与后续 hooks 预留基础 |

## M1-T1 Entity 标识模型

产出：

- `Entity` 数据结构
- generation 校验逻辑
- 实体相等性、排序、调试展示规范

验收标准：

- 旧 generation 的实体句柄不能误判为有效实体
- 实体可以作为 map / set 的 key
- 可以稳定打印日志与测试断言

## M1-T2 实体分配与回收器

产出：

- free list
- reserved entities 能力
- spawn / despawn 的状态流转
- entity meta 与 location 占位

验收标准：

- despawn 后 index 可以复用
- 复用时 generation 递增
- 批量分配不会破坏现有实体

## M1-T3 组件注册表

产出：

- `ComponentId`
- `ComponentDescriptor`
- `ComponentInfo`
- `StorageType`
- 类型到组件 ID 的映射缓存

验收标准：

- 同一类型多次注册返回同一 `ComponentId`
- 能查询组件名称、布局、存储类型
- 后续 Query 能依赖该注册表进行编译

## M1-T4 Bundle 元数据编译

产出：

- `BundleInfo`
- 组件集合排序与去重逻辑
- 组件写入入口表

验收标准：

- Tuple / 自定义 Bundle 都能被编译为稳定组件集合
- 重复组件在开发期可被拦截
- Bundle 能为模块二的布局迁移提供输入

## M1-T5 Required Components 图

产出：

- 组件依赖声明结构
- required 展开算法
- 依赖顺序约束文档

验收标准：

- 插入组件 A 时能自动补齐依赖组件
- 已显式提供的组件不会被重复覆盖
- 文档中明确记录循环依赖暂未处理或如何处理

## 风险与边界

- 这一层不要急着做宏，先用手写注册路径把机制立住。
- Required Components 不能反过来侵入存储层实现；它应当先展开为最终组件集合。

## 参考 Bevy

- [`entity/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/entity/mod.rs)
- [`component/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/component/mod.rs)
