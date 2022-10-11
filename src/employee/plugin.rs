use bevy::prelude::*;

use super::{
    resources::{EmployeeController, GreetTimer, WorkCheckTimer},
    startup::add_people,
    systems::{make_people_work, move_taskable},
};

pub(crate) struct EmployeePlugin;

impl Plugin for EmployeePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .insert_resource(WorkCheckTimer(Timer::from_seconds(0.5, true)))
            .insert_resource(EmployeeController::new())
            .add_startup_system_to_stage(StartupStage::Startup, add_people)
            .add_system(make_people_work)
            .add_system(move_taskable);
    }
}
