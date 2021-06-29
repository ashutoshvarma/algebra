use crate::{
    boundary::{CallId, CrossAffine, CrossBoundary, CrossProjective, CurveParameters},
    curves::BoundaryCurves,
    serialize::{NonCanonicalDeserialize, NonCanonicalSerialize},
};
use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{
    bytes::{FromBytes, ToBytes},
    fields::PrimeField,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Cursor, Read, Result as IoResult, Write},
    ops::{Add, AddAssign, MulAssign, Neg, Sub, SubAssign},
    rand::{
        distributions::{Distribution, Standard},
        Rng,
    },
    vec::Vec,
};
use derive_more::Display;
use num_traits::Zero;
use zeroize::Zeroize;

mod macros;

pub mod short_weierstrass_jacobian;
pub mod twisted_edwards_extended;

// Helper Trait for wrapping types
pub trait Wrapped {
    type WrapTarget;
    fn wrapped(&self) -> &Self::WrapTarget;
    fn mut_wrapped(&mut self) -> &mut Self::WrapTarget;
    fn set_wrapped(&mut self, warpped: Self::WrapTarget);
    fn new_wrap(inner: Self::WrapTarget) -> Self;
}

impl<C: CrossAffine> Wrapped for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
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

impl<C: CrossProjective> Wrapped for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
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
#[repr(transparent)]
pub struct GroupAffine<C: CrossAffine>(pub C)
where
    C::Projective: CrossProjective;

impl<C: CrossAffine> Display for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        <C as Display>::fmt(self.wrapped(), f)
    }
}

impl<A: CrossAffine, P: CrossProjective> PartialEq<GroupProjective<P>> for GroupAffine<A>
where
    A: PartialEq<P>,
    A::Projective: CrossProjective,
    P::Affine: CrossAffine,
{
    fn eq(&self, other: &GroupProjective<P>) -> bool {
        // self.into_projective() == *other
        A::eq(self.wrapped(), other.wrapped())
    }
}

impl<A: CrossAffine, P: CrossProjective> PartialEq<GroupAffine<A>> for GroupProjective<P>
where
    P: PartialEq<A>,
    A::Projective: CrossProjective,
    P::Affine: CrossAffine,
{
    fn eq(&self, other: &GroupAffine<A>) -> bool {
        P::eq(self.wrapped(), other.wrapped())
    }
}

impl<C: CrossAffine> GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    #[allow(dead_code)]
    fn new(inner: C) -> Self {
        GroupAffine(inner)
    }
}

impl<C: CrossAffine> Zeroize for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    fn zeroize(&mut self) {
        C::zeroize(self.mut_wrapped())
    }
}

impl<C: CrossAffine> Zero for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    #[inline]
    fn zero() -> Self {
        GroupAffine(C::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        C::is_zero(self.wrapped())
    }
}

impl<C: CrossAffine> Add<Self> for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        GroupAffine(C::add(*self.wrapped(), *other.wrapped()))
    }
}

impl<C: CrossAffine> CurveParameters for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    type Parameters = C::Parameters;
}

impl<C: CrossAffine> AffineCurve for GroupAffine<C>
where
    C::Projective: CrossProjective,
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

impl<C: CrossAffine> Neg for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        GroupAffine(C::neg(*self.wrapped()))
    }
}

impl<C: CrossAffine> ToBytes for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    #[inline]
    fn write<W: Write>(&self, writer: W) -> IoResult<()> {
        C::write(self.wrapped(), writer)
    }
}

impl<C: CrossAffine> FromBytes for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    #[inline]
    fn read<R: Read>(reader: R) -> IoResult<Self> {
        match C::read(reader) {
            Ok(v) => Ok(GroupAffine(v)),
            Err(e) => Err(e),
        }
    }
}

impl<C: CrossAffine> Default for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    #[inline]
    fn default() -> Self {
        GroupAffine(C::default())
    }
}

impl<C: CrossAffine> core::iter::Sum<Self> for GroupAffine<C>
where
    C::Projective: CrossProjective,
    Standard: Distribution<C::Projective>,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(GroupProjective::<C::Projective>::zero(), |sum, x| {
            sum.add_mixed(&x)
        })
        .into()
    }
}

impl<'a, C: CrossAffine> core::iter::Sum<&'a Self> for GroupAffine<C>
where
    C::Projective: CrossProjective,
    Standard: Distribution<C::Projective>,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(GroupProjective::<C::Projective>::zero(), |sum, x| {
            sum.add_mixed(&x)
        })
        .into()
    }
}

