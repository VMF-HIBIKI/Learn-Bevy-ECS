use std::{fmt, num::NonZeroU32};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity {
    index: u32,
    generation: NonZeroU32,
}

impl Entity {
    pub fn new(index: u32, generation: NonZeroU32) -> Self {
        Self { index, generation }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn generation(&self) -> NonZeroU32 {
        self.generation
    }
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 固定日志格式，避免以后内部表示变化时把测试输出搞乱。
        f.debug_struct("Entity")
            .field("index", &self.index)
            .field("generation", &self.generation.get())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Entity;
    use std::{collections::HashSet, num::NonZeroU32};

    fn generation(value: u32) -> NonZeroU32 {
        NonZeroU32::new(value).expect("generation must be non-zero")
    }

    fn is_current_generation(entity: Entity, current_generation: NonZeroU32) -> bool {
        entity.generation() == current_generation
    }

    #[test]
    fn identity_uses_index_and_generation() {
        assert_ne!(Entity::new(1, generation(1)), Entity::new(1, generation(2)));
    }

    #[test]
    fn stale_generation_is_rejected() {
        let entity = Entity::new(3, generation(2));

        assert!(is_current_generation(entity, generation(2)));
        assert!(!is_current_generation(entity, generation(1)));
        assert!(!is_current_generation(entity, generation(3)));
    }

    #[test]
    fn entity_can_be_hash_key() {
        let entity = Entity::new(7, generation(1));
        let mut set = HashSet::new();
        set.insert(entity);
        assert!(set.contains(&entity));
    }

    #[test]
    fn debug_output_is_stable() {
        assert_eq!(
            format!("{:?}", Entity::new(9, generation(4))),
            "Entity { index: 9, generation: 4 }"
        );
    }
}
