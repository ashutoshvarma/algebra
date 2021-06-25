use ark_ec::models::twisted_edwards_extended::{
    GroupAffine as GroupAffineED, GroupProjective as GroupProjectiveED,
};
use ark_ec::models::TEModelParameters;

pub use crate::wrapped::{GroupAffine, GroupProjective, Wrapped};
// use ark_ff::PrimeField;
use ark_std::ops::{Add, AddAssign, MulAssign, Sub, SubAssign};

use ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use ark_ff::Zero;

// wrap inherent methods for short weierstrass group affine
impl<P: TEModelParameters> GroupAffine<GroupAffineED<P>> {
    #[allow(dead_code)]
    fn scale_by_cofactor(&self) -> GroupProjective<GroupProjectiveED<P>> {
        GroupProjective(GroupAffineED::<P>::scale_by_cofactor(self.wrapped()))
    }

    // #[allow(dead_code)]
    // pub(crate) fn mul_bits(
    //     &self,
    //     bits: impl Iterator<Item = bool>,
    // ) -> GroupProjective<GroupProjectiveED<P>> {
    //     GroupProjective(GroupAffineED::<P>::mul_bits(self.wrapped(), bits))
    // }

    #[allow(dead_code)]
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

impl<'a, P: TEModelParameters> Add<&'a Self> for GroupAffine<GroupAffineED<P>> {
    type Output = Self;
    fn add(self, other: &'a Self) -> Self {
        GroupAffine(GroupAffineED::<P>::add(*self.wrapped(), other.wrapped()))
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

impl<P: TEModelParameters> core::str::FromStr for GroupAffine<GroupAffineED<P>>
where
    P::BaseField: core::str::FromStr<Err = ()>,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match GroupAffineED::<P>::from_str(s) {
            Ok(v) => Ok(GroupAffine(v)),
            Err(e) => Err(e),
        }
    }
}

// // ark_ff::impl_additive_ops_from_ref!(_GroupAffine, TEModelParameters);
// `Add` in above macro conflicts as its already been impl for generic GroupAffine
// So we have to manually impl rest of traits here

#[allow(unused_qualifications)]
impl<'a, P: TEModelParameters> core::ops::Add<&'a mut Self> for GroupAffine<GroupAffineED<P>> {
    type Output = Self;

    #[inline]
    fn add(self, other: &'a mut Self) -> Self {
        let mut result = self;
        result.add_assign(&*other);
        result
    }
}

#[allow(unused_qualifications)]
impl<P: TEModelParameters> core::ops::Sub<Self> for GroupAffine<GroupAffineED<P>> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        let mut result = self;
        result.sub_assign(&other);
        result
    }
}

#[allow(unused_qualifications)]
impl<'a, P: TEModelParameters> core::ops::Sub<&'a mut Self> for GroupAffine<GroupAffineED<P>> {
    type Output = Self;

    #[inline]
    fn sub(self, other: &'a mut Self) -> Self {
        let mut result = self;
        result.sub_assign(&*other);
        result
    }
}

#[allow(unused_qualifications)]
impl<P: TEModelParameters> core::iter::Sum<Self> for GroupAffine<GroupAffineED<P>> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), core::ops::Add::add)
    }
}

#[allow(unused_qualifications)]
impl<'a, P: TEModelParameters> core::iter::Sum<&'a Self> for GroupAffine<GroupAffineED<P>> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), core::ops::Add::add)
    }
}

#[allow(unused_qualifications)]
impl<P: TEModelParameters> core::ops::AddAssign<Self> for GroupAffine<GroupAffineED<P>> {
    fn add_assign(&mut self, other: Self) {
        self.add_assign(&other)
    }
}

#[allow(unused_qualifications)]
impl<P: TEModelParameters> core::ops::SubAssign<Self> for GroupAffine<GroupAffineED<P>> {
    fn sub_assign(&mut self, other: Self) {
        self.sub_assign(&other)
    }
}

#[allow(unused_qualifications)]
impl<'a, P: TEModelParameters> core::ops::AddAssign<&'a mut Self>
    for GroupAffine<GroupAffineED<P>>
{
    fn add_assign(&mut self, other: &'a mut Self) {
        self.add_assign(&*other)
    }
}

#[allow(unused_qualifications)]
impl<'a, P: TEModelParameters> core::ops::SubAssign<&'a mut Self>
    for GroupAffine<GroupAffineED<P>>
{
    fn sub_assign(&mut self, other: &'a mut Self) {
        self.sub_assign(&*other)
    }
}

mod group_impl {
    use super::*;
    use ark_ec::group::Group;

    impl<P: TEModelParameters> Group for GroupAffine<GroupAffineED<P>> {
        type ScalarField = P::ScalarField;

        #[inline]
        fn double(&self) -> Self {
            GroupAffine(self.wrapped().double())
        }

        #[inline]
        fn double_in_place(&mut self) -> &mut Self {
            // let mut tmp = *self;
            // tmp += &*self;
            // *self = tmp;
            // self
            self.mut_wrapped().double_in_place();
            self
        }
    }
}
