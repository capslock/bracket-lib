use crate::{BracketCamera, BracketContext, TerminalScalingMode};
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    ecs::event::Events,
    prelude::*,
    render::camera::RenderTarget,
    sprite::Mesh2dHandle,
    window::{PrimaryWindow, WindowRef, WindowResized},
};

use super::{BracketMesh, ScreenScaler};

pub(crate) fn update_consoles(
    ctx: Res<BracketContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    find_mesh: Query<&BracketMesh, With<Mesh2dHandle>>,
    scaler: Res<ScreenScaler>,
) {
    let mut terms = ctx.terminals.lock();
    for id in find_mesh.iter() {
        let terminal_id = id.0;
        terms[terminal_id].new_mesh(&ctx, &mut meshes, &scaler);
    }
}

pub(crate) fn update_timing(mut ctx: ResMut<BracketContext>, diagnostics: Res<Diagnostics>) {
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.measurement() {
            ctx.fps = fps_avg.value.round();
        }
    }

    if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_avg) = frame_time.measurement() {
            ctx.frame_time_ms = frame_time_avg.value.round();
        }
    }
}

pub(crate) fn window_resize(
    mut context: ResMut<BracketContext>,
    resize_event: Res<Events<WindowResized>>,
    mut scaler: ResMut<ScreenScaler>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        scaler.set_screen_size(e.width, e.height);
        if let TerminalScalingMode::ResizeTerminals = context.scaling_mode {
            context.resize_terminals(&scaler);
        }
        scaler.recalculate(context.get_pixel_size(), context.largest_font());
    }
}

pub(crate) fn apply_all_batches(mut context: ResMut<BracketContext>) {
    context.render_all_batches();
}

pub(crate) fn update_mouse_position(
    wnds: Query<(&Window, Option<&PrimaryWindow>)>,
    q_camera: Query<(&Camera, &GlobalTransform), With<BracketCamera>>,
    mut context: ResMut<BracketContext>,
    scaler: Res<ScreenScaler>,
) {
    // Modified from: https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    // Bevy really needs a nicer way to do this
    let (camera, camera_transform) = q_camera.single();
    let wnd = if let RenderTarget::Window(WindowRef::Entity(e)) = camera.target {
        wnds.get(e).ok()
    } else {
        wnds.iter().find(|(_w, primary)| primary.is_some())
    };

    let wnd = if let Some(wnd) = wnd {
        wnd.0
    } else {
        return;
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        let result = (world_pos.x, world_pos.y);

        context.set_mouse_pixel_position(result, &scaler);
    }
}
