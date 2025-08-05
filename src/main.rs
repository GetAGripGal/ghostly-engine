use ghostly::{
    lua::{LuaRuntime, api::LuaAPI},
    world::World,
};

use log::LevelFilter;
use raylib::prelude::{Color, KeyboardKey, RaylibDraw, RaylibDrawHandle, RaylibHandle};
use std::{cell::RefCell, rc::Rc};

fn main() -> anyhow::Result<()> {
    init_logger();

    let (rl, thread) = raylib::init()
        //.fullscreen()
        //.vsync()
        .size(1920, 1080)
        .title("Ghostly")
        .build();
    log::info!("Initialized raylib window.");

    let rl = Rc::new(RefCell::new(rl));
    let thread = Rc::new(thread);

    let world = World::new_cell();

    let lua_runtime = LuaRuntime::new();
    let lua_api = LuaAPI::new(
        rl.clone(),
        thread.clone(),
        world.clone(),
        lua_runtime.systems_cell(), // TODO: This might need refactoring. LuaSystemManager should exist
                                    // independant of LuaRuntime
    );

    lua_runtime.run_script(
        &format!(
            "{}/debug_assets/scripts/input.lua",
            env!("CARGO_MANIFEST_DIR")
        ),
        lua_api.clone(),
    )?;

    'main: loop {
        lua_runtime.systems().update(lua_api.clone());

        {
            let mut rl = rl.borrow_mut();
            let mut draw_handle = rl.begin_drawing(&thread);
            draw_handle.clear_background(Color::BLACK);

            draw_world(&mut draw_handle, &world);
            {
                let world = world.borrow();
                draw_handle.draw_text(
                    &format!("Entity Count: {}", world.entities.len()),
                    0,
                    32,
                    32,
                    Color::LIGHTGREEN,
                );
            }
            draw_handle.draw_fps(0, 0);
        }

        // temp fix
        if rl.borrow().window_should_close() {
            break 'main;
        }
    }

    Ok(())
}

/// Initialize the logger.
fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info) // I hold no secrets
        .init();
}

fn draw_world<'a>(draw_handle: &mut RaylibDrawHandle<'a>, world: &Rc<RefCell<World>>) {
    let world = world.borrow();
    let entities = world
        .entities
        .iter()
        .filter(|entity| entity.enabled())
        .collect();

    const OUTLINE_SIZE: f32 = 8.0;
    entities.iter().for_each(|id| {
        let entity = world.entities.get(*id).unwrap();
        let position = &entity.data.position;
        draw_handle.draw_rectangle(
            position.x as i32,
            position.y as i32,
            64,
            64,
            if id % 2 == 0 {
                Color::BROWN
            } else if id % 3 == 0 {
                Color::DARKRED
            } else {
                Color::DARKBLUE
            },
        );
        draw_handle.draw_rectangle(
            (position.x + OUTLINE_SIZE) as i32,
            (position.y + OUTLINE_SIZE) as i32,
            64 - (OUTLINE_SIZE as i32 * 2),
            64 - (OUTLINE_SIZE as i32 * 2),
            if id % 2 == 0 {
                Color::YELLOW
            } else if id % 3 == 0 {
                Color::RED
            } else {
                Color::BLUE
            },
        );
    });
}
