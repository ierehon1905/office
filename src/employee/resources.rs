use bevy::time::Timer;
use pid_control::PIDController;

pub(crate) struct GreetTimer(pub(crate) Timer);
pub(crate) struct WorkCheckTimer(pub(crate) Timer);
pub(crate) struct EmployeeController(pub(crate) PIDController);

impl EmployeeController {
    pub fn new() -> Self {
        Self(PIDController::new(1.0, 1.0, 1.0))
    }
}
