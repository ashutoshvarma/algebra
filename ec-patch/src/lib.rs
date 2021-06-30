pub use ark_ec_orig::{
    prepare_g1, prepare_g2, AffineCurve, CurveCycle, PairingEngine, PairingFriendlyCycle,
    ProjectiveCurve,
};

pub mod models {
    pub use ark_ec_orig::models::*;
}
pub use models::*;

pub mod group {
    pub use ark_ec_orig::group::*;
}

pub mod wnaf {
    pub use ark_ec_orig::wnaf::*;
}

// pub use ark_ec_orig::msm;
// Instead of ark_ec::msm we replace that with boundary msm
pub use ark_native_boundary::msm;
