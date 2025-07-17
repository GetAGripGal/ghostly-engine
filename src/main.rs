use cgmath::Vector3;
use ghostly::world::{World, entity::EntityData};
use raylib::prelude::{Color, KeyboardKey, RaylibDraw, RaylibDrawHandle, RaylibHandle};

fn main() {
    let mut world = World::default();
    world.entities.spawn(EntityData {
        role: "player".into(),
        position: Vector3::new(1280f32 / 2f32, 720f32 / 2f32, 0f32),
    });

    let (mut rl, thread) = raylib::init()
        .vsync()
        .fullscreen()
        .size(1280, 720)
        .title("Ghostly")
        .build();

    while !rl.window_should_close() {
        move_players(&rl, &world);

        let mut draw_handle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);

        draw_world(&mut draw_handle, &world);
        draw_handle.draw_fps(0, 0);
    }
}

fn move_players(rl: &RaylibHandle, world: &World) {
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

fn draw_world<'a>(draw_handle: &mut RaylibDrawHandle<'a>, world: &World) {
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
