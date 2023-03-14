use bevy::prelude::*;
use bevy_pixels::*;
use crate::component;
use crate::WIDTH;

pub fn clear(mut pixels_resource: ResMut<PixelsResource>) {
    let frame = pixels_resource.pixels.get_frame_mut();
    frame.copy_from_slice(&[0x00, 0x00, 0x00, 0x00].repeat(frame.len()/4));
}

pub fn draw(
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
