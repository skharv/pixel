use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Component)]
pub struct TurnRate {
    pub angle: f32,
}

#[derive(Component)]
pub struct Facing {
    pub angle: f32,
}

#[derive(Component)]
pub struct Target {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Hunger {
    pub value: f32,
}

#[derive(Component)]
pub struct Anger {
    pub value: f32,
}

#[derive(Component)]
pub struct SightRange {
    pub value: f32,
}
