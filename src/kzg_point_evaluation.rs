use hex_literal::hex;
use revm_precompile::kzg_point_evaluation::run;
use revm_primitives::{Bytes, Env};
use revm_primitives::{PrecompileError, PrecompileErrors};

/// Type alias for test case input
type TestCaseInput = &'static [u8];
/// Type alias for test case expected output
type TestCaseOutput = Result<[u8; 64], PrecompileErrors>;
/// Type alias for a KZG point evaluation test case
type KzgPointEvaluationTestCase = (TestCaseInput, TestCaseOutput);

/// Vector of test cases for KZG point evaluation precompile.
/// Each test case consists of (input_bytes, expected_output_bytes or expected_error).
const KZG_POINT_EVALUATION_TEST_CASES: &[KzgPointEvaluationTestCase] = &[
    // https://github.com/ethereum/go-ethereum/blob/master/core/vm/testdata/precompiles/pointEvaluation.json
    (
        &hex!("01e798154708fe7789429634053cbf9f99b619f9f084048927333fce637f549b564c0a11a0f704f4fc3e8acfe0f8245f0ad1347b378fbf96e206da11a5d3630624d25032e67a7e6a4910df5834b8fe70e6bcfeeac0352434196bdf4b2485d5a18f59a8d2a1a625a17f3fea0fe5eb8c896db3764f3185481bc22f91b4aaffcca25f26936857bc3a7c2539ea8ec3a952b7873033e038326e87ed3e1276fd140253fa08e9fc25fb2d9a98527fc22a2c9612fbeafdad446cbc7bcdbdcd780af2c16a"),
        Ok(hex!("000000000000000000000000000000000000000000000000000000000000100073eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001")),
    ),
    // https://etherscan.io/tx/0x77e17d28223f3250ad3c6943da309077409427eb1d44c4613727d0155fbcd970
    (
        &hex!("01daea18a8eae6352683ed6614db4369569cd049c0228f64cd9d898de870f5692e5e86404fe0e8bd315c5d24fc35657b306db2c697e8630602c469a2a402152726e7af293b70e0f025157954fd0db9ca98209741de8fc5c1604d15cf7299ad6199ed84aaaae68a5ee748d90005b29ee8b2215bea04b5310b0b6ccffc5af94a3d2772dbe4ae3bf94d89957699abac7e99aa77f12f3fbbe1d2f4793f0040b9b490d2d2990850851693cbd256c6a67ea8ee1c5e31c271f2b7a25be096efbadbf621"),
        Err(PrecompileErrors::Error(PrecompileError::BlobVerifyKzgProofFailed)),
    ),
];

/// Run all KZG point evaluation precompile test cases
pub fn run_kzg_point_evaluation_tests() {
    for (input, expected) in KZG_POINT_EVALUATION_TEST_CASES {
        let input = Bytes::from_static(input);
        let result = run(&input, u64::MAX, &Env::default());

        match (result, expected) {
            (Ok(output), Ok(expected_bytes)) => {
                assert_eq!(output.bytes.as_ref(), expected_bytes.as_slice());
            }
            (Err(error), Err(expected_error)) => {
                assert_eq!(error, *expected_error);
            }
            (Ok(output), Err(expected_error)) => {
                panic!("Expected error {expected_error:?}, but got success: {output:?}");
            }
            (Err(error), Ok(expected_bytes)) => {
                panic!("Expected success with bytes {expected_bytes:?}, but got error: {error:?}");
            }
        }
    }
}
