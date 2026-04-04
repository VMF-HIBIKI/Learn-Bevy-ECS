# 模块六可执行子任务：Schedule、执行器、并行与工程化

导航：
- [第三层实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/06-schedule-executor-tooling.md)
- [第五层开发计划](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/05-Development-Plans/06-schedule-executor-tooling.md)

## 子任务表

| 子任务 ID | 对应大任务 | 子任务内容 | 完成标志 |
| --- | --- | --- | --- |
| M6-S1 | M6-T1 | 定义 Schedule、Node、Edge 数据结构 | 调度图有稳定表示 |
| M6-S2 | M6-T1 | 实现系统注册与构建缓存 | 系统可重复加入与重建 |
| M6-S3 | M6-T2 | 实现 before / after 显式边 | 顺序约束可表达 |
| M6-S4 | M6-T2 | 实现 set 与 run condition | 分组与条件执行可工作 |
| M6-S5 | M6-T2 | 插入 apply_deferred 边界 | Commands 刷新与调度整合 |
| M6-S6 | M6-T3 | 实现串行执行器 | Schedule 可以稳定运行 |
| M6-S7 | M6-T3 | 实现并行冲突分批或规划草案 | 并行为后续演进打基础 |
| M6-S8 | M6-T4 | 实现歧义与冲突诊断 | 没有顺序约束的冲突会被报告 |
| M6-S9 | M6-T4 | 实现系统命名、调试输出与 tracing 钩子 | 调度可观测性提升 |
| M6-S10 | M6-T5 | 建立调度层测试与 benchmark 基建 | 工程化验证可持续推进 |

## 执行提示

- 串行执行器是正确性基线，不是临时代码。
- 并行规划一定要建立在访问集合和显式顺序已经稳定的前提下。
