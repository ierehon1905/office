use bevy::prelude::*;

use super::{
    resources::DeskAssignTimer,
    startup::add_office_block,
    systems::{assign_employees_to_desks, notify_employee_reach_desk},
};

pub(crate) struct OfficeBlockPlugin;

impl Plugin for OfficeBlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::Startup, add_office_block)
            .insert_resource(DeskAssignTimer(Timer::from_seconds(5.0, true)))
            .add_system(notify_employee_reach_desk)
            .add_system(assign_employees_to_desks);
    }
}
