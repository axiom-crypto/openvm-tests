use hex_literal::hex;
use revm_precompile::kzg_point_evaluation::run;
use revm_primitives::{Bytes, Env};

/// Vector of test cases for KZG point evaluation precompile.
/// Each test case consists of (input_bytes, expected_output_bytes).
const KZG_POINT_EVALUATION_TEST_CASES: &[(&[u8], [u8; 64])] = &[
    // https://github.com/ethereum/go-ethereum/blob/master/core/vm/testdata/precompiles/pointEvaluation.json
    (
        &hex!("01e798154708fe7789429634053cbf9f99b619f9f084048927333fce637f549b564c0a11a0f704f4fc3e8acfe0f8245f0ad1347b378fbf96e206da11a5d3630624d25032e67a7e6a4910df5834b8fe70e6bcfeeac0352434196bdf4b2485d5a18f59a8d2a1a625a17f3fea0fe5eb8c896db3764f3185481bc22f91b4aaffcca25f26936857bc3a7c2539ea8ec3a952b7873033e038326e87ed3e1276fd140253fa08e9fc25fb2d9a98527fc22a2c9612fbeafdad446cbc7bcdbdcd780af2c16a"),
        hex!("000000000000000000000000000000000000000000000000000000000000100073eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001"),
    ),
];

/// Run all KZG point evaluation precompile test cases
pub fn run_kzg_point_evaluation_tests() {
    for (input, expected) in KZG_POINT_EVALUATION_TEST_CASES {
        let input = Bytes::from_static(input);
        let outcome = run(&input, u64::MAX, &Env::default()).unwrap();
        assert_eq!(outcome.bytes.as_ref(), expected.as_slice());
    }
}
