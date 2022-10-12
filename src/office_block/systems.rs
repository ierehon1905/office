use bevy::{ecs::query::QueryCombinationIter, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::camera::CursorPosition;
use crate::collisions::{to_interaction_group, OFFICE_BLOCK_GROUP};
use crate::employee::components::employee::Employee;

use crate::employee::resources::EmployeeDeskTable;
use crate::office_block::startup::add_office_block;
use crate::utils::Title;
use crate::world::GameMaterials;

use super::components::desk::Desk;
use super::components::office_block::OfficeBlock;
use super::resources::DeskAssignTimer;

pub(crate) fn notify_employee_reach_desk(
    employees: Query<(&Title, Entity), With<Employee>>,
    desks: Query<(&Title, Entity), With<Desk>>,
    rapier_context: Res<RapierContext>,
) {
    for employee in employees.iter() {
        for desk in desks.iter() {
            if let Some(true) = rapier_context.intersection_pair(employee.1, desk.1) {
                // println!("{} reached desk {}", employee.0, desk.0);
            }
        }
    }
}

pub(crate) fn assign_employees_to_desks(
    mut commands: Commands,
    materials: Res<GameMaterials>,
    time: Res<Time>,
    employees: Query<(&Title, Entity), With<Employee>>,
    mut timer: ResMut<DeskAssignTimer>,
    mut desks: Query<(&Title, &mut Desk, &mut Handle<ColorMaterial>, Entity)>,
    mut employee_desk: ResMut<EmployeeDeskTable>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for employee in employees.iter() {
            for mut desk in desks.iter_mut() {
                if employee_desk.0.contains_key(&employee.1)
                    || employee_desk.0.contains_key_alt(&desk.3)
                {
                    continue;
                }
                println!("Assigned {} to desk {}", employee.0, desk.0);
                employee_desk.0.insert(employee.1, desk.3, ());
                *desk.2 = materials.desk_materials.occupied.clone_weak();
            }
        }
    }
}

pub(crate) fn create_office_block(
    cursor_pos: Res<CursorPosition>,
    mut commands: Commands,
    btn: Res<Input<MouseButton>>,
    handle: Res<GameMaterials>,
    query: Query<(&Transform, &Title, Entity), With<OfficeBlock>>,
    rapier_context: Res<RapierContext>,
) {
    if btn.just_pressed(MouseButton::Left) {
        // if btn.pressed(MouseButton::Left) {
        // dbg!("Just pressed left");
        dbg!(cursor_pos.0);
        if let Some(pos) = cursor_pos.0 {
            let mut rounded_pos = pos.clone();
            rounded_pos.y = (rounded_pos.y / 300.0).round() * 300.0;
            let possible_collider = Collider::cuboid(500.0, 150.0 - 2.0);

            let possible_intersection = rapier_context.intersection_with_shape(
                rounded_pos,
                0.0,
                &possible_collider,
                QueryFilter::new().groups(InteractionGroups::new(
                    to_interaction_group(OFFICE_BLOCK_GROUP),
                    to_interaction_group(OFFICE_BLOCK_GROUP),
                )),
            );

            match possible_intersection {
                Some(intersection) => match query.get(intersection) {
                    Ok(office_block) => {
                        println!("Office block {} is intersection", office_block.1);
                    }
                    Err(_) => {
                        println!("There is unknown intersection {}", intersection.id());
                    }
                },
                None => add_office_block(&mut commands, handle, rounded_pos),
            }
            // ;
        }
    }
}
