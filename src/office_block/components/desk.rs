use bevy::prelude::*;

pub(crate) enum Cleanness {
    Clean,
    SomewhatDirty,
    Dirty,
    Mess,
}

impl Default for Cleanness {
    fn default() -> Self {
        Cleanness::Clean
    }
}

#[derive(Component, Default)]
pub(crate) struct Desk {
    pub(crate) cleanness: Cleanness,
}

impl Desk {
    pub(crate) fn new() -> Self {
        Desk::default()
    }
}
