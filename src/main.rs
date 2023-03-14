use bevy::{prelude::*, window::WindowResizeConstraints};

use bevy_pixels::*;
use rand::{self, Rng};

mod component;
mod bundle;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

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
    .add_startup_system(spawn_pixel)
        .add_system_to_stage(PixelsStage::Draw, draw_background)
        .add_system_to_stage(PixelsStage::Draw, draw_pixel.after(draw_background))
        .add_system(move_pixel)
        .run();
}

fn spawn_pixel(
    mut command: Commands ) {
    for _ in 0..100000 {
        let px = rand::thread_rng().gen_range(0..WIDTH/2); 
        let py = rand::thread_rng().gen_range(0..HEIGHT/2);

        let vx = rand::thread_rng().gen_range(0..WIDTH)/10;
        let vy = rand::thread_rng().gen_range(0..HEIGHT)/10;

        let r = rand::thread_rng().gen_range(0..255);
        let g = rand::thread_rng().gen_range(0..255);
        let b = rand::thread_rng().gen_range(0..255);

    command.spawn(bundle::PixelBundle{
        position: component::Position {x: px as f32, y: py as f32},
        velocity: component::Velocity {x: vx as f32, y: vy as f32},
        colour: component::Colour { r: r, g: g, b: b, a: 255 },
    });
    }
}

fn move_pixel(
    time: Res<Time>,
    mut query: Query<(&mut component::Position, &mut component::Velocity)>
    ) {
    for (mut position, mut velocity) in query.iter_mut() {
        let proposed_x = position.x + velocity.x * time.delta_seconds();
        let proposed_y = position.y + velocity.y * time.delta_seconds();
        
        if proposed_x >= WIDTH as f32 || proposed_x <= 0. {
            velocity.x = -velocity.x;
            position.x += velocity.x * time.delta_seconds();
        } else {
            position.x = proposed_x;
        }
        if proposed_y >= HEIGHT as f32 || proposed_y <= 0. {
            velocity.y = -velocity.y;
            position.y += velocity.y * time.delta_seconds();
        } else {
            position.y = proposed_y;
        }
    }
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
