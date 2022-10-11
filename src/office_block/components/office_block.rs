use bevy::prelude::*;

pub(crate) enum OfficeBlockTye {
    ItDepartment,
    Janitors,
    BackOffice,
    Supervisor,
}

#[derive(Component)]
pub struct OfficeBlock {
    pub(crate) block_type: OfficeBlockTye,
}

impl OfficeBlock {
    pub(crate) const fn new(block_type: OfficeBlockTye) -> Self {
        Self { block_type }
    }

    pub(crate) const IT_DEPARTMENT: Self = Self::new(OfficeBlockTye::ItDepartment);
    pub(crate) const JANITOR: Self = Self::new(OfficeBlockTye::Janitors);
    pub(crate) const BACK_OFFICE: Self = Self::new(OfficeBlockTye::BackOffice);
    pub(crate) const SUPERVISOR: Self = Self::new(OfficeBlockTye::Supervisor);
}
