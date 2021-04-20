use crate::{AffineCurve, ProjectiveCurve};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};

use ark_ff::{
    bytes::{FromBytes, ToBytes},
    fields::PrimeField,
};

use ark_std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Read, Result as IoResult, Write},
    ops::{Add, AddAssign, MulAssign, Neg, Sub, SubAssign},
};

use num_traits::Zero;
use zeroize::Zeroize;

use ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use derive_more::Display;

pub mod short_weierstrass_jacobian;
pub mod twisted_edwards_extended;

// Helper Trait for wrapping types
pub trait Wrapped: Sized {
    type WrapTarget;
    fn wrapped(&self) -> &Self::WrapTarget;
    fn mut_wrapped(&mut self) -> &mut Self::WrapTarget;
    fn set_wrapped(&mut self, warpped: Self::WrapTarget);
    fn new_wrap(inner: Self::WrapTarget) -> Self;
}

impl<C: AffineCurve> Wrapped for GroupAffine<C> {
    type WrapTarget = C;
    fn wrapped(&self) -> &Self::WrapTarget {
        &(self.0)
    }
    fn mut_wrapped(&mut self) -> &mut Self::WrapTarget {
        &mut (self.0)
    }
    fn set_wrapped(&mut self, wrapped: Self::WrapTarget) {
        self.0 = wrapped
    }
    fn new_wrap(inner: Self::WrapTarget) -> Self {
        Self(inner)
    }
}

impl<C: ProjectiveCurve> Wrapped for GroupProjective<C> {
    type WrapTarget = C;
    fn wrapped(&self) -> &Self::WrapTarget {
        &(self.0)
    }
    fn mut_wrapped(&mut self) -> &mut Self::WrapTarget {
        &mut (self.0)
    }
    fn set_wrapped(&mut self, wrapped: Self::WrapTarget) {
        self.0 = wrapped
    }
    fn new_wrap(inner: Self::WrapTarget) -> Self {
        Self(inner)
    }
}

// #######################################################################
#[derive(Derivative, PartialEq, Eq)]
#[derivative(
    Copy(bound = "C: AffineCurve"),
    Clone(bound = "C: AffineCurve"),
    Debug(bound = "C: AffineCurve"),
    Hash(bound = "C: AffineCurve"),
    // PartialEq(bound = "C: AffineCurve"),
    // Eq(bound = "C: AffineCurve")
)]
pub struct GroupAffine<C: AffineCurve>(pub C);

impl<C: AffineCurve> Display for GroupAffine<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        <C as Display>::fmt(self.wrapped(), f)
    }
}

impl<A: AffineCurve, P: ProjectiveCurve> PartialEq<GroupProjective<P>> for GroupAffine<A>
where
    A: PartialEq<P>,
{
    fn eq(&self, other: &GroupProjective<P>) -> bool {
        // self.into_projective() == *other
        A::eq(self.wrapped(), other.wrapped())
    }
}

impl<A: AffineCurve, P: ProjectiveCurve> PartialEq<GroupAffine<A>> for GroupProjective<P>
where
    P: PartialEq<A>,
{
    fn eq(&self, other: &GroupAffine<A>) -> bool {
        P::eq(self.wrapped(), other.wrapped())
    }
}

impl<C: AffineCurve> GroupAffine<C> {
    fn new(inner: C) -> Self {
        GroupAffine(inner)
    }
}

impl<C: AffineCurve> Zeroize for GroupAffine<C> {
    fn zeroize(&mut self) {
        C::zeroize(self.mut_wrapped())
    }
}

impl<C: AffineCurve> Zero for GroupAffine<C> {
    #[inline]
    fn zero() -> Self {
        GroupAffine(C::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        C::is_zero(self.wrapped())
    }
}

impl<C: AffineCurve> Add<Self> for GroupAffine<C> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        GroupAffine(C::add(*self.wrapped(), *other.wrapped()))
    }
}

// impl<'a, C: AffineCurve + AddAssign> AddAssign<&'a Self> for GroupAffine<C> {
//     fn add_assign(&mut self, other: &'a Self) {
//         C::add_assign(self.mut_wrapped(), other.wrapped())
//     }
// }

impl<C: AffineCurve> AffineCurve for GroupAffine<C>
where
    Standard: Distribution<C::Projective>,
{
    const COFACTOR: &'static [u64] = C::COFACTOR;
    type BaseField = C::BaseField;
    type ScalarField = C::ScalarField;
    type Projective = GroupProjective<C::Projective>;

    #[inline]
    fn prime_subgroup_generator() -> Self {
        GroupAffine(C::prime_subgroup_generator())
    }

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        match C::from_random_bytes(bytes) {
            Some(v) => Some(GroupAffine(v)),
            None => None,
        }
    }

    #[inline]
    fn mul<S: Into<<Self::ScalarField as PrimeField>::BigInt>>(&self, by: S) -> Self::Projective {
        GroupProjective(C::mul(self.wrapped(), by))
    }

    #[inline]
    fn mul_by_cofactor_to_projective(&self) -> Self::Projective {
        GroupProjective(C::mul_by_cofactor_to_projective(self.wrapped()))
    }

    fn mul_by_cofactor_inv(&self) -> Self {
        GroupAffine(C::mul_by_cofactor_inv(self.wrapped()))
    }
}

