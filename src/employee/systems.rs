use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::{na::RealField, prelude::Velocity};
// use pid_control::*;
use rand::random;

use crate::{office_block::components::desk::Desk, utils::Title};

use super::{
    components::{
        employee::Employee,
        taskable::{TaskType, Taskable},
        *,
    },
    resources::*,
};

const DEFAULT_EMPLOYEE_SPEED: f32 = 500.0;

pub(crate) fn move_taskable(
    time: Res<Time>,
    mut controller: ResMut<EmployeeController>,
    // mut timer: ResMut<GreetTimer>,
    mut query: Query<(&mut Velocity, &Transform, &mut Taskable, &Title)>,
) {
    // println!("Processing {} tasks", query.iter().len());
    for (mut velocity, transform, mut taskable, name) in query.iter_mut() {
        if let Some(task) = &taskable.task {
            // println!("Processing task of {}!", name.0);
            let millis = time.delta().as_millis() as f32;

            match task {
                TaskType::Walk(to) => {
                    let distance = to.x - transform.translation.x;

                    if distance.abs() > 10.0 {
                        let direction: f32 = if distance.is_sign_positive() {
                            1.0
                        } else {
                            -1.0
                        };
                        velocity.linvel.x = direction * DEFAULT_EMPLOYEE_SPEED;

                        // .clamp(-100.0, 100.0)
                        // .clamp(-max_dist, max_dist);
                    } else {
                        // println!("{} reached goal", &name.0);
                        taskable.task = None;
                        velocity.linvel = Vec2::ZERO;
                    }
                }
            }
        }
    }
}

pub(crate) fn make_people_work(
    time: Res<Time>,
    mut timer: ResMut<WorkCheckTimer>,
    mut employee_query: Query<(&mut Taskable, &Transform)>,
    desk_query: Query<&GlobalTransform, With<Desk>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        for (mut taskable, taskable_pos) in employee_query.iter_mut() {
            match taskable.task {
                None => {
                    use rand::seq::IteratorRandom;

                    let desk = desk_query.iter().choose(&mut rng);

                    match desk {
                        Some(pos) => {
                            *taskable = Taskable::walk(pos.translation().xy());
                        }
                        None => {
                            *taskable = Taskable::walk(Vec2::new(
                                (random::<f32>() - 0.5) * 800.0,
                                taskable_pos.translation.y,
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
