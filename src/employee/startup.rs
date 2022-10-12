use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{collisions::*, utils::*, world::GameMaterials};

use super::components::{employee::Employee, taskable::Taskable, *};

const INITIAL_EMPLOYEE_COUNT: i32 = 3;

pub(crate) fn add_people(mut commands: Commands, handles: Res<GameMaterials>) {
    for i in 0..INITIAL_EMPLOYEE_COUNT {
        add_person(&mut commands, &handles, &format!("Employee #{}", i + 1));
    }
}

fn add_person(commands: &mut Commands, handles: &Res<GameMaterials>, name: &str) {
    let mut pos = vec2_to_vec3(&random_vec2(100.0));
    pos.y += 100.0;
    pos.z = EntitiesZIndex::Employee.into();

    commands
        .spawn()
        .insert(Employee)
        .insert(Title(name.to_string()))
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
