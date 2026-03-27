struct EntityIndex(u32);
struct EntityGeneration(u32);

struct Entity {
    index: EntityIndex,
    generation: EntityGeneration,
}

struct EntityMeta {
    /// 当前这个数组下标（Index）的版本号
    generation: EntityGeneration,
    /// 指向该实体组件数据真正存放的底层二维表（Archetype 和 Table）的具体位置
    location: Option<EntityLocation>,
    /// 记录该实体最后一次被创建（Spawn）或被销毁（Despawn）的具体位置（代码调用栈跟踪）以及引擎的全局滴答数（Tick）
    spawned_or_despawned: SpawnedOrDespawned,
}

/// 指向该实体组件数据真正存放的底层二维表（Archetype 和 Table）的具体位置
struct EntityLocation {
    //TODO：需要在Archetype和Table实现之后再回来完成
}

/// 记录该实体最后一次被创建（Spawn）或被销毁（Despawn）的具体位置（代码调用栈跟踪）以及引擎的全局滴答数（Tick）
struct SpawnedOrDespawned {
    //TODO: 需要实现Tick功能之后再回来实现相关逻辑
}

/// World的组件
struct Entites {
    entities: Vec<EntityMeta>,
}

struct World {
    id: u32,
    entities: Entites,
}
