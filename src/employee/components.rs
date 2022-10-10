use bevy::prelude::*;

use crate::utils::Name;

#[derive(Component)]
pub struct Employee;

#[derive(Component, Default)]

pub(crate) struct Taskable {
    pub(crate) task: Option<TaskType>,
}

pub(crate) enum TaskType {
    Walk(Vec2),
}

impl Taskable {
    pub fn new(task: Option<TaskType>) -> Self {
        Self { task }
    }

    pub fn walk(to: Vec2) -> Self {
        Self {
            task: Some(TaskType::Walk(to)),
        }
    }
}
