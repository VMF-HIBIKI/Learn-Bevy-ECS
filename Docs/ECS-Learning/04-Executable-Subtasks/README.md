# 第四层：可执行子任务

返回上层：
- [ECS Learning 根索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/README.md)
- [第三层实现大任务](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/README.md)

## 子任务入口

1. [实体、组件与元数据](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/01-entity-component-metadata.md)
2. [存储、Archetype 与布局迁移](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/02-storage-archetype.md)
3. [World、资源与访问外观](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/03-world-resource-access.md)
4. [Query、访问分析与变更检测](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/04-query-access-change-detection.md)
5. [System、SystemParam、命令与事件](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/05-system-command-event.md)
6. [Schedule、执行器、并行与工程化](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/04-Executable-Subtasks/06-schedule-executor-tooling.md)

## 本层目标

- 让每个大任务都能进一步拆成单次可编码、可测试、可提交的工作单元。
- 子任务默认要能映射到一个 Issue 和一个 Commit。
