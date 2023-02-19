use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    let bterm = BTermBuilder::simple_80x50().with_fancy_console(0, 80, 50);
    //bterm.with_post_scanlines(true);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bterm)
        .insert_resource(State {
            x: 0.0,
            spin: Radians::new(0.0),
        })
        .add_system(tick)
        .run();
}

#[derive(Resource)]
struct State {
    x: f32,
    spin: Radians,
}

fn tick(ctx: Res<BracketContext>, mut state: ResMut<State>) {
    let mut draw_batch = ctx.new_draw_batch();

    // Show frame rate
    draw_batch.target(1);
    draw_batch.cls();

    draw_batch.draw_double_box(
        bracket_bevy::prelude::Rect::with_size(39, 0, 20, 3),
        ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
    );
    draw_batch.print_color(
        Point::new(40, 1),
        &format!("FPS: {}", ctx.fps),
        ColorPair::new(RGB::named(YELLOW), RGB::named(BLACK)),
    );
    draw_batch.print_color(
        Point::new(40, 2),
        &format!("Frame Time: {} ms", ctx.frame_time_ms),
        ColorPair::new(RGB::named(CYAN), RGB::named(BLACK)),
    );

    let y = 20.0 + (f32::sin(state.x / 2.0) * 5.0);
    let scale = f32::cos(state.x / 3.0) + 1.1;
    draw_batch.set_fancy(
        PointF { x: state.x, y },
        0,
        state.spin,
        PointF { x: scale, y: scale },
        ColorPair::new(RGB::named(YELLOW), RGB::named(BLACK)),
        to_cp437('@'),
    );

    // Submission
    ctx.submit_batch(0, draw_batch);

    // Moving
    state.x += 0.1;
    if state.x > 80.0 {
        state.x = 0.0;
    }
    state.spin.0 += 0.2;
}
