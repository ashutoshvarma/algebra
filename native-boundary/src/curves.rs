use crate::wrapped::WrappedCurve;
use ark_ec::models::mnt4::MNT4Parameters;
use ark_ed_on_bls12_377::EdwardsParameters as EdBls12_377_Parameters;
use ark_mnt4_298::Parameters as MNT4_298_Parameters;
use ark_pallas::PallasParameters;
use std::any::TypeId;

macro_rules! try_from_u8 {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u8> for $name {
            type Error = ();

            fn try_from(v: u8) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u8 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

try_from_u8! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum CurveParameters {
        // ark_pallas
        Pallas = 1,
        // ark_ed_on_bls12_377
        EdBls12_377 = 2,
        // ark_mnt4_298
        MNT4_298G1 = 3,
        MNT4_298G2 = 4,
    }
}

impl CurveParameters {
    pub fn try_from_wrapped<T: WrappedCurve>() -> Result<Self, ()> {
        let id = TypeId::of::<T::InnerCurveParameter>();
        // Pallas
        if id == TypeId::of::<PallasParameters>() {
            Ok(CurveParameters::Pallas)
        // EdBls12_377
        } else if id == TypeId::of::<EdBls12_377_Parameters>() {
            Ok(CurveParameters::EdBls12_377)
        // MNT4_298
        } else if id == TypeId::of::<<MNT4_298_Parameters as MNT4Parameters>::G1Parameters>() {
            Ok(CurveParameters::MNT4_298G1)
        } else if id == TypeId::of::<<MNT4_298_Parameters as MNT4Parameters>::G2Parameters>() {
            Ok(CurveParameters::MNT4_298G2)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::wrapped::{GroupAffine, GroupProjective};
    use ark_mnt4_298::{G1Affine as M298_G1Affine, G2Projective as M298_G2Projective};
    use ark_pallas::{Affine as PAffine, Projective as PProjective};
    use std::convert::TryInto;

    fn assert_u8_try_from(p: CurveParameters) {
        let back = (p.clone() as u8).try_into().unwrap();
        assert_eq!(p, back);
    }
    #[test]
    fn test_from_u8() {
        assert_u8_try_from(CurveParameters::Pallas);
        assert_u8_try_from(CurveParameters::EdBls12_377);
        assert_u8_try_from(CurveParameters::MNT4_298G1);
        assert_u8_try_from(CurveParameters::MNT4_298G2);
    }

    fn assert_from_wrapped<C: WrappedCurve>(p: CurveParameters) {
        let param = CurveParameters::try_from_wrapped::<C>().unwrap();
        assert_eq!(param, p);
    }

    #[test]
    fn test_from_wrapped() {
        assert_from_wrapped::<GroupAffine<PAffine>>(CurveParameters::Pallas);
        assert_from_wrapped::<GroupProjective<PProjective>>(CurveParameters::Pallas);
        // mnt4_298
        assert_from_wrapped::<GroupAffine<M298_G1Affine>>(CurveParameters::MNT4_298G1);
        assert_from_wrapped::<GroupProjective<M298_G2Projective>>(CurveParameters::MNT4_298G2);
    }
}
