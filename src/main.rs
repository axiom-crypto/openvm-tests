mod ecadd;
mod ecmul;
mod ecpairing;
mod ecrecover;
mod keccak256;
mod modexp;
mod secp256r1;
mod sha256;

#[allow(unused_imports)]
use openvm_ecc_guest::k256::Secp256k1Point;
#[allow(unused_imports)]
use openvm_ecc_guest::p256::P256Point;
#[allow(unused_imports)]
use openvm_pairing_guest::bn254::Bn254G1Affine;

openvm_algebra_guest::moduli_macros::moduli_init! {
    "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47", // bn254 (alt bn128) coordinate field
    "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001", // bn254 (alt bn128) scalar field
    "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", // secp256k1 coordinate field
    "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", // secp256k1 scalar field
    "0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff", // p256 (secp256r1) coordinate field
    "0xffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551", // p256 (secp256r1) scalar field
}

// use openvm_ecc_guest::weierstrass::WeierstrassPoint;

openvm_ecc_guest::sw_macros::sw_init! {
    Bn254G1Affine,
    Secp256k1Point,
    P256Point,
}

openvm_algebra_complex_macros::complex_init! {
    Bn254Fp2 { mod_idx = 0 },
}

openvm::entry!(main);

fn main() {
    {
        setup_all_moduli();
        setup_all_curves();
        setup_all_complex_extensions();
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
    ecrecover::run_ecrecover_tests();

    // secp256r1
    // TODO: no intrinsic
    secp256r1::run_p256_tests();
}
