use bevy::prelude::*;
use bracket_bevy::prelude::*;
use std::cmp::{max, min};

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50().with_random_number_generator(true))
        .add_startup_system(setup)
        .add_system(draw_map)
        .insert_resource(FixedTime::new_from_secs(0.1))
        .add_system_to_schedule(CoreSchedule::FixedUpdate, player_input)
        .run();
}

fn setup(mut commands: Commands, rng: Res<RandomNumbers>) {
    let (rooms, map) = new_map_rooms_and_corridors(&rng);
    let (player_x, player_y) = rooms[0].center();
    commands.insert_resource(map);
    commands
        .spawn_empty()
        .insert(Position {
            x: player_x,
            y: player_y,
        })
        .insert(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .insert(Player {});
}

fn player_input(
    map: Res<Map>,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Position, With<Player>>,
) {
    let delta = player::player_input(&keyboard);
    if delta != (0, 0) {
        let mut pos = player_query.single_mut();
        let destination_idx = xy_idx(pos.x + delta.0, pos.y + delta.1);
        if map.0[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta.0));
            pos.y = min(49, max(0, pos.y + delta.1));
        }
    }
}

fn draw_map(ctx: Res<BracketContext>, map: Res<Map>, query: Query<(&Position, &Renderable)>) {
    ctx.cls();

    map::draw_map(&map.0, &ctx);
    for (pos, render) in query.iter() {
        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
}
