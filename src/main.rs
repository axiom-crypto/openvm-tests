mod bn128;
mod ecrecover;
mod keccak256;
mod kzg_point_evaluation;
mod modexp;
mod secp256r1;
mod sha256;

#[allow(unused_imports)]
use k256::Secp256k1Point;
#[allow(unused_imports)]
use openvm_pairing::bls12_381::Bls12_381G1Affine;
#[allow(unused_imports)]
use openvm_pairing::bn254::Bn254G1Affine;
#[allow(unused_imports)]
use p256::P256Point;

openvm::init!("openvm_init.rs");

openvm::entry!(main);

pub fn my_custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    buf.fill(0);
    Ok(())
}

fn main() {
    getrandom::register_custom_getrandom!(my_custom_getrandom);

    // hash function tests
    keccak256::run_keccak_tests();
    sha256::run_sha256_tests();

    // modexp tests
    modexp::run_modexp_tests();

    // bn254 (alt bn128) tests
    bn128::run_ecadd_tests();
    bn128::run_ecmul_tests();
    bn128::run_ecpairing_tests();

    // secp256k1
    ecrecover::run_ecrecover_tests();

    // secp256r1 (p256) verify
    secp256r1::run_p256_tests();

    // kzg point evaluation
    kzg_point_evaluation::run_kzg_point_evaluation_tests();
}
