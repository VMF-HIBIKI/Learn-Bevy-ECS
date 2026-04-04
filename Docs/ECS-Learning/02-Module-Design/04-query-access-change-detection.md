# 模块四：Query、访问分析与变更检测

导航：
- [模块设计索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/README.md)
- [本模块实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/04-query-access-change-detection.md)
- [本模块可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/04-query-access-change-detection.md)
- [本模块开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/04-query-access-change-detection.md)

## 模块职责

这个模块负责把“我想读什么数据”编译成“如何安全又高效地访问数据”。如果说存储层解决“数据在哪”，那 Query 层解决“数据怎么拿”。

## Query 至少要解决的五个问题

### 1. 访问声明

系统需要明确声明它读哪些组件、写哪些组件、读哪些资源、写哪些资源。

### 2. 候选过滤

不是所有 Archetype 都要遍历。Query 要先排除不可能匹配的 Archetype。

### 3. 数据提取

对于 `&T`、`&mut T`、`Option<&T>`、实体 ID、Tuple Query、Filter Query，要能统一提取。

### 4. 借用安全

Query 编译结果必须能够产出访问集合，供 Schedule 做冲突分析。

### 5. 变化追踪

查询不仅要拿到数据，还要知道它是不是“本轮新增”“自上次系统运行后改变”。

## 变更检测为什么难

因为它会穿透多个模块：

- 组件写入时要打 tick
- 系统运行时要有自己的上次运行 tick
- Query 取值时要能比较 `added` / `changed`
- Resource 也要共享相同语义

## 建议能力分层

### 第一批

- `Query<&T>`
- `Query<&mut T>`
- `Query<(Entity, &T)>`
- `With<T>` / `Without<T>`

### 第二批

- `Option<&T>`
- Tuple 嵌套
- `Added<T>` / `Changed<T>`
- `RemovedComponents<T>` 或等价跟踪

### 第三批

- `AnyOf`
- 组合迭代
- 并行迭代
- 默认过滤器与禁用实体

## 建议数据结构

- `AccessSet`
- `FilteredAccess`
- `QueryState`
- `FetchState`
- `FilterState`
- `Tick`
- `ComponentTicks`

## 与其他模块关系

- 依赖模块一的 `ComponentId`
- 依赖模块二的 `Archetype` 与 `Table`
- 依赖模块三的 `UnsafeWorldCell`
- 向模块五暴露 `Query` 作为 `SystemParam`
- 向模块六暴露访问集合用于调度冲突分析

## 参考 Bevy 源码

- [`query/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/query/mod.rs)
- [`change_detection`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/change_detection)
- [`lifecycle`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/lifecycle)

## 第一版建议

- Query 状态缓存一定要做，因为它是 Schedule 和 System 重复执行的前提。
- 变更检测第一版先覆盖组件与资源，不急着把所有事件系统一起塞进来。
