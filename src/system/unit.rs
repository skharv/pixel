use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{self, Rng};

use crate::{WIDTH, HEIGHT};
use crate::component;
use crate::bundle;

pub fn spawn(mut command: Commands ) {
    for _ in 0..10000 {
        let px = WIDTH/2;//rand::thread_rng().gen_range(0..WIDTH/2); 
        let py = HEIGHT/2;//rand::thread_rng().gen_range(0..HEIGHT/2);
                          
        let tx = rand::thread_rng().gen_range(0..WIDTH);
        let ty = rand::thread_rng().gen_range(0..HEIGHT);

        let vx = (rand::thread_rng().gen_range(10..(WIDTH*100))as f32)/1000.;
        let vy = (rand::thread_rng().gen_range(10..(WIDTH*100))as f32)/1000.;

        let cr = rand::thread_rng().gen_range(0..255);
        let cg = rand::thread_rng().gen_range(0..255);
        let cb = rand::thread_rng().gen_range(0..255);

        let a = rand::thread_rng().gen_range(0..360) as f32;
    command.spawn(bundle::UnitBundle{
        facing: component::Facing {angle: f32::to_radians(a)},
        turn_rate: component::TurnRate {angle: 45.},
        sight_range: component::SightRange {value: 100.},
        position: component::Position {x: px as f32, y: py as f32},
        velocity: component::Velocity {x: vx as f32, y: vy as f32},
        colour: component::Colour { r: cr, g: cg, b: cb, a: 255 },
        target: component::Target {x : tx as f32, y: ty as f32},
    });
    }
}

pub fn movement(
    time: Res<Time>,
    mut query: Query<(&mut component::Facing, &mut component::Position, &mut component::Velocity)>
    ) {
    for (facing, mut position, mut velocity) in query.iter_mut() {
        let v = Vec2::from_angle(facing.angle);

        let proposed_x = position.x + velocity.x * v.x * time.delta_seconds();
        let proposed_y = position.y + velocity.y * v.y * time.delta_seconds();
        
        if proposed_x >= WIDTH as f32 || proposed_x <= 0. {
            velocity.x = -velocity.x;
            position.x += velocity.x * v.x * time.delta_seconds();
        } else {
            position.x = proposed_x;
        }
        if proposed_y >= HEIGHT as f32 || proposed_y <= 0. {
            velocity.y = -velocity.y;
            position.y += velocity.y * v.y * time.delta_seconds();
        } else {
            position.y = proposed_y;
        }
    }
}

pub fn turn(
    time: Res<Time>,
    mut query: Query<(&mut component::SightRange, &mut component::Position, &mut component::TurnRate, &mut component::Facing, &mut component::Target)>
    ) {
    for (sight, position, turn_rate, mut facing, target) in query.iter_mut() {
        let ang = Vec2::from_angle(facing.angle);

        //TURN TO TARGET
        let relative_pos = Vec2::new(target.x - position.x, target.y - position.y);
        let tang = f32::atan2(relative_pos.y, relative_pos.x) * 180. / PI;
        if tang > facing.angle {
            facing.angle += turn_rate.angle * time.delta_seconds();
        }
        if tang < facing.angle {
            facing.angle -= turn_rate.angle * time.delta_seconds();
        }
    }
}
