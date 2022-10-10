use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::{na::RealField, prelude::Velocity};
use pid_control::*;
use rand::random;

use crate::utils::Name;

use super::{components::*, resources::*};

pub(crate) fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<(&Name, &Velocity), With<Employee>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, ..) in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

const DEFAULT_EMPLOYEE_SPEED: f32 = 500.0;

pub(crate) fn move_taskable(
    time: Res<Time>,
    mut controller: ResMut<EmployeeController>,
    // mut timer: ResMut<GreetTimer>,
    mut query: Query<(&mut Velocity, &Transform, &mut Taskable, &Name)>,
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
                        let max_dist = distance.abs();
                        let direction: f32 = if distance.is_sign_positive() {
                            1.0
                        } else {
                            -1.0
                        };
                        velocity.linvel.x = direction * DEFAULT_EMPLOYEE_SPEED;
                        // .clamp(-100.0, 100.0)
                        // .clamp(-max_dist, max_dist);
                    } else {
                        println!("{} reached goal", &name.0);
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
    mut query: Query<&mut Taskable>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut taskable in query.iter_mut() {
            match taskable.task {
                None => {
                    println!("gave task");
                    *taskable = Taskable::walk(Vec2::new((random::<f32>() - 0.5) * 800.0, 0.0));
                }
                _ => {}
            }
        }
        // for mut name in query.iter_mut().filter_map(|x| x.task) {
        //     let a = name.into();
        // }
    }
}
