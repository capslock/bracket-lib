use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    let bterm = BTermBuilder::simple_80x50().with_fancy_console(0, 80, 50);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bterm)
        .insert_resource(State { x: 0.0 })
        .add_system(tick)
        .run();
}

fn tick(ctx: Res<BracketContext>, mut state: ResMut<State>) {
    let mut draw_batch = ctx.new_draw_batch();

    draw_batch.target(1);
    draw_batch.cls();

    let simple_x = state.x as i32;
    let fancy_x = state.x + 20.0;

    draw_batch.print(Point::new(0, 0), format!("Simple Console"));
    draw_batch.print(Point::new(0, 1), format!("X={}", simple_x));
    draw_batch.print(Point::new(20, 0), format!("Fancy Console"));
    draw_batch.print(Point::new(20, 1), format!("X={:2}", fancy_x));

    draw_batch.print(Point::new(simple_x, 3), "@");
    draw_batch.set_fancy(
        PointF::new(fancy_x, 4.0),
        1,
        Degrees::new(0.0),
        PointF::new(1.0, 1.0),
        ColorPair::new(WHITE, BLACK),
        to_cp437('@'),
    );

    ctx.submit_batch(0, draw_batch);

    state.x += 0.05;
    if state.x > 10.0 {
        state.x = 0.0;
    }
}

#[derive(Resource)]
struct State {
    x: f32,
}
