# 模块四实现大任务：Query、访问分析与变更检测

导航：
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/04-query-access-change-detection.md)
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/04-query-access-change-detection.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/04-query-access-change-detection.md)

## 大任务总览

| 任务 ID | 名称 | 目标 |
| --- | --- | --- |
| M4-T1 | 访问集合模型 | 表达组件/资源的读写访问与过滤访问 |
| M4-T2 | QueryState 编译 | 把查询声明编译为可缓存状态 |
| M4-T3 | Query 迭代与提取 | 在 Archetype / Table 上执行 fetch |
| M4-T4 | 过滤器与组合查询 | 支持 `With`、`Without`、`Option`、Tuple |
| M4-T5 | 变更检测与移除跟踪 | 支持 `Added`、`Changed`、removed tracking |

## M4-T1 访问集合模型

产出：

- `AccessSet`
- 读写冲突判定
- 过滤后访问模型

验收标准：

- 调度器可据此判断两个系统是否冲突

## M4-T2 QueryState 编译

产出：

- 组件访问声明到内部状态的编译路径
- Archetype 匹配缓存
- 与 World 元数据版本的兼容策略

验收标准：

- 重复执行同一 Query 不需要重新解析所有类型信息

## M4-T3 Query 迭代与提取

产出：

- `&T`
- `&mut T`
- `(Entity, &T)`
- 多组件元组 fetch

验收标准：

- Query 能跨多个 Archetype 正确迭代
- 只读与可写访问不会混用出错

## M4-T4 过滤器与组合查询

产出：

- `With<T>`
- `Without<T>`
- `Option<&T>`
- Tuple Query

验收标准：

- 过滤器不会错误访问不匹配的组件列

## M4-T5 变更检测与移除跟踪

产出：

- `Tick`
- `ComponentTicks`
- `Added<T>`
- `Changed<T>`
- removed components 追踪

验收标准：

- 系统两次运行之间的变化能够被正确识别

## 参考 Bevy

- [`query/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/query/mod.rs)
- [`change_detection`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/change_detection)
