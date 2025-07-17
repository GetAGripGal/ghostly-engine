use cgmath::{Vector3, Zero};

/// The type of an entity id.
pub type EntityId = usize;

/// Represents an entity in the world.
#[derive(Debug)]
pub struct Entity {
    pub(super) id: EntityId,
    pub(super) enabled: bool,
    pub data: EntityData,
}

impl Entity {
    /// Get the entity id.
    pub fn id(&self) -> EntityId { self.id }
    /// Check whether the entity is enabled.
    pub fn enabled(&self) -> bool { self.enabled }
    
    /// (Re)enable an entity.
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Disable the entity
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// The data in an entity.
#[derive(Debug, Clone)]
pub struct EntityData {
    pub role: String,
    pub position: Vector3<f32>,
}

impl Default for EntityData {
    fn default() -> Self { 
        Self {
            role: Default::default(),
            position: Vector3::zero(),
        }
    }
}
