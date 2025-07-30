use std::{cell::RefCell, rc::Rc};

use mlua::UserData;
use raylib::{RaylibHandle, RaylibThread};

use crate::{
    lua::{
        performance::LuaAPIPerformance, systems::{LuaAPISystemManager, LuaSystemManager}, window::LuaAPIWindow, world::LuaAPIWorld
    },
    world::World,
};

/// The api for the engine's lua runtime.
#[derive(Debug, Clone)]
pub struct LuaAPI {
    world: LuaAPIWorld,
    window: LuaAPIWindow,
    systems: LuaAPISystemManager,
    performance: LuaAPIPerformance,
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
            performance: LuaAPIPerformance(rl.clone())
        }
    }
}

impl UserData for LuaAPI {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("world", |_, this, ()| Ok(this.world.clone()));
        methods.add_method("window", |_, this, ()| Ok(this.window.clone()));
        methods.add_method("systems", |_, this, ()| Ok(this.systems.clone()));
        methods.add_method("performance", |_, this, ()| Ok(this.performance.clone()));
    }
}
