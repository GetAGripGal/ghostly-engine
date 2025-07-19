use std::{cell::RefCell, rc::Rc};
use ghostly::{lua::world::LuaWorld, world::World};
use mlua::Lua;
use raylib::prelude::{Color, KeyboardKey, RaylibDraw, RaylibDrawHandle, RaylibHandle};

fn main() {
    let world = {
        let world = World::default();
        Rc::new(RefCell::new(world))
    };

    let lua = Lua::new();
    let globals = lua.globals();

    let lua_world_ref = LuaWorld(world.clone());
    
    let script = std::fs::read_to_string(format!("{}/assets/scripts/debug.lua", env!("CARGO_MANIFEST_DIR"))).unwrap();
    lua.load(script).exec().unwrap();

    let script_init: mlua::Function = globals.get("Init").unwrap();
    let script_update: mlua::Function = globals.get("Update").unwrap();

    let (mut rl, thread) = raylib::init()
        .vsync()
        //.fullscreen()
        .size(1280, 720)
        .title("Ghostly")
        .build();

    script_init.call::<()>(lua_world_ref.clone()).unwrap();

    while !rl.window_should_close() {
        script_update.call::<()>(lua_world_ref.clone()).unwrap();
        move_players(&rl, &world);

        let mut draw_handle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);

        draw_world(&mut draw_handle, &world);
        draw_handle.draw_fps(0, 0);
    }
}

fn move_players(rl: &RaylibHandle, world: &Rc<RefCell<World>>) {
    let world = world.borrow_mut();
    let players = world
        .entities
        .iter()
        .filter(|entity| entity.enabled())
        .filter(|entity| entity.data.role == "player")
        .collect();
    players.iter().for_each(|id| {
        let mut entity = world.entities.get_mut(*id).unwrap();
        let xinput = -(rl.is_key_down(KeyboardKey::KEY_A) as i8 as f32)
            + rl.is_key_down(KeyboardKey::KEY_D) as i8 as f32;
        entity.data.position.x += xinput * 200f32 * rl.get_frame_time();
    });
}
fn draw_world<'a>(draw_handle: &mut RaylibDrawHandle<'a>, world: &Rc<RefCell<World>>) {
    let world = world.borrow();
    let entities = world
        .entities
        .iter()
        .filter(|entity| entity.enabled())
        .collect();
    entities.iter().for_each(|id| {
        let entity = world.entities.get(*id).unwrap();
        let position = &entity.data.position;
        draw_handle.draw_rectangle(position.x as i32, position.y as i32, 64, 64, Color::YELLOW);
    });
}
