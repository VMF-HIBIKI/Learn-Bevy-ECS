# 模块六开发计划：Schedule、执行器、并行与工程化

导航：
- [第四层可执行子任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/06-schedule-executor-tooling.md)
- [第二层设计文档](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/06-schedule-executor-tooling.md)

## 推荐开发顺序

| 顺序 | 子任务 | 建议 Issue 标题 | 前置条件 | 测试重点 | 完成定义 |
| --- | --- | --- | --- | --- | --- |
| 1 | M6-S1 | `feat: define schedule graph primitives` | M5-S2 | Node / Edge 建模 | 调度图结构可用 |
| 2 | M6-S2 | `feat: cache schedule build results` | M6-S1 | 系统注册与重建 | Schedule 可稳定构建 |
| 3 | M6-S3 | `feat: add explicit system ordering edges` | M6-S2 | before / after | 顺序约束可生效 |
| 4 | M6-S4 | `feat: add system sets and run conditions` | M6-S3 | set / condition | 条件执行可工作 |
| 5 | M6-S5 | `feat: insert apply deferred schedule boundaries` | M5-S8, M6-S4 | deferred 刷新点 | Commands 与 Schedule 协调完成 |
| 6 | M6-S6 | `feat: implement serial schedule executor` | M6-S5 | 顺序运行、错误传播 | 串行执行正确 |
| 7 | M6-S7 | `perf: draft parallel execution planning` | M4-S2, M6-S6 | 冲突分批、依赖边 | 并行规划原型存在 |
| 8 | M6-S8 | `feat: report schedule ambiguities and conflicts` | M6-S7 | 歧义诊断 | 冲突系统可被报告 |
| 9 | M6-S9 | `feat: add schedule diagnostics and naming` | M6-S8 | 系统名、调试输出 | 调度可观测性增强 |
| 10 | M6-S10 | `test: add schedule regression and benchmark harness` | M6-S9 | 回归与性能基线 | 调度层验证基建建立 |

## 实施建议

- 不要把并行执行器当作第一优先级；串行执行器才是整个 ECS 的正确性基座。
- 只有在访问集合、系统顺序和 deferred 边界稳定后，再推进并行规划，才不会把调试难度放大。
