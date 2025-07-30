use crate::{
    lua::entity::LuaEntity,
    world::{World, entity::EntityId},
};
use mlua::UserData;
use std::{cell::RefCell, rc::Rc};

/// A wrapper around a world for the lua runtime.
#[derive(Debug, Clone)]
pub struct LuaAPIWorld(pub Rc<RefCell<World>>);

impl UserData for LuaAPIWorld {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("spawn", |_, this, ()| {
            let mut world = this.0.borrow_mut();
            Ok(world.entities.spawn(Default::default()))
        });

        methods.add_method("entity_count", |_, this, ()| {
            let world = this.0.borrow();
            Ok(world.entities.len())
        });

        methods.add_method("get_entity", |_, this, id: EntityId| {
            let world = this.0.borrow();
            let cell = world
                .entities
                .get_cell(id)
                .ok_or(mlua::Error::runtime(&format!("Invalid entity id: {}", id)))?;
            Ok(LuaEntity(cell))
        });
    }
}
