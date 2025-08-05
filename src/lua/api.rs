use std::{cell::RefCell, rc::Rc};

use mlua::UserData;
use raylib::{RaylibHandle, RaylibThread};

use crate::{
    lua::{
        input::LuaAPIInput, performance::LuaAPIPerformance, systems::{LuaAPISystemManager, LuaSystemManager}, window::LuaAPIWindow, world::LuaAPIWorld
    },
    world::World,
};

/// A macro that exposes a lua api to the runtime.
macro_rules! expose_lua_api {
    ($methods_userdata:ident, $api_name:expr, $api_field_name:ident) => {
        $methods_userdata.add_method($api_name, |_, this, ()| Ok(this.$api_field_name.clone()));        
    };
}

/// The api for the engine's lua runtime.
#[derive(Debug, Clone)]
pub struct LuaAPI {
    world: LuaAPIWorld,
    window: LuaAPIWindow,
    systems: LuaAPISystemManager,
    performance: LuaAPIPerformance,
    input: LuaAPIInput,
}

impl LuaAPI {
    /// Initialize a lua context and it's API's.
    pub fn new(
        rl: Rc<RefCell<RaylibHandle>>,
        thread: Rc<RaylibThread>,
        world: Rc<RefCell<World>>,
        systems: Rc<RefCell<LuaSystemManager>>,
    ) -> Self {
        Self {
            world: LuaAPIWorld(world),
            window: LuaAPIWindow {
                rl: rl.clone(),
                thread: thread.clone(),
            },
            systems: LuaAPISystemManager(systems),
            performance: LuaAPIPerformance(rl.clone()),
            input: LuaAPIInput(rl.clone()),
        }
    }
}

impl UserData for LuaAPI {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        expose_lua_api!(methods, "world", world);
        expose_lua_api!(methods, "window", window);
        expose_lua_api!(methods, "systems", systems);
        expose_lua_api!(methods, "performance", performance);
        expose_lua_api!(methods, "input", input);
    }
}
