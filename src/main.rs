mod camera;
mod collisions;
mod utils;
mod world;
mod employee;

use bevy::prelude::*;
use camera::CustomCameraPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldPlugin)
        .add_plugin(CustomCameraPlugin {})
        .run();

    // let a = Point
}
