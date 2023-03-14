use bevy::prelude::*;

use crate::component;

#[derive(Bundle)]
pub struct UnitBundle {
    pub angle: component::Angle,
    pub position: component::Position,
    pub velocity: component::Velocity,
    pub colour: component::Colour,
}
