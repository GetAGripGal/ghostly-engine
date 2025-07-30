use std::{cell::RefCell, rc::Rc};

use mlua::UserData;
use raylib::{RaylibHandle, RaylibThread};

/// The window API for the lua runtime.
#[derive(Debug, Clone)]
pub struct LuaAPIWindow {
    pub rl: Rc<RefCell<RaylibHandle>>,
    pub thread: Rc<RaylibThread>,
}

impl UserData for LuaAPIWindow {
    fn add_fields<F: mlua::UserDataFields<Self>>(_fields: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("set_title", |_, this, value: String| {
            let rl = this.rl.borrow();
            rl.set_window_title(&this.thread, &value);
            Ok(())
        });

        methods.add_method("width", |_, this, ()| {
            let rl = this.rl.borrow();
            Ok(rl.get_render_width())
        });
        methods.add_method("height", |_, this, ()| {
            let rl = this.rl.borrow();
            Ok(rl.get_render_height())
        });
    }
}
