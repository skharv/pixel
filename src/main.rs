use bevy::{prelude::*, window::WindowResizeConstraints, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};


mod component;
mod bundle;
mod system;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 600;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Pixels".to_string(),
            width: WIDTH as f32,
            height: HEIGHT as f32,
            resize_constraints: WindowResizeConstraints {
                min_width: WIDTH as f32,
                min_height: HEIGHT as f32,
                ..default()
            },
            fit_canvas_to_parent: true,
            ..default()
        },
        ..default()
    }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(system::GamePlugin)
        .run();
}

