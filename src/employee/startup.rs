use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{collisions::*, utils::*, world::GameMaterials};

use super::components::*;

pub(crate) fn add_people(mut commands: Commands, handles: Res<GameMaterials>) {
    add_person(&mut commands, &handles, "Elaina Proctor");
    add_person(&mut commands, &handles, "Renzo Hume");
    add_person(&mut commands, &handles, "Zayna Nieves");
}

fn add_person(commands: &mut Commands, handles: &Res<GameMaterials>, name: &str) {
    let mut pos = vec2_to_vec3(&random_vec2(100.0));
    pos.y += 100.0;

    commands
        .spawn()
        .insert(Employee)
        .insert(Name(name.to_string()))
        .insert_bundle(bevy::sprite::MaterialMesh2dBundle {
            mesh: handles.employee_mesh.clone_weak().into(),
            material: handles.employee_material.clone_weak().into(),
            transform: Transform::from_translation(pos),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(10.0, 50.0))
        .insert(CollisionGroups::new(
            EMPLOYEE_GROUP,
            Group::ALL - EMPLOYEE_GROUP,
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Taskable::default())
        .insert(Velocity::zero())
        // .insert(Accel)
        // .insert(GravityScale(0.0))
        ;
}
