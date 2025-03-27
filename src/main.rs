mod ecadd;
mod ecmul;
mod ecpairing;
// mod ecrecover;
mod keccak256;
// mod kzg_point_evaluation;
mod modexp;
mod secp256r1;
mod sha256;

use openvm_algebra_complex_macros::complex_init;
use openvm_algebra_guest::moduli_macros::moduli_init;
use openvm_ecc_guest::sw_macros::sw_init;

#[allow(unused_imports)]
use openvm_ecc_guest::k256::Secp256k1Point;
#[allow(unused_imports)]
use openvm_ecc_guest::p256::P256Point;
#[allow(unused_imports)]
use openvm_pairing_guest::bls12_381::Bls12_381G1Affine;
#[allow(unused_imports)]
use openvm_pairing_guest::bn254::Bn254G1Affine;

// initialize moduli
moduli_init! {
    // bn254 (alt bn128)
    "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47", // coordinate field
    "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001", // scalar field
    // secp256k1
    "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", // coordinate field
    "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", // scalar field
    // secp256r1 (p256)
    "0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff", // coordinate field
    "0xffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551", // scalar field
    // bls12_381
    "0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab", // coordinate field
    "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001", // scalar field
}
// initialize complex extensions of moduli
complex_init! {
    Bn254Fp2 { mod_idx = 0 },
    Bls12_381Fp2 { mod_idx = 6 },
}
// initialize elliptic curves
sw_init! {
    Bn254G1Affine,
    Secp256k1Point,
    P256Point,
    Bls12_381G1Affine,
}

openvm::entry!(main);

fn main() {
    {
        setup_all_moduli();
        setup_all_complex_extensions();
        setup_all_curves();
    }

    // hash function tests
    keccak256::run_keccak_tests();
    sha256::run_sha256_tests();

    // modexp tests
    modexp::run_modexp_tests();

    // bn254 (alt bn128) tests
    ecadd::run_ecadd_tests();
    ecmul::run_ecmul_tests();
    ecpairing::run_ecpairing_tests();

    // secp256k1
    // ecrecover::run_ecrecover_tests();

    // secp256r1 (p256) verify
    // TODO: create an intrinsic for this?
    secp256r1::run_p256_tests();

    // kzg point evaluation
    // kzg_point_evaluation::run_kzg_point_evaluation_tests();
}
