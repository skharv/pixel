use bevy::prelude::*;

use crate::component;

#[derive(Bundle)]
pub struct UnitBundle {
    pub facing: component::Facing,
    pub turn_rate: component::TurnRate,
    pub sight_range: component::SightRange,
    pub position: component::Position,
    pub velocity: component::Velocity,
    pub colour: component::Colour,
    pub target: component::Target,
}
