use mlua::{LuaSerdeExt, UserData};
use std::cell::RefCell;
use std::rc::Rc;

use crate::world::entity::Entity;

/// A wrapper around an entity cell for the lua runtime.
pub struct LuaEntity(pub Rc<RefCell<Entity>>);

impl UserData for LuaEntity {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| {
            let entity = this.0.borrow();
            Ok(entity.id())
        });
        
        fields.add_field_method_get("role", |_, this| {
            let entity = this.0.borrow();
            Ok(entity.data.role.clone())
        });
        fields.add_field_method_set("role", |_, this, value| {
            let mut entity = this.0.borrow_mut();
            entity.data.role = value;
            Ok(())
        });

        fields.add_field_method_get("position", |lua, this| {
            let entity = this.0.borrow();
            lua.to_value(&entity.data.position)
        });
        fields.add_field_method_set("position", |lua, this, value| {
        let mut entity = this.0.borrow_mut();
            entity.data.position = lua.from_value(value)?;
            Ok(())
        });
    }
}
