use crate::{
    lua::entity::LuaEntity,
    world::{World, entity::EntityId},
};
use mlua::UserData;
use std::{cell::RefCell, rc::Rc};

/// A wrapper around a world for the lua runtime.
#[derive(Debug, Clone)]
pub struct LuaWorld(pub Rc<RefCell<World>>);

impl UserData for LuaWorld {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("spawn", |_, this, ()| {
            let mut world = this.0.borrow_mut();
            Ok(world.entities.spawn(Default::default()))
        });

        methods.add_method("get_entity", |_, this, id: EntityId| {
            let world = this.0.borrow();
            let cell = world.entities.get_cell(id).unwrap();
            Ok(LuaEntity(cell))
        });
    }
}