impl<C: AffineCurve> Neg for GroupAffine<C> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        GroupAffine(C::neg(*self.wrapped()))
    }
}

impl<C: AffineCurve> ToBytes for GroupAffine<C> {
    #[inline]
    fn write<W: Write>(&self, mut writer: W) -> IoResult<()> {
        C::write(self.wrapped(), writer)
    }
}

impl<C: AffineCurve> FromBytes for GroupAffine<C> {
    #[inline]
    fn read<R: Read>(mut reader: R) -> IoResult<Self> {
        match C::read(reader) {
            Ok(v) => Ok(GroupAffine(v)),
            Err(e) => Err(e),
        }
    }
}

impl<C: AffineCurve> Default for GroupAffine<C> {
    #[inline]
    fn default() -> Self {
        GroupAffine(C::default())
    }
}

// GroupProjective STARTs
#[derive(Eq, PartialEq, Display, Derivative)]
#[derivative(
    Copy(bound = "C: ProjectiveCurve"),
    Clone(bound = "C: ProjectiveCurve"),
    Debug(bound = "C: ProjectiveCurve"),
    Hash(bound = "C: ProjectiveCurve")
)]
pub struct GroupProjective<C: ProjectiveCurve>(pub C);

impl<C: ProjectiveCurve> Distribution<GroupProjective<C>> for Standard
where
    Standard: Distribution<C>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GroupProjective<C> {
        GroupProjective(Standard::sample(self, rng))
    }
}

impl<C: ProjectiveCurve + ToBytes> ToBytes for GroupProjective<C> {
    #[inline]
    fn write<W: Write>(&self, mut writer: W) -> IoResult<()> {
        // C::write(&self.0, writer)
        self.wrapped().write(writer)
    }
}

impl<C: ProjectiveCurve + FromBytes> FromBytes for GroupProjective<C> {
    #[inline]
    fn read<R: Read>(mut reader: R) -> IoResult<Self> {
        match C::read(reader) {
            Ok(v) => Ok(GroupProjective(v)),
            Err(e) => Err(e),
        }
    }
}

impl<C: ProjectiveCurve + Default> Default for GroupProjective<C> {
    #[inline]
    fn default() -> Self {
        GroupProjective(C::default())
    }
}

impl<C: ProjectiveCurve> GroupProjective<C> {
    pub fn new(inner: C) -> Self {
        GroupProjective(inner)
    }
}

impl<C: ProjectiveCurve> Zeroize for GroupProjective<C> {
    fn zeroize(&mut self) {
        // self.mut_wrapped().zeroize()
        C::zeroize(&mut self.mut_wrapped())
    }
}

impl<C: ProjectiveCurve> Zero for GroupProjective<C> {
    #[inline]
    fn zero() -> Self {
        GroupProjective(C::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        // self.wrapped().is_zero()
        C::is_zero(self.wrapped())
    }
}

impl<C: ProjectiveCurve> ProjectiveCurve for GroupProjective<C>
where
    Standard: Distribution<C>,
    // Self: From<C::Affine>
{
    const COFACTOR: &'static [u64] = C::COFACTOR;
    type BaseField = C::BaseField;
    type ScalarField = C::ScalarField;
    type Affine = GroupAffine<C::Affine>;

    #[inline]
    fn prime_subgroup_generator() -> Self {
        GroupProjective(C::prime_subgroup_generator())
    }

    #[inline]
    fn is_normalized(&self) -> bool {
        // self.wrapped().is_normalized()
        C::is_normalized(self.wrapped())
    }

    // Take inner type
    #[inline]
    fn batch_normalization(v: &mut [Self]) {
        // C::batch_normalization(v)
    }

    fn double_in_place(&mut self) -> &mut Self {
        // self.mut_wrapped().double_in_place()
        C::double_in_place(self.mut_wrapped());
        self
    }

    fn add_assign_mixed(&mut self, other: &GroupAffine<C::Affine>) {
        C::add_assign_mixed(self.mut_wrapped(), other.wrapped())
    }
}

impl<C: ProjectiveCurve> Neg for GroupProjective<C> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        GroupProjective(self.wrapped().neg())
    }
}

// common to both sw and ed
ark_ff::impl_additive_ops_from_ref!(GroupProjective, ProjectiveCurve);

impl<'a, C: ProjectiveCurve> Add<&'a Self> for GroupProjective<C> {
    type Output = Self;
    fn add(mut self, other: &'a Self) -> Self {
        // GroupProjective(self.wrapped().add(other))
        GroupProjective(C::add(*self.mut_wrapped(), other.wrapped()))
    }
}

impl<'a, C: ProjectiveCurve> AddAssign<&'a Self> for GroupProjective<C> {
    fn add_assign(&mut self, other: &'a Self) {
        // self.mut_wrapped().add_assign(other.wrapped())
        C::add_assign(self.mut_wrapped(), other.wrapped())
    }
}

