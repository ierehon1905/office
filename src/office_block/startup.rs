use bevy::{math::Vec2Swizzles, prelude::*, utils::tracing::instrument::WithSubscriber};
use bevy_rapier2d::prelude::*;

use crate::{
    collisions::{DESK_BLOCK_GROUP, OFFICE_BLOCK_GROUP},
    utils::*,
    world::GameMaterials,
};

use super::components::{desk::Desk, office_block::OfficeBlock};

pub(crate) fn add_initial_office_block(mut commands: Commands, handles: Res<GameMaterials>) {
    add_office_block(&mut commands, handles, Vec2::new(0.0, 0.0));
}

pub(crate) fn add_office_block(commands: &mut Commands, handles: Res<GameMaterials>, pos: Vec2) {
    commands
        .spawn_bundle(OfficeBlockBundle::basic(&handles, Some((pos.x, pos.y))))
        .with_children(|children| {
            let pid = children.parent_entity().id();
            let stride = 1000.0 / 5.0;
            for i in 0..4 {
                children.spawn_bundle(DeskBundle {
                    title: Title(format!("Desk {i} of IT dep #{pid}")),
                    ..DeskBundle::basic(
                        &handles,
                        Some((i as f32 * (stride) - 500.0 + stride, -150.0 + 20.0 + 10.0)),
                    )
                });
            }
        })
        .with_children(|children| {
            children
                .spawn_bundle(bevy::sprite::MaterialMesh2dBundle {
                    mesh: handles.floor_mesh.clone_weak().into(),
                    material: handles.floor_materials.clone_weak().into(),
                    transform: Transform::from_translation(Vec3::new(0.0, -150.0 + 5.0, 1.0)),
                    ..default()
                })
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(500.0, 5.0));
        });
}

#[derive(Bundle)]
struct OfficeBlockBundle {
    office_block: OfficeBlock,
    title: Title,
    collider: Collider,
    sensor: Sensor,
    collision_groups: CollisionGroups,
    #[bundle]
    mesh: bevy::sprite::MaterialMesh2dBundle<ColorMaterial>,
}

impl OfficeBlockBundle {
    fn basic(handles: &Res<GameMaterials>, pos: Option<(f32, f32)>) -> Self {
        Self {
            office_block: OfficeBlock::IT_DEPARTMENT,
            title: Title("Office block".to_owned()),
            sensor: Sensor,
            collider: Collider::cuboid(500.0, 150.0),
            collision_groups: CollisionGroups::new(OFFICE_BLOCK_GROUP, Group::ALL),
            mesh: bevy::sprite::MaterialMesh2dBundle {
                mesh: handles.office_block_mesh.clone_weak().into(),
                material: handles.office_block_material.clone_weak().into(),
                transform: Transform::from_translation(Vec3::new(
                    pos.unwrap_or_default().0,
                    pos.unwrap_or_default().1,
                    1.0,
                )),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
struct DeskBundle {
    title: Title,
    desk: Desk,
    collider: Collider,
    sensor: Sensor,
    collision_groups: CollisionGroups,
    #[bundle]
    mesh: bevy::sprite::MaterialMesh2dBundle<ColorMaterial>,
}

impl DeskBundle {
    fn basic(handles: &Res<GameMaterials>, pos: Option<(f32, f32)>) -> Self {
        Self {
            title: Title("Office block".to_owned()),
            collider: Collider::cuboid(20.0, 20.0),
            desk: Desk::new(),
            sensor: Sensor,
            collision_groups: CollisionGroups::new(DESK_BLOCK_GROUP, Group::ALL),
            mesh: bevy::sprite::MaterialMesh2dBundle {
                mesh: handles.desk_mesh.clone_weak().into(),
                material: handles.desk_materials.empty.clone_weak().into(),
                transform: Transform::from_translation(Vec3::new(
                    pos.unwrap_or_default().0,
                    pos.unwrap_or_default().1,
                    2.0,
                )),
                ..default()
            },
        }
    }
}
