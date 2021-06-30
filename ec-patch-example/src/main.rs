use ark_mnt4_298::G1Affine;
use num_traits::identities::Zero;

fn main() {
    let z = G1Affine::zero();
    println!("Zero - {}", &z);
}
