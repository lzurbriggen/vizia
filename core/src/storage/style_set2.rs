use crate::{Entity, Rule, GenerationalId};

const INDEX_MASK: u32 = std::u32::MAX / 4;
const INLINE_MASK: u32 = 1 << 31;
const INHERITED_MASK: u32 = 1 << 30;


/// The data index determines which dense array to index
pub enum DataIndex {
    Inline(usize),
    Shared(usize),
}

pub struct EntityIndices {
    array: Vec<EntityIndex>,
}

impl EntityIndices {
    pub fn new() -> Self {
        Self {
            array: Vec::new(),
        }
    }

    /// Insert an inline data index into the 
    pub fn insert_inline(&mut self, entity: Entity, index: usize) {
        if entity.index() >= self.array.len() {
            self.array.resize(entity.index() + 1, EntityIndex::null());
        }
        
        self.array[entity.index()] = EntityIndex::new_inline(index);
        
    }

    pub fn insert_shared(&mut self, entity: Entity) {

    }

    pub fn get(&self, entity: Entity) -> Option<DataIndex> {
        self.array.get(entity.index()).map(|entity_index|{
            let index = entity_index.index();
            if entity_index.is_inline() {
                DataIndex::Inline(index)
            } else {
                DataIndex::Shared(index)
            }
        })
    }
}

/// Represents an index that can either be used to retrieve inline or shared data
///
/// Since inline data will override shared data, this allows the same index to be used
/// with a flag to indicate which data the index refers to.
/// The first bit of the u32 internal value is used to signify if the data index
/// refers to shared (default) or inline data:
/// - 0 - shared
/// - 1 - inline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityIndex(u32);

impl EntityIndex {

    pub fn null() -> Self {
        Self(std::u32::MAX >> 1)
    }

    pub fn new_inline(index: usize) -> Self {
        assert!((index as u32) < INDEX_MASK);
        let value = (index as u32) | INLINE_MASK;
        Self(value)
    }

    pub fn new_shared(index: usize) -> Self {
        assert!((index as u32) < INDEX_MASK);
        Self(index as u32)
    }

    pub fn with_inherited(self, flag: bool) -> Self {
        let value = self.0;
        Self(value | INHERITED_MASK)
    }

    pub fn index(&self) -> usize {
        (self.0 & INDEX_MASK) as usize
    }

    /// Returns true if the data index refers to inline data.
    pub fn is_inline(&self) -> bool {
        (self.0 & INLINE_MASK).rotate_left(1) != 0
    }

    /// Returns true if the data index refers to an inherited value
    pub fn is_inherited(&self) -> bool {
        (self.0 & INHERITED_MASK).rotate_left(2) != 0
    }
}

pub struct RuleIndex(u32);

pub struct Entry<T> {
    value: T,
    entity: Entity,
}

impl<T> Entry<T> {
    pub fn new(entity: Entity, value: T) -> Self {
        Self {
            value,
            entity,
        }
    }
}

pub struct StyleStore<T> {
    entity_indices: EntityIndices,
    rule_indices: Vec<RuleIndex>,
    inline_data: Vec<Entry<T>>,
    shared_data: Vec<Entry<T>>,
}

impl<T> StyleStore<T>
where 
    T: 'static + std::fmt::Debug 
{
    pub fn insert_inline(&mut self, entity: Entity, value: T) {
        if entity == Entity::null() {
            return;
        }

        self.entity_indices.insert_inline(entity, self.inline_data.len());
        self.inline_data.push(Entry::new(entity, value));
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {

        let data_index = self.entity_indices.get(entity)?;
        match data_index {
            DataIndex::Inline(index) => {
                self.inline_data.get(index).map(|entry| &entry.value)
            }

            DataIndex::Shared(index) => {
                self.shared_data.get(index).map(|entry| &entry.value)
            }
        }
    }
}