use std::{cell::{Ref, RefCell, RefMut}, path::Path, rc::Rc};

use mlua::{Lua, Table};

use crate::lua::{api::LuaAPI, systems::LuaSystemManager};

pub mod api;
pub mod entity;
pub mod systems;
pub mod window;
pub mod world;
pub mod performance;

/// The lua runtime for the engine.
#[derive(Debug)]
pub struct LuaRuntime {
    lua: Lua,
    systems: Rc<RefCell<LuaSystemManager>>,
}

impl LuaRuntime {
    /// Initialize the lua runtime.
    pub fn new() -> Self {
        Self { 
            lua: Lua::new(), 
            systems: LuaSystemManager::new_cell() 
        }
    }

    /// Load a script from a file and run it.
    pub fn run_script<P>(&self, path: P, api: LuaAPI) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let code = std::fs::read_to_string(path)?;

        // Each script is run in it's own scope.
        self.lua
            .scope(|_| {
                let globals = self.lua.globals();
                setup_package_paths(path, &globals)?;
                let init_fn = self.lua.load(&code).eval::<mlua::Function>()?;
                init_fn.call::<()>(api)?;
                Ok(())
            })
            .map_err(|e| anyhow::anyhow!("Failed to execute script `{:?}`: {}", path, e))?;
        Ok(())
    }

    /// Return a reference to the system manager.
    pub fn systems(&self) -> Ref<LuaSystemManager> {
        self.systems.borrow()
    }

    /// Return an ref-counted reference to the system manager's cell.
    pub fn systems_cell(&self) -> Rc<RefCell<LuaSystemManager>> {
        self.systems.clone()
    }
}

/// Setup the package paths to allow for proper imports.
fn setup_package_paths(path: &Path, globals: &Table) -> mlua::Result<()> {
    let package: Table = globals.get("package")?;
    let package_path: String = package.get("path")?;
    let search_path = path.parent().unwrap();
    let search_path = search_path.as_os_str().to_str().unwrap();

    package.set("path", format!("{}/?.lua;{}", search_path, package_path))?;
    globals.set("package", package)?;

    Ok(())
}
