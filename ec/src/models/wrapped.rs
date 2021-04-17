use crate::short_weierstrass_jacobian::{
    GroupAffine as GroupAffineSW, GroupProjective as GroupProjectiveSW,
};
use crate::twisted_edwards_extended::{
    GroupAffine as GroupAffineED, GroupProjective as GroupProjectiveED,
};

use crate::{
    models::{SWModelParameters, TEModelParameters},
    AffineCurve, ProjectiveCurve,
};

use crate::group::Group;
use ark_ff::fields::{Field, PrimeField, SquareRootField};

use ark_std::vec::Vec;

// Helper Trait for wrapping types
pub trait Wrapped: Sized {
    type WrapTarget;
    fn wrapped(&self) -> &Self::WrapTarget;
    fn mut_wrapped(&mut self) -> &mut Self::WrapTarget;
}

// Wrapped AffineCurve Trait
// has default implementations to delegate the methods to
// wrapped instance
pub trait WAffineCurve: Wrapped
where
    Self::WrapTarget: AffineCurve,
{
    fn prime_subgroup_generator() -> Self::WrapTarget {
        Self::WrapTarget::prime_subgroup_generator()
    }

    fn into_projective(&self) -> <Self::WrapTarget as AffineCurve>::Projective {
        self.wrapped().into_projective()
    }

    fn from_random_bytes(bytes: &[u8]) -> Option<Self::WrapTarget> {
        Self::WrapTarget::from_random_bytes(bytes)
    }

    fn mul<S: Into<<<Self::WrapTarget as AffineCurve>::ScalarField as PrimeField>::BigInt>>(
        &self,
        other: S,
    ) -> <Self::WrapTarget as AffineCurve>::Projective {
        self.wrapped().mul(other)
    }

    fn mul_by_cofactor_to_projective(&self) -> <Self::WrapTarget as AffineCurve>::Projective {
        self.wrapped().mul_by_cofactor_to_projective()
    }

    fn mul_by_cofactor(&self) -> Self::WrapTarget {
        self.wrapped().mul_by_cofactor()
    }

    fn mul_by_cofactor_inv(&self) -> Self::WrapTarget {
        self.wrapped().mul_by_cofactor_inv()
    }
}

// Wrapped ProjectiveCurve Trait
// has default implementations to delegate the methods to
// wrapped instance
pub trait WProjectiveCurve: Wrapped
where
    Self::WrapTarget: ProjectiveCurve,
{
    fn prime_subgroup_generator() -> Self::WrapTarget {
        Self::WrapTarget::prime_subgroup_generator()
    }

    fn batch_normalization(v: &mut [Self::WrapTarget]) {
        Self::WrapTarget::batch_normalization(v)
    }

    fn batch_normalization_into_affine(
        v: &[Self::WrapTarget],
    ) -> Vec<<Self::WrapTarget as ProjectiveCurve>::Affine> {
        Self::WrapTarget::batch_normalization_into_affine(v)
    }

    fn is_normalized(&self) -> bool {
        self.wrapped().is_normalized()
    }

    fn double(&self) -> Self::WrapTarget {
        <Self::WrapTarget as ProjectiveCurve>::double(self.wrapped())
    }

    fn double_in_place(&mut self) -> &mut Self::WrapTarget {
        <Self::WrapTarget as ProjectiveCurve>::double_in_place(self.mut_wrapped())
    }

    fn into_affine(&self) -> <Self::WrapTarget as ProjectiveCurve>::Affine {
        self.wrapped().into_affine()
    }

    fn add_mixed(
        mut self,
        other: &<Self::WrapTarget as ProjectiveCurve>::Affine,
    ) -> Self::WrapTarget {
        self.mut_wrapped().add_mixed(other)
    }

    fn add_assign_mixed(&mut self, other: &<Self::WrapTarget as ProjectiveCurve>::Affine) {
        self.mut_wrapped().add_assign_mixed(other)
    }

    fn mul<S: AsRef<[u64]>>(mut self, other: S) -> Self::WrapTarget {
        self.mut_wrapped().mul(other)
    }
}

// Wrapped GroupAffineSW
pub struct WGroupAffineSW<P: SWModelParameters>(pub GroupAffineSW<P>);

impl<P: SWModelParameters> Wrapped for WGroupAffineSW<P> {
    type WrapTarget = GroupAffineSW<P>;
    fn wrapped(&self) -> &Self::WrapTarget {
        &(self.0)
    }
    fn mut_wrapped(&mut self) -> &mut Self::WrapTarget {
        &mut (self.0)
    }
}

impl<P: SWModelParameters> WAffineCurve for WGroupAffineSW<P> {}

// Wrapped GroupProjectiveSW
pub struct WGroupProjectiveSW<P: SWModelParameters>(pub GroupProjectiveSW<P>);

impl<P: SWModelParameters> Wrapped for WGroupProjectiveSW<P> {
    type WrapTarget = GroupProjectiveSW<P>;

    fn wrapped(&self) -> &Self::WrapTarget {
        &(self.0)
    }

    fn mut_wrapped(&mut self) -> &mut Self::WrapTarget {
        &mut (self.0)
    }
}

impl<P: SWModelParameters> WProjectiveCurve for WGroupProjectiveSW<P> {}
