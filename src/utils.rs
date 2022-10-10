use bevy::prelude::*;
use rand::random;

pub fn random_vec2(max: f32) -> Vec2 {
    Vec2 {
        x: (random::<f32>() - 0.5) * max,
        y: (random::<f32>() - 0.5) * max,
    }
}

pub fn vec2_to_vec3(input: &Vec2) -> Vec3 {
    Vec3 {
        x: input.x,
        y: input.y,
        z: 0.0,
    }
}

#[derive(Component)]
pub struct Name(pub String);
