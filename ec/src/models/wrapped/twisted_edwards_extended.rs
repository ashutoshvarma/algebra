use crate::models::twisted_edwards_extended::{
    GroupAffine as GroupAffineED, GroupProjective as GroupProjectiveED,
};
use crate::models::TEModelParameters;
use crate::{AffineCurve, ProjectiveCurve};

use crate::wrapped::Wrapped;
pub use crate::wrapped::{GroupAffine, GroupProjective};
use ark_ff::PrimeField;
use ark_std::ops::{AddAssign, MulAssign, Sub, SubAssign};

use ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// type _GroupAffine<P> = GroupAffine<GroupAffineED<P>>;

// wrap inherent methods for short weierstrass group affine
impl<P: TEModelParameters> GroupAffine<GroupAffineED<P>> {
    fn scale_by_cofactor(&self) -> GroupProjective<GroupProjectiveED<P>> {
        GroupProjective(GroupAffineED::<P>::scale_by_cofactor(self.wrapped()))
    }

    pub(crate) fn mul_bits(
        &self,
        bits: impl Iterator<Item = bool>,
    ) -> GroupProjective<GroupProjectiveED<P>> {
        GroupProjective(GroupAffineED::<P>::mul_bits(self.wrapped(), bits))
    }

    fn get_point_from_x(x: P::BaseField, greatest: bool) -> Option<Self> {
        match GroupAffineED::<P>::get_point_from_x(x, greatest) {
            Some(v) => Some(GroupAffine(v)),
            None => None,
        }
    }

    pub fn is_on_curve(&self) -> bool {
        GroupAffineED::<P>::is_on_curve(self.wrapped())
    }

    pub fn is_in_correct_subgroup_assuming_on_curve(&self) -> bool {
        GroupAffineED::<P>::is_in_correct_subgroup_assuming_on_curve(self.wrapped())
    }
}

impl<'a, P: TEModelParameters> AddAssign<&'a Self> for GroupAffine<GroupAffineED<P>> {
    fn add_assign(&mut self, other: &'a Self) {
        GroupAffineED::<P>::add_assign(self.mut_wrapped(), other.wrapped())
    }
}

impl<'a, P: TEModelParameters> Sub<&'a Self> for GroupAffine<GroupAffineED<P>> {
    type Output = Self;
    fn sub(self, other: &'a Self) -> Self {
        GroupAffine(GroupAffineED::<P>::sub(*self.wrapped(), other.wrapped()))
    }
}

impl<'a, P: TEModelParameters> SubAssign<&'a Self> for GroupAffine<GroupAffineED<P>> {
    fn sub_assign(&mut self, other: &'a Self) {
        GroupAffineED::<P>::sub_assign(self.mut_wrapped(), other.wrapped())
    }
}

impl<'a, P: TEModelParameters> MulAssign<P::ScalarField> for GroupAffine<GroupAffineED<P>> {
    fn mul_assign(&mut self, other: P::ScalarField) {
        GroupAffineED::<P>::mul_assign(self.mut_wrapped(), other)
    }
}

impl<P: TEModelParameters> Distribution<GroupAffine<GroupAffineED<P>>> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GroupAffine<GroupAffineED<P>> {
        GroupAffine(Standard::sample(self, rng))
    }
}

// ark_ff::impl_additive_ops_from_ref!(_GroupAffine, TEModelParameters);

impl<P: TEModelParameters> core::str::FromStr for GroupAffine<GroupAffineED<P>>
where
    P::BaseField: core::str::FromStr<Err = ()>,
{
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        match GroupAffineED::<P>::from_str(s) {
            Ok(v) => Ok(GroupAffine(v)),
            Err(e) => Err(e),
        }
    }
}
