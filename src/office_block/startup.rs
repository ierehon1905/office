use bevy::{prelude::*, utils::tracing::instrument::WithSubscriber};
use bevy_rapier2d::prelude::*;

use crate::{utils::*, world::GameMaterials};

use super::components::{desk::Desk, office_block::OfficeBlock};

pub(crate) fn add_office_block(mut commands: Commands, handles: Res<GameMaterials>) {
    commands
        .spawn()
        .insert(OfficeBlock::IT_DEPARTMENT)
        .insert(Name("Test block".to_string()))
        .insert_bundle(bevy::sprite::MaterialMesh2dBundle {
            mesh: handles.office_block_mesh.clone_weak().into(),
            material: handles.office_block_material.clone_weak().into(),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                150.0,
                EntitiesZIndex::Office.into(),
            )),
            ..default()
        })
        .with_children(|children| {
            let pid = children.parent_entity().id();
            for i in 0..4 {
                children
                    .spawn()
                    .insert(Desk::new())
                    .insert(Name(format!("Desk {i} of IT dep #{pid}")))
                    .insert_bundle(bevy::sprite::MaterialMesh2dBundle {
                        mesh: handles.desk_mesh.clone_weak().into(),
                        material: handles.desk_materials.empty.clone_weak().into(),
                        transform: Transform::from_translation(Vec3::new(
                            i as f32 * (1000.0 / 4.0) - 500.0,
                            -150.0 + 20.0,
                            1.0,
                        )),
                        ..default()
                    })
                    .insert(Collider::cuboid(10.0, 10.0))
                    .insert(Sensor);
            }
        });
    // .insert(RigidBody::Dynamic)
    // .insert(Collider::cuboid(10.0, 50.0))
    // .insert(CollisionGroups::new(
    //     EMPLOYEE_GROUP,
    //     Group::ALL - EMPLOYEE_GROUP,
    // ))
    // .insert(LockedAxes::ROTATION_LOCKED)
    // .insert(Taskable::default())
    // .insert(Velocity::zero())
}
