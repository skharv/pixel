use bevy::{prelude::*, window::WindowResizeConstraints, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};

use bevy_pixels::*;

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
    .add_plugin(PixelsPlugin {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        ..default()
    })
    .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(system::unit::spawn)
        .add_system_to_stage(PixelsStage::Draw, draw_background)
        .add_system_to_stage(PixelsStage::Draw, draw_pixel.after(draw_background))
        .add_system(system::unit::movement)
        .run();
}

fn draw_background(mut pixels_resource: ResMut<PixelsResource>) {
    let frame = pixels_resource.pixels.get_frame_mut();
    frame.copy_from_slice(&[0x00, 0x00, 0x00, 0x00].repeat(frame.len()/4));
}

fn draw_pixel(
    mut pixels_resource: ResMut<PixelsResource>,
    query: Query<(&component::Position, &component::Colour)>
    ) {
    let frame = pixels_resource.pixels.get_frame_mut();

    for(position, color) in query.iter() {
        let index = ((((position.y as i32) * 4) * WIDTH) + (position.x as i32) * 4) as usize;
        frame[index] = color.r;
        frame[index+1] = color.g;
        frame[index+2] = color.b;
        frame[index+3] = color.g;
    } 
}
