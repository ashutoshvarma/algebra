use ark_ec::{
    AffineCurve, ModelParameters, PairingEngine, ProjectiveCurve, SWModelParameters,
    TEModelParameters,
};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};

use ark_ff::{
    bytes::{FromBytes, ToBytes},
    fields::PrimeField,
};

use ark_std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Read, Result as IoResult, Write},
    ops::{Add, AddAssign, MulAssign, Neg, Sub, SubAssign},
    vec::Vec,
};

use num_traits::Zero;
use zeroize::Zeroize;

use ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use derive_more::Display;

// pub mod serialize;
pub mod serialize;
pub mod short_weierstrass_jacobian;
pub mod twisted_edwards_extended;

pub trait WrappedCurve: Wrapped {
    type InnerCurveParameter: ModelParameters;
}

// Helper Trait for wrapping types
pub trait Wrapped {
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
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
// #[derive(Derivative)]
// #[derivative(
//     Copy(bound = "C: AffineCurve"),
//     Clone(bound = "C: AffineCurve"),
//     Debug(bound = "C: AffineCurve"),
//     Hash(bound = "C: AffineCurve"),
//     // PartialEq(bound = "C: AffineCurve"),
//     // Eq(bound = "C: AffineCurve")
// )]
#[repr(transparent)]
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
    #[allow(dead_code)]
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
    fn write<W: Write>(&self, writer: W) -> IoResult<()> {
        C::write(self.wrapped(), writer)
    }
}

impl<C: AffineCurve> FromBytes for GroupAffine<C> {
    #[inline]
    fn read<R: Read>(reader: R) -> IoResult<Self> {
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
#[derive(Eq, PartialEq, Display, Copy, Clone, Debug, Hash)]
// #[derive(Derivative)]
// #[derivative(
//     Copy(bound = "C: ProjectiveCurve"),
//     Clone(bound = "C: ProjectiveCurve"),
//     Debug(bound = "C: ProjectiveCurve"),
//     Hash(bound = "C: ProjectiveCurve")
// )]
#[repr(transparent)]
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
    fn write<W: Write>(&self, writer: W) -> IoResult<()> {
        // C::write(&self.0, writer)
        self.wrapped().write(writer)
    }
}

impl<C: ProjectiveCurve + FromBytes> FromBytes for GroupProjective<C> {
    #[inline]
    fn read<R: Read>(reader: R) -> IoResult<Self> {
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
        // This is slow, should be changed to a tested "unsafe" impl after
        // bench and profiling
        let mut p_s = v.iter().map(|p| *p.wrapped()).collect::<Vec<_>>();
        C::batch_normalization(&mut p_s);

        v.iter_mut().zip(p_s).for_each(|(wg, g)| wg.set_wrapped(g));
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
    fn serialize_uncompressed<W: Write>(&self, writer: W) -> Result<(), SerializationError> {
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
    fn serialize_uncompressed<W: Write>(&self, writer: W) -> Result<(), SerializationError> {
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

    fn deserialize_unchecked<R: Read>(reader: R) -> Result<Self, SerializationError> {
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

    fn deserialize_unchecked<R: Read>(reader: R) -> Result<Self, SerializationError> {
        match C::deserialize_unchecked(reader) {
            Ok(v) => Ok(GroupProjective(v)),
            Err(e) => Err(e),
        }
    }
}

// #########################################################

// G1 Types
pub type G1Affine<E> = GroupAffine<<E as PairingEngine>::G1Affine>;
pub type G1Projective<E> = GroupProjective<<E as PairingEngine>::G1Projective>;

#[derive(Clone, Debug)]
pub struct G1Prepared<E: PairingEngine>(pub E::G1Prepared);

impl<E: PairingEngine> From<G1Affine<E>> for G1Prepared<E> {
    fn from(other: G1Affine<E>) -> Self {
        G1Prepared(E::G1Prepared::from(other.0))
    }
}

impl<E: PairingEngine> Default for G1Prepared<E> {
    fn default() -> Self {
        G1Prepared(E::G1Prepared::default())
    }
}

impl<E: PairingEngine> ToBytes for G1Prepared<E> {
    fn write<W: Write>(&self, writer: W) -> IoResult<()> {
        self.0.write(writer)
    }
}

// G2 Types
pub type G2Affine<E> = GroupAffine<<E as PairingEngine>::G2Affine>;
pub type G2Projective<E> = GroupProjective<<E as PairingEngine>::G2Projective>;

#[derive(Clone, Debug)]
pub struct G2Prepared<E: PairingEngine>(pub E::G2Prepared);

impl<E: PairingEngine> From<G2Affine<E>> for G2Prepared<E> {
    fn from(other: G2Affine<E>) -> Self {
        G2Prepared(E::G2Prepared::from(other.0))
    }
}

impl<E: PairingEngine> Default for G2Prepared<E> {
    fn default() -> Self {
        G2Prepared(E::G2Prepared::default())
    }
}

impl<E: PairingEngine> ToBytes for G2Prepared<E> {
    fn write<W: Write>(&self, writer: W) -> IoResult<()> {
        self.0.write(writer)
    }
}

// Pairing Curve

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct EngineWrapper<E: PairingEngine>(pub E);

impl<E: PairingEngine> PairingEngine for EngineWrapper<E>
where
    Standard: Distribution<E::G1Projective>,
    Standard: Distribution<E::G2Projective>,
{
    type Fr = E::Fr;
    type G1Projective = G1Projective<E>;
    type G1Affine = G1Affine<E>;
    type G1Prepared = G1Prepared<E>;
    type G2Projective = G2Projective<E>;
    type G2Affine = G2Affine<E>;
    type G2Prepared = G2Prepared<E>;
    type Fq = E::Fq;
    type Fqe = E::Fqe;
    type Fqk = E::Fqk;

    fn miller_loop<'a, I>(i: I) -> Self::Fqk
    where
        I: IntoIterator<Item = &'a (Self::G1Prepared, Self::G2Prepared)>,
    {
        // NOTE: current impl is extremely slow as it requires double iter
        let pairs = i
            .into_iter()
            .map(|i| (i.0 .0.clone(), i.1 .0.clone()))
            .collect::<Vec<_>>();
        return E::miller_loop(pairs.iter().collect::<Vec<_>>());
    }

    fn final_exponentiation(f: &Self::Fqk) -> Option<Self::Fqk> {
        return E::final_exponentiation(f);
    }
}
