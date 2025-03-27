mod ecadd;
mod ecmul;
mod ecpairing;
// mod ecrecover;
mod keccak256;
// mod modexp;
// mod secp256r1;
mod sha256;

#[allow(unused_imports)]
use openvm_pairing_guest::bn254::Bn254G1Affine;

openvm_algebra_guest::moduli_macros::moduli_init! {
    "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47", // Bn254Fp Coordinate field
    "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001", // Bn254 Scalar
    // "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F", // secp256k1 Coordinate field
    // "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141", // secp256k1 Scalar field
    // "0xffffffff 00000001 00000000 00000000 00000000 ffffffff ffffffff ffffffff", // secp256r1_coord_prime
    // "0xffffffff 00000000 ffffffff ffffffff bce6faad a7179e84 f3b9cac2 fc632551" // secp256r1_scalar_prime
}

// use openvm_ecc_guest::weierstrass::WeierstrassPoint;

openvm_ecc_guest::sw_macros::sw_init! {
    // Secp256k1Point,
    Bn254G1Affine,
    // P256Point,
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

    // bn254 (alt bn128) tests
    ecadd::run_ecadd_tests();
    ecmul::run_ecmul_tests();
    ecpairing::run_ecpairing_tests();

    // secp256k1
    // ecrecover::run_ecrecover_tests();
}
