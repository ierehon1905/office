use bevy::{ecs::query::QueryCombinationIter, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::employee::components::employee::Employee;
use crate::utils::Name;
use crate::world::GameMaterials;

use super::components::desk::Desk;
use super::resources::DeskAssignTimer;

pub(crate) fn notify_employee_reach_desk(
    employees: Query<(&Name, Entity), With<Employee>>,
    desks: Query<(&Name, Entity), With<Desk>>,
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
    employees: Query<(&Name, Entity), With<Employee>>,
    mut desks: Query<(&Name, &mut Desk, &mut Handle<ColorMaterial>, Entity)>,
    time: Res<Time>,
    mut timer: ResMut<DeskAssignTimer>,
    materials: Res<GameMaterials>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for employee in employees.iter() {
            for mut desk in desks.iter_mut() {
                if desk.1.occupant.is_none() {
                    *desk.2 = materials.desk_materials.occupied.clone_weak();
                    // println!("{}");
                    desk.1.occupant = Some(employee.1);
                }
            }
        }
    }
}
