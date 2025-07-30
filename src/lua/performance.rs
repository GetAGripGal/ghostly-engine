use std::{cell::RefCell, rc::Rc};

use mlua::UserData;
use raylib::RaylibHandle;

/// The performance api for the lua runtime.
#[derive(Debug, Clone)]
pub struct LuaAPIPerformance(pub Rc<RefCell<RaylibHandle>>);

impl UserData for LuaAPIPerformance {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("delta_secs", |_, this, ()| {
            let rl = this.0.borrow();
            Ok(rl.get_frame_time())
        });

        methods.add_method("fps", |_, this, ()| {
            let rl = this.0.borrow();
            Ok(rl.get_fps())
        });
    }
}
