pub mod entity;

use crate::world::entity::{Entity, EntityData, EntityId};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

/// Represents the game world.
#[derive(Debug, Default)]
pub struct World {
    pub entities: EntityStore,
}

/// A wrapper for a vector of entities.
#[derive(Debug, Default, Clone)]
pub struct EntityStore(Vec<Rc<RefCell<Entity>>>);

impl EntityStore {
    /// Get the amount of the entities in the store.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Spawn an entity with the provided data.
    /// ### Returns
    /// The entity id.
    /// ### Remarks
    /// The entity will be enabled by default.
    pub fn spawn(&mut self, data: EntityData) -> EntityId {
        self._spawn(data, true)
    }

    /// Spawn an entity with the provided data that is disabled by default.
    /// ### Returns
    /// The entity id.
    /// ### Remarks
    /// The entity will be disabled by default.
    pub fn spawn_disabled(&mut self, data: EntityData) -> EntityId {
        self._spawn(data, false)
    }

    fn _spawn(&mut self, data: EntityData, enabled: bool) -> EntityId {
        let id = self.0.len();
        self.0
            .push(Rc::new(RefCell::new(Entity { id, enabled, data })));
        id
    }

    /// Get a refernce te the entity's refcell directly wrapped in an [`std::rc::Rc`].
    /// ### Returns
    /// An [`std::rc::Rc`] of a refcell for the entity of the provided id. Returns None if the id is invalid.
    pub fn get_cell(&self, id: EntityId) -> Option<Rc<RefCell<Entity>>> {
        let cell = self.0.get(id);
        cell.cloned()
    }

    /// Get a immutable reference to an entity by it's id.
    /// ### Returns
    /// An immutable reference to the entity with the provided id. Returns None if the id is invalid.
    pub fn get(&self, id: EntityId) -> Option<Ref<Entity>> {
        let cell = self.0.get(id)?;
        Some(cell.borrow())
    }

    /// Get a mutable reference to an entity by it's id.
    /// ### Returns
    /// A mutable reference to the entity with the provided id. Returns None if the id is invalid.
    pub fn get_mut(&self, id: EntityId) -> Option<RefMut<Entity>> {
        let cell = self.0.get(id)?;
        Some(cell.borrow_mut())
    }

    /// Returns an iterator of the entity id's in the store.
    /// ### Returns
    /// Creates an [`EntityIdIter`] containing the entity id's.
    pub fn iter(&self) -> EntityIdIter {
        EntityIdIter::new(self)
    }
}

/// A wrapper for an iterator of entity id's allowing for easier filtering.
pub struct EntityIdIter<'a> {
    entities: &'a EntityStore,
    iter: Box<dyn Iterator<Item = EntityId> + 'a>,
}

impl<'a> EntityIdIter<'a> {
    fn new(entities: &'a EntityStore) -> Self {
        let iter = Box::new(0..entities.len()) as Box<dyn Iterator<Item = EntityId>>;
        Self { entities, iter }
    }

    /// Filter the entity id and returns an iterator containing the filtered list.
    /// ### Returns
    /// An [`EntityIdIter`] containing the filtered list.
    pub fn filter<P>(self, predicate: P) -> EntityIdIter<'a>
    where
        P: Fn(Ref<Entity>) -> bool + 'a,
    {
        let filtered_iter = Box::new(self.iter.filter(move |id| {
            let entity = self
                .entities
                .get(*id)
                .expect(&format!("Invalid entity id during filtering: {}", id));
            predicate(entity)
        }));

        Self {
            entities: self.entities,
            iter: filtered_iter,
        }
    }

    /// Collect the entity ids.
    /// ### Returns
    /// A list of entity id'.
    pub fn collect(self) -> Vec<EntityId> {
        self.iter.collect()
    }
}
