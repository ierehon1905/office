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

#[derive(Component, Debug)]
pub(crate) struct Title(pub String);

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) enum EntitiesZIndex {
    Background,
    Ground,
    Office,
    Misc,
    Employee,
}

impl From<EntitiesZIndex> for f32 {
    fn from(z_index: EntitiesZIndex) -> Self {
        z_index as u32 as f32
    }
}
