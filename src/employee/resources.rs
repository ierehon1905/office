use bevy::prelude::*;
use pid_control::PIDController;

pub(crate) struct GreetTimer(pub(crate) Timer);
pub(crate) struct WorkCheckTimer(pub(crate) Timer);
pub(crate) struct EmployeeController(pub(crate) PIDController);

impl EmployeeController {
    pub fn new() -> Self {
        Self(PIDController::new(1.0, 1.0, 1.0))
    }
}

pub(crate) struct EmployeeDesk {
    pub(crate) employee: Entity,
    pub(crate) desk: Entity,
}

type EmployeeEntity = Entity;
type DeskEntity = Entity;

#[derive(Default)]
pub(crate) struct EmployeeDeskTable(pub(crate) multi_map::MultiMap<EmployeeEntity, DeskEntity, ()>);