impl<'a, C: ProjectiveCurve> Sub<&'a Self> for GroupProjective<C> {
    type Output = Self;
    fn sub(mut self, other: &'a Self) -> Self {
        // GroupProjective(self.wrapped().sub(other))
        GroupProjective(C::sub(*self.mut_wrapped(), other.wrapped()))
    }
}

impl<'a, C: ProjectiveCurve> SubAssign<&'a Self> for GroupProjective<C> {
    fn sub_assign(&mut self, other: &'a Self) {
        // self.mut_wrapped().sub_assign(other.wrapped())
        C::sub_assign(self.mut_wrapped(), other.wrapped())
    }
}

impl<C: ProjectiveCurve> MulAssign<C::ScalarField> for GroupProjective<C> {
    fn mul_assign(&mut self, other: C::ScalarField) {
        self.mut_wrapped().mul_assign(other)
    }
}

// From and Serialize Traits

impl<A: AffineCurve, P: ProjectiveCurve> From<GroupAffine<A>> for GroupProjective<P>
where
    P: From<A>,
{
    fn from(p: GroupAffine<A>) -> GroupProjective<P> {
        GroupProjective(P::from(*p.wrapped()))
    }
}

impl<A: AffineCurve, P: ProjectiveCurve> From<GroupProjective<P>> for GroupAffine<A>
where
    A: From<P>,
{
    fn from(p: GroupProjective<P>) -> GroupAffine<A> {
        GroupAffine(A::from(*p.wrapped()))
    }
}

impl<C: AffineCurve> CanonicalSerialize for GroupAffine<C> {
    #[inline]
    fn serialize<W: Write>(&self, writer: W) -> Result<(), SerializationError> {
        // self.wrapped().serialize()
        C::serialize(self.wrapped(), writer)
    }

    #[inline]
    fn serialized_size(&self) -> usize {
        // self.wrapped().serialized_size()
        C::serialized_size(self.wrapped())
    }

    #[inline]
    fn serialize_uncompressed<W: Write>(&self, mut writer: W) -> Result<(), SerializationError> {
        // self.wrapped().serialize_uncompressed(writer)
        C::serialize_uncompressed(self.wrapped(), writer)
    }

    #[inline]
    fn uncompressed_size(&self) -> usize {
        // self.wrapped().uncompressed_size()
        C::uncompressed_size(self.wrapped())
    }
}

impl<C: ProjectiveCurve> CanonicalSerialize for GroupProjective<C> {
    #[inline]
    fn serialize<W: Write>(&self, writer: W) -> Result<(), SerializationError> {
        // self.wrapped().serialize()
        C::serialize(self.wrapped(), writer)
    }

    #[inline]
    fn serialized_size(&self) -> usize {
        // self.wrapped().serialized_size()
        C::serialized_size(self.wrapped())
    }

    #[inline]
    fn serialize_uncompressed<W: Write>(&self, mut writer: W) -> Result<(), SerializationError> {
        // self.wrapped().serialize_uncompressed(writer)
        C::serialize_uncompressed(self.wrapped(), writer)
    }

    #[inline]
    fn uncompressed_size(&self) -> usize {
        // self.wrapped().uncompressed_size()
        C::uncompressed_size(self.wrapped())
    }
}

impl<C: AffineCurve> CanonicalDeserialize for GroupAffine<C> {
    fn deserialize<R: Read>(reader: R) -> Result<Self, SerializationError> {
        match C::deserialize(reader) {
            Ok(v) => Ok(GroupAffine(v)),
            Err(e) => Err(e),
        }
    }

    fn deserialize_uncompressed<R: Read>(
        reader: R,
    ) -> Result<Self, ark_serialize::SerializationError> {
        match C::deserialize_uncompressed(reader) {
            Ok(v) => Ok(GroupAffine(v)),
            Err(e) => Err(e),
        }
    }

    fn deserialize_unchecked<R: Read>(mut reader: R) -> Result<Self, SerializationError> {
        match C::deserialize_unchecked(reader) {
            Ok(v) => Ok(GroupAffine(v)),
            Err(e) => Err(e),
        }
    }
}

impl<C: ProjectiveCurve> CanonicalDeserialize for GroupProjective<C> {
    fn deserialize<R: Read>(reader: R) -> Result<Self, SerializationError> {
        match C::deserialize(reader) {
            Ok(v) => Ok(GroupProjective(v)),
            Err(e) => Err(e),
        }
    }

    fn deserialize_uncompressed<R: Read>(
        reader: R,
    ) -> Result<Self, ark_serialize::SerializationError> {
        match C::deserialize_uncompressed(reader) {
            Ok(v) => Ok(GroupProjective(v)),
            Err(e) => Err(e),
        }
    }

    fn deserialize_unchecked<R: Read>(mut reader: R) -> Result<Self, SerializationError> {
        match C::deserialize_unchecked(reader) {
            Ok(v) => Ok(GroupProjective(v)),
            Err(e) => Err(e),
        }
    }
}
