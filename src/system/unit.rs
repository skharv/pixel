use bevy::prelude::*;
use rand::{self, Rng};

use crate::{WIDTH, HEIGHT};
use crate::component;
use crate::bundle;

pub fn spawn(mut command: Commands ) {
    for _ in 0..50000 {
        let px = rand::thread_rng().gen_range(0..WIDTH/2); 
        let py = rand::thread_rng().gen_range(0..HEIGHT/2);

        let vx = rand::thread_rng().gen_range(0..WIDTH)/10;
        let vy = rand::thread_rng().gen_range(0..HEIGHT)/10;

        let cr = rand::thread_rng().gen_range(0..255);
        let cg = rand::thread_rng().gen_range(0..255);
        let cb = rand::thread_rng().gen_range(0..255);

    command.spawn(bundle::PixelBundle{
        position: component::Position {x: px as f32, y: py as f32},
        velocity: component::Velocity {x: vx as f32, y: vy as f32},
        colour: component::Colour { r: cr, g: cg, b: cb, a: 255 },
    });
    }
}

pub fn movement(
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


