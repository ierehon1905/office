use bevy_rapier2d::prelude::Group;

pub const GROUND_GROUP: Group = Group::GROUP_1;
pub const EMPLOYEE_GROUP: Group = Group::GROUP_2;
pub const OFFICE_BLOCK_GROUP: Group = Group::GROUP_3;
pub const DESK_BLOCK_GROUP: Group = Group::GROUP_4;

pub(crate) fn to_interaction_group(
    collision_group: Group,
) -> bevy_rapier2d::rapier::geometry::Group {
    bevy_rapier2d::rapier::geometry::Group::from_bits(collision_group.bits()).unwrap()
}
