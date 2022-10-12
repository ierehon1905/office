use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Employee;

pub(crate) enum Genders {
    Male,
    Female,
    NA,
}
#[derive(Component)]
pub(crate) struct Gender(Genders);

impl Gender {
    pub(crate) const MALE: Self = Self(Genders::Male);
    pub(crate) const Female: Self = Self(Genders::Female);
}


