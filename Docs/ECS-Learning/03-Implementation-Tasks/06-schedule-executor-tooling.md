# 模块六实现大任务：Schedule、执行器、并行与工程化

导航：
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/06-schedule-executor-tooling.md)
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/06-schedule-executor-tooling.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/06-schedule-executor-tooling.md)

## 大任务总览

| 任务 ID | 名称 | 目标 |
| --- | --- | --- |
| M6-T1 | Schedule 图模型 | 建立系统注册、节点、边和构建结果 |
| M6-T2 | 顺序规则与条件 | 支持 before / after / set / condition |
| M6-T3 | 执行器 | 先串行后并行地运行系统图 |
| M6-T4 | 诊断与歧义检测 | 输出冲突、顺序歧义与执行信息 |
| M6-T5 | 工程化验证 | 建立测试、基准、命名、调试辅助 |

## M6-T1 Schedule 图模型

产出：

- `Schedule`
- 系统节点注册
- 图构建缓存

验收标准：

- 系统能够被稳定加入和重建

## M6-T2 顺序规则与条件

产出：

- before / after
- system set
- run condition
- apply deferred 插入点

验收标准：

- 用户声明的顺序约束能反映在执行序中

## M6-T3 执行器

产出：

- 串行执行器
- 并行批次划分或并行规划草案
- 系统运行结果汇总

验收标准：

- 串行执行完全正确
- 并行版本不会违反访问冲突约束

## M6-T4 诊断与歧义检测

产出：

- 冲突报告
- 歧义报告
- 调度构建日志

验收标准：

- 没有显式顺序的冲突系统能够被诊断出来

## M6-T5 工程化验证

产出：

- 调度层测试基建
- 简单 benchmark
- 系统命名与调试打印能力

验收标准：

- 关键路径都能被回归测试覆盖

## 参考 Bevy

- [`schedule/mod.rs`](/e:/RustProgram/Bevy/bevy-0.18/crates/bevy_ecs/src/schedule/mod.rs)
