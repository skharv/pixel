use bevy::prelude::*;

use crate::component;

#[derive(Bundle)]
pub struct PixelBundle {
    pub position: component::Position,
    pub velocity: component::Velocity,
    pub colour: component::Colour,
}
