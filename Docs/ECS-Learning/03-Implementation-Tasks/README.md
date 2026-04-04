# 第三层：模块实现大任务

返回上层：
- [ECS Learning 根索引](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/README.md)
- [第二层模块设计](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/02-Module-Design/README.md)

## 模块任务入口

1. [实体、组件与元数据](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/01-entity-component-metadata.md)
2. [存储、Archetype 与布局迁移](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/02-storage-archetype.md)
3. [World、资源与访问外观](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/03-world-resource-access.md)
4. [Query、访问分析与变更检测](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/04-query-access-change-detection.md)
5. [System、SystemParam、命令与事件](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/05-system-command-event.md)
6. [Schedule、执行器、并行与工程化](/e:/RustProject/Learn-Bevy-ECS/Docs/ECS-Learning/03-Implementation-Tasks/06-schedule-executor-tooling.md)

## 本层目标

- 把“模块设计”变成真正可排期的实现任务。
- 每个大任务都能映射到后面的可执行子任务与开发计划。
- 保证任务边界尽量对应单一机制，而不是混杂多个核心难点。
