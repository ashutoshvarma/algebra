use crate::boundary::{CurveParameters, CurveType};
use ark_ec::models::mnt4::MNT4Parameters;
use ark_ec::{ModelParameters, SWModelParameters, TEModelParameters};
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
    // ark_bls12_277,
    // Bls12_377G1,
    // Bls12_377G2,
    // ark_mnt4_298
    MNT4_298G1,
    MNT4_298G2,
}

impl BoundaryCurves {
    pub fn try_from_curve<T: CurveParameters>() -> Result<Self, ()> {
        match T::TYPE {
            // SW Curves
            CurveType::SW => {
                // pallas
                if compare_sw_parameters::<T::SWParameters, pallas::PallasParameters>() {
                    Ok(BoundaryCurves::Pallas)
                // bls12_377
                // } else if compare_parameters::<T::Parameters, bls12_377::g1::Parameters>() {
                //     Ok(BoundaryCurves::Bls12_377G1)
                // } else if compare_parameters::<T::Parameters, bls12_377::g2::Parameters>() {
                //     Ok(BoundaryCurves::Bls12_377G2)
                // mnt4_298
                } else if compare_sw_parameters::<T::SWParameters, mnt4_298::g1::Parameters>() {
                    Ok(BoundaryCurves::MNT4_298G1)
                } else if compare_sw_parameters::<T::SWParameters, mnt4_298::g2::Parameters>() {
                    Ok(BoundaryCurves::MNT4_298G2)
                } else {
                    Err(())
                }
            }
            CurveType::ED => {
                // ed_on_bls12_377
                if compare_ed_parameters::<T::TEParameters, ed_on_bls12_377::EdwardsParameters>() {
                    Ok(BoundaryCurves::EdBls12_377)
                } else {
                    Err(())
                }
            }
        }
    }
}

pub fn compare_sw_parameters<T: SWModelParameters, U: SWModelParameters>() -> bool {
    let model = T::BaseField::characteristic() == U::BaseField::characteristic()
        && T::ScalarField::characteristic() == U::ScalarField::characteristic();

    model && T::COFACTOR == U::COFACTOR
}

pub fn compare_ed_parameters<T: TEModelParameters, U: TEModelParameters>() -> bool {
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

    fn assert_from_wrapped<C: CurveParameters>(p: BoundaryCurves) {
        let param = BoundaryCurves::try_from_curve::<C>().unwrap();
        assert_eq!(param, p);
    }

    #[test]
    fn test_from_wrapped() {
        // pallas
        assert_from_wrapped::<ark_pallas::Affine>(BoundaryCurves::Pallas);
        assert_from_wrapped::<ark_pallas::Projective>(BoundaryCurves::Pallas);
        // ed_on_bls12_377
        assert_from_wrapped::<ark_ed_on_bls12_377::EdwardsAffine>(BoundaryCurves::EdBls12_377);
        assert_from_wrapped::<ark_ed_on_bls12_377::EdwardsProjective>(BoundaryCurves::EdBls12_377);
        // mnt4_298
        assert_from_wrapped::<ark_mnt4_298::G1Affine>(BoundaryCurves::MNT4_298G1);
        assert_from_wrapped::<ark_mnt4_298::G1Projective>(BoundaryCurves::MNT4_298G1);
        assert_from_wrapped::<ark_mnt4_298::G2Affine>(BoundaryCurves::MNT4_298G2);
        assert_from_wrapped::<ark_mnt4_298::G2Projective>(BoundaryCurves::MNT4_298G2);
    }
}
