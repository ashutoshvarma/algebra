use crate::ark_ec::{AffineCurve, ProjectiveCurve};
use crate::boundary::{CurveParameters, CurveType};
use ark_ff::Field;
use ark_std::any::TypeId;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub mod bls12_377;
pub mod ed_on_bls12_377;
pub mod mnt4_298;
pub mod pallas;

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum BoundaryCurves {
    // ark_pallas
    Pallas,
    // ark_ed_on_bls12_377
    EdBls12_377,
    // // ark_bls12_277,
    // Bls12_377G1,
    // Bls12_377G2,
    // ark_mnt4_298
    MNT4_298G1,
    MNT4_298G2,
}

impl BoundaryCurves {
    pub fn try_from_affine<T: AffineCurve>() -> Result<Self, ()> {
        if compare_affine_type::<T, pallas::Affine>() {
            Ok(BoundaryCurves::Pallas)
        } else if compare_affine_type::<T, mnt4_298::g1::G1Affine>() {
            Ok(BoundaryCurves::MNT4_298G1)
        } else if compare_affine_type::<T, mnt4_298::g2::G2Affine>() {
            Ok(BoundaryCurves::MNT4_298G2)
        // } else if compare_affine_type::<T, bls12_377::G1Affine>() {
        //     Ok(BoundaryCurves::Bls12_377G1)
        // } else if compare_affine_type::<T, bls12_377::G2Affine>() {
        //     Ok(BoundaryCurves::Bls12_377G2)
        } else if compare_affine_type::<T, ed_on_bls12_377::EdwardsAffine>() {
            Ok(BoundaryCurves::EdBls12_377)
        } else {
            Err(())
        }
    }
    pub fn try_from_projective<T: ProjectiveCurve>() -> Result<Self, ()> {
        if compare_projective_type::<T, pallas::Projective>() {
            Ok(BoundaryCurves::Pallas)
        } else if compare_projective_type::<T, mnt4_298::g1::G1Projective>() {
            Ok(BoundaryCurves::MNT4_298G1)
        } else if compare_projective_type::<T, mnt4_298::g2::G2Projective>() {
            Ok(BoundaryCurves::MNT4_298G2)
        // } else if compare_projective_type::<T, bls12_377::G1Affine>() {
        //     Ok(BoundaryCurves::Bls12_377G1)
        // } else if compare_projective_type::<T, bls12_377::G2Affine>() {
        //     Ok(BoundaryCurves::Bls12_377G2)
        } else if compare_projective_type::<T, ed_on_bls12_377::EdwardsProjective>() {
            Ok(BoundaryCurves::EdBls12_377)
        } else {
            Err(())
        }
    }
}

pub fn compare_affine_type<T: AffineCurve, U: AffineCurve>() -> bool {
    let model = T::BaseField::characteristic() == U::BaseField::characteristic()
        && T::ScalarField::characteristic() == U::ScalarField::characteristic();

    model && T::COFACTOR == U::COFACTOR
}
pub fn compare_projective_type<T: ProjectiveCurve, U: ProjectiveCurve>() -> bool {
    let model = T::BaseField::characteristic() == U::BaseField::characteristic()
        && T::ScalarField::characteristic() == U::ScalarField::characteristic();

    model && T::COFACTOR == U::COFACTOR
}

#[cfg(test)]
pub mod test {
    use super::*;
    use ark_std::convert::TryInto;

    fn assert_u8_try_from(p: BoundaryCurves) {
        let back = (p.clone() as u8).try_into().unwrap();
        assert_eq!(p, back);
    }
    #[test]
    fn test_from_u8() {
        assert_u8_try_from(BoundaryCurves::Pallas);
        assert_u8_try_from(BoundaryCurves::EdBls12_377);
        assert_u8_try_from(BoundaryCurves::MNT4_298G1);
        assert_u8_try_from(BoundaryCurves::MNT4_298G2);
    }

    fn assert_from_affine<C: AffineCurve>(p: BoundaryCurves) {
        let param = BoundaryCurves::try_from_affine::<C>().unwrap();
        assert_eq!(param, p);
    }

    #[test]
    fn test_from_affine() {
        // pallas
        assert_from_affine::<ark_pallas::Affine>(BoundaryCurves::Pallas);
        // ed_on_bls12_377
        assert_from_affine::<ark_ed_on_bls12_377::EdwardsAffine>(BoundaryCurves::EdBls12_377);
        // mnt4_298
        assert_from_affine::<ark_mnt4_298::G1Affine>(BoundaryCurves::MNT4_298G1);
        assert_from_affine::<ark_mnt4_298::G2Affine>(BoundaryCurves::MNT4_298G2);
    }
}
