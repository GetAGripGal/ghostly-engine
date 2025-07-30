use std::{cell::RefCell, rc::Rc};

use mlua::UserData;

use crate::lua::api::LuaAPI;

/// The systems api for lua scripts.
#[derive(Debug, Clone)]
pub struct LuaAPISystemManager(pub Rc<RefCell<LuaSystemManager>>);

/// Manages the systems defined in lua scripts.
/// Systems are lua functions registered to run at certain intervals or triggers.
#[derive(Debug, Default)]
pub struct LuaSystemManager {
    update_systems: Vec<mlua::Function>,
}

impl LuaSystemManager {
    /// Create a new refcell.
    pub fn new_cell() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }

    /// Add an update system.
    pub fn add_update(&mut self, system: mlua::Function) {
        self.update_systems.push(system);
    }

    /// Trigger the update systems.
    pub fn update(&self, api: LuaAPI) {
        self.update_systems.iter().for_each(|system| {
            let _ = system
                .call::<()>(api.clone())
                .map_err(|e| log::error!("Error in system: {}", e));
        });
    }

    /// Remove all system.
    pub fn clear(&mut self) {
        self.update_systems.clear();
    }
}

impl UserData for LuaAPISystemManager {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("register_update", |_, this, system: mlua::Function| {
            let mut systems = this.0.borrow_mut();
            systems.add_update(system);
            Ok(())
        });
    }
}