// GroupProjective STARTs
#[derive(Eq, PartialEq, Display, Copy, Clone, Debug, Hash)]
#[repr(transparent)]
pub struct GroupProjective<C: CrossProjective>(pub C)
where
    C::Affine: CrossAffine;

impl<C: CrossProjective> Distribution<GroupProjective<C>> for Standard
where
    Standard: Distribution<C>,
    C::Affine: CrossAffine,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GroupProjective<C> {
        GroupProjective(Standard::sample(self, rng))
    }
}

impl<C: CrossProjective + ToBytes> ToBytes for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    #[inline]
    fn write<W: Write>(&self, writer: W) -> IoResult<()> {
        // C::write(&self.0, writer)
        self.wrapped().write(writer)
    }
}

impl<C: CrossProjective + FromBytes> FromBytes for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    #[inline]
    fn read<R: Read>(reader: R) -> IoResult<Self> {
        match C::read(reader) {
            Ok(v) => Ok(GroupProjective(v)),
            Err(e) => Err(e),
        }
    }
}

impl<C: CrossProjective + Default> Default for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    #[inline]
    fn default() -> Self {
        GroupProjective(C::default())
    }
}

impl<C: CrossProjective> CurveParameters for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    type Parameters = C::Parameters;
}

impl<C: CrossProjective> GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    pub fn new(inner: C) -> Self {
        GroupProjective(inner)
    }
}

impl<C: CrossProjective> Zeroize for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    fn zeroize(&mut self) {
        // self.mut_wrapped().zeroize()
        C::zeroize(&mut self.mut_wrapped())
    }
}

impl<C: CrossProjective> Zero for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
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

impl<C: CrossProjective> ProjectiveCurve for GroupProjective<C>
where
    Standard: Distribution<C>,
    C::Affine: CrossAffine,
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
        match Self::get_native_boundary() {
            Some(nb) => {
                // get the curve type
                let cp = BoundaryCurves::try_from_curve::<C>().unwrap();
                // alloc empty buff
                let mut buff = Cursor::new(vec![0; v.len() * v[0].noncanonical_serialized_size()]);
                // serialise all curves (since serialization for wrapped and non-wrapped are
                // same, we can just directly serializa them without unwrapping)
                for i in v.iter() {
                    i.noncanonical_serialize_uncompressed_unchecked(&mut buff)
                        .unwrap();
                }

                // call boundary
                // result is the serialized inner curves which are batch normalized
                let result = &nb
                    .call(
                        CallId::ProjBN,
                        Some(vec![&buff.into_inner()]),
                        vec![cp as u8],
                    )
                    .unwrap()
                    .unwrap()[0];

                let mut raw = Cursor::new(result);
                for i in v.iter_mut() {
                    i.set_wrapped(
                        C::noncanonical_deserialize_uncompressed_unchecked(&mut raw).unwrap(),
                    );
                }
            }
            None => {
                if C::get_native_fallback() {
                    let mut inner_curves = v.iter().map(|p| *p.wrapped()).collect::<Vec<_>>();
                    C::batch_normalization(&mut inner_curves);
                    v.iter_mut()
                        .zip(inner_curves)
                        .for_each(|(wg, g)| wg.set_wrapped(g));
                } else {
                    panic!("No boundary available!")
                }
            }
        }
    }

    fn double_in_place(&mut self) -> &mut Self {
        // self.mut_wrapped().double_in_place()
        C::double_in_place(self.mut_wrapped());
        self
    }

    fn add_assign_mixed(&mut self, other: &Self::Affine) {
        C::add_assign_mixed(self.mut_wrapped(), other.wrapped())
    }
}

impl<C: CrossProjective> Neg for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        GroupProjective(self.wrapped().neg())
    }
}

trait A: ProjectiveCurve + CrossBoundary {}

// common to both sw and ed
impl_additive_ops_from_wrapped_cross_projective!(GroupProjective);

impl<'a, C: CrossProjective> Add<&'a Self> for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    type Output = Self;
    fn add(mut self, other: &'a Self) -> Self {
        // GroupProjective(self.wrapped().add(other))
        GroupProjective(C::add(*self.mut_wrapped(), other.wrapped()))
    }
}

