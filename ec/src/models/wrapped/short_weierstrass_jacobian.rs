use crate::models::short_weierstrass_jacobian::{
    GroupAffine as GroupAffineSW, GroupProjective as GroupProjectiveSW,
};
use crate::models::SWModelParameters;

use crate::wrapped::Wrapped;
pub use crate::wrapped::{GroupAffine, GroupProjective};
use ark_std::ops::AddAssign;

// wrap inherent methods for short weierstrass group affine
impl<P: SWModelParameters> GroupAffine<GroupAffineSW<P>> {
    #[allow(dead_code)]
    fn scale_by_cofactor(&self) -> GroupProjective<GroupProjectiveSW<P>> {
        GroupProjective(GroupAffineSW::<P>::scale_by_cofactor(self.wrapped()))
    }

    #[allow(dead_code)]
    pub(crate) fn mul_bits(
        &self,
        bits: impl Iterator<Item = bool>,
    ) -> GroupProjective<GroupProjectiveSW<P>> {
        GroupProjective(GroupAffineSW::<P>::mul_bits(self.wrapped(), bits))
    }

    #[allow(dead_code)]
    fn get_point_from_x(x: P::BaseField, greatest: bool) -> Option<Self> {
        match GroupAffineSW::<P>::get_point_from_x(x, greatest) {
            Some(v) => Some(GroupAffine(v)),
            None => None,
        }
    }

    pub fn is_on_curve(&self) -> bool {
        GroupAffineSW::<P>::is_on_curve(self.wrapped())
    }

    pub fn is_in_correct_subgroup_assuming_on_curve(&self) -> bool {
        GroupAffineSW::<P>::is_in_correct_subgroup_assuming_on_curve(self.wrapped())
    }
}

impl<'a, P: SWModelParameters> AddAssign<&'a Self> for GroupAffine<GroupAffineSW<P>> {
    fn add_assign(&mut self, other: &'a Self) {
        GroupAffineSW::add_assign(self.mut_wrapped(), other.wrapped())
    }
}
