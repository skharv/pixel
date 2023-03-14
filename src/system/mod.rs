use bevy::prelude::*;
use bevy_pixels::*;
use crate::{WIDTH, HEIGHT};

mod unit;
mod pixel;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PixelsPlugin {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            ..default()
        })
        .add_system_to_stage(PixelsStage::Draw, pixel::clear)
        .add_system_to_stage(PixelsStage::Draw, pixel::draw.after(pixel::clear))
        .add_startup_system(unit::spawn)
        .add_system(unit::movement)
        .run();
    }
}