impl<'a, C: CrossProjective> AddAssign<&'a Self> for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    fn add_assign(&mut self, other: &'a Self) {
        // self.mut_wrapped().add_assign(other.wrapped())
        C::add_assign(self.mut_wrapped(), other.wrapped())
    }
}

impl<'a, C: CrossProjective> Sub<&'a Self> for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    type Output = Self;
    fn sub(mut self, other: &'a Self) -> Self {
        // GroupProjective(self.wrapped().sub(other))
        GroupProjective(C::sub(*self.mut_wrapped(), other.wrapped()))
    }
}

impl<'a, C: CrossProjective> SubAssign<&'a Self> for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    fn sub_assign(&mut self, other: &'a Self) {
        // self.mut_wrapped().sub_assign(other.wrapped())
        C::sub_assign(self.mut_wrapped(), other.wrapped())
    }
}

impl<C: CrossProjective> MulAssign<C::ScalarField> for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    fn mul_assign(&mut self, other: C::ScalarField) {
        self.mut_wrapped().mul_assign(other)
    }
}

// From and Serialize Traits

impl<A: CrossAffine, P: CrossProjective> From<GroupAffine<A>> for GroupProjective<P>
where
    P: From<A>,
    P::Affine: CrossAffine,
    A::Projective: CrossProjective,
{
    fn from(p: GroupAffine<A>) -> GroupProjective<P> {
        GroupProjective(P::from(*p.wrapped()))
    }
}

impl<A: CrossAffine, P: CrossProjective> From<GroupProjective<P>> for GroupAffine<A>
where
    A: From<P>,
    P::Affine: CrossAffine,
    A::Projective: CrossProjective,
{
    fn from(p: GroupProjective<P>) -> GroupAffine<A> {
        GroupAffine(A::from(*p.wrapped()))
    }
}

impl<C: CrossAffine> CanonicalSerialize for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
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

impl<C: CrossProjective> CanonicalSerialize for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
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

impl<C: CrossAffine> CanonicalDeserialize for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
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

impl<C: CrossProjective> CanonicalDeserialize for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
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

impl<C: CrossProjective> NonCanonicalSerialize for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
        &self,
        writer: W,
    ) -> Result<(), SerializationError> {
        self.wrapped()
            .noncanonical_serialize_uncompressed_unchecked(writer)
    }

    fn noncanonical_serialized_size(&self) -> usize {
        self.wrapped().noncanonical_serialized_size()
    }
}

impl<C: CrossAffine> NonCanonicalSerialize for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    fn noncanonical_serialize_uncompressed_unchecked<W: Write>(
        &self,
        writer: W,
    ) -> Result<(), SerializationError> {
        self.wrapped()
            .noncanonical_serialize_uncompressed_unchecked(writer)
    }

    fn noncanonical_serialized_size(&self) -> usize {
        self.wrapped().noncanonical_serialized_size()
    }
}

impl<C: CrossProjective> NonCanonicalDeserialize for GroupProjective<C>
where
    C::Affine: CrossAffine,
{
    fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
        reader: R,
    ) -> Result<Self, SerializationError>
    where
        Self: Sized,
    {
        match C::noncanonical_deserialize_uncompressed_unchecked(reader) {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(e),
        }
    }
}
impl<C: CrossAffine> NonCanonicalDeserialize for GroupAffine<C>
where
    C::Projective: CrossProjective,
{
    fn noncanonical_deserialize_uncompressed_unchecked<R: Read>(
        reader: R,
    ) -> Result<Self, SerializationError>
    where
        Self: Sized,
    {
        match C::noncanonical_deserialize_uncompressed_unchecked(reader) {
            Ok(v) => Ok(Self(v)),
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

impl<E: PairingEngine> From<G1Affine<E>> for G1Prepared<E>
where
    E::G1Projective: CrossProjective,
    E::G1Affine: CrossAffine,
    E::G2Projective: CrossProjective,
    E::G2Affine: CrossAffine,
{
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

impl<E: PairingEngine> From<G2Affine<E>> for G2Prepared<E>
where
    E::G1Projective: CrossProjective,
    E::G1Affine: CrossAffine,
    E::G2Projective: CrossProjective,
    E::G2Affine: CrossAffine,
{
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
    E::G1Projective: CrossProjective,
    E::G1Affine: CrossAffine,
    E::G2Projective: CrossProjective,
    E::G2Affine: CrossAffine,
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
