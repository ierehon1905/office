use crate::{employee::plugin::EmployeePlugin, office_block::plugin::OfficeBlockPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Ground;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec2 {
                    x: 0.0,
                    y: -9.8 * 100.0,
                },
                ..Default::default()
            })
            .add_startup_system_set_to_stage(
                StartupStage::PreStartup,
                SystemSet::new()
                    .with_system(setup)
                    .with_system(setup_physics),
            )
            .add_plugin(EmployeePlugin)
            .add_plugin(OfficeBlockPlugin);
    }
}

pub(crate) struct DeskMaterials {
    pub(crate) empty: Handle<ColorMaterial>,
    pub(crate) occupied: Handle<ColorMaterial>,
}

pub(crate) struct GameMaterials {
    pub(crate) employee_material: Handle<ColorMaterial>,
    pub(crate) employee_mesh: Handle<Mesh>,
    pub(crate) office_block_material: Handle<ColorMaterial>,
    pub(crate) office_block_mesh: Handle<Mesh>,
    pub(crate) desk_mesh: Handle<Mesh>,
    pub(crate) desk_materials: DeskMaterials,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let res = GameMaterials {
        employee_material: materials.add(ColorMaterial::from(Color::YELLOW)),
        employee_mesh: meshes.add(shape::Quad::new(Vec2::new(20.0, 100.0)).into()),
        office_block_material: materials.add(ColorMaterial::from(Color::GRAY)),
        office_block_mesh: meshes.add(shape::Quad::new(Vec2::new(1000.0, 300.0)).into()),
        desk_materials: DeskMaterials {
            empty: materials.add(ColorMaterial::from(Color::RED)),
            occupied: materials.add(ColorMaterial::from(Color::BLUE)),
        },
        desk_mesh: meshes.add(shape::Quad::new(Vec2::new(40.0, 40.0)).into()),
    };

    commands.insert_resource(res);
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ground_height = 10.0;
    let ground_width = 1000.0;
    commands
        .spawn_bundle(bevy::sprite::MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(ground_width, ground_height)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            transform: Transform::from_translation(Vec3::new(0.0, -ground_height / 2.0, 0.)),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 5.0));
    /* Create the bouncing ball. */
    // commands
    //     .spawn()
    //     .insert(RigidBody::Dynamic)
    //     .insert(Collider::ball(50.0))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}
