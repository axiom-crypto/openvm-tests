use hex_literal::hex;
use revm_precompile::bn128::run_pair;
use revm_precompile::{PrecompileError, PrecompileErrors};

/// Type alias for test case input
type TestCaseInput = &'static [u8];
/// Type alias for test case expected output
type TestCaseOutput = Result<&'static [u8], PrecompileErrors>;
/// Type alias for a pairing test case
type PairingTestCase = (TestCaseInput, TestCaseOutput);

/// Vector of test cases for ecPairing precompile.
/// Each test case consists of (input_bytes, expected_output_bytes or expected_error).
const ECPAIRING_CASES: &[PairingTestCase] = &[
    // pair
    (
        &hex!("1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // ethereum/tests/Checking a pairing equation on curve alt_bn128
    (
        &hex!("1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // no input
    (
        b"",
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001"))
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#jeff1
    (
        &hex!("1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#jeff2
    (
        &hex!("2eca0c7238bf16e83e7a1e6c5d49540685ff51380f309842a98561558019fc0203d3260361bb8451de5ff5ecd17f010ff22f5c31cdf184e9020b06fa5997db841213d2149b006137fcfb23036606f848d638d576a120ca981b5b1a5f9300b3ee2276cf730cf493cd95d64677bbb75fc42db72513a4c1e387b476d056f80aa75f21ee6226d31426322afcda621464d0611d226783262e21bb3bc86b537e986237096df1f82dff337dd5972e32a8ad43e28a78a96a823ef1cd4debe12b6552ea5f06967a1237ebfeca9aaae0d6d0bab8e28c198c5a339ef8a2407e31cdac516db922160fa257a5fd5b280642ff47b65eca77e626cb685c84fa6d3b6882a283ddd1198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#jeff3
    (
        &hex!("0f25929bcb43d5a57391564615c9e70a992b10eafa4db109709649cf48c50dd216da2f5cb6be7a0aa72c440c53c9bbdfec6c36c7d515536431b3a865468acbba2e89718ad33c8bed92e210e81d1853435399a271913a6520736a4729cf0d51eb01a9e2ffa2e92599b68e44de5bcf354fa2642bd4f26b259daa6f7ce3ed57aeb314a9a87b789a58af499b314e13c3d65bede56c07ea2d418d6874857b70763713178fb49a2d6cd347dc58973ff49613a20757d0fcc22079f9abd10c3baee245901b9e027bd5cfc2cb5db82d4dc9677ac795ec500ecd47deee3b5da006d6d049b811d7511c78158de484232fc68daf8a45cf217d1c2fae693ff5871e8752d73b21198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#jeff4
    (
        &hex!("2f2ea0b3da1e8ef11914acf8b2e1b32d99df51f5f4f206fc6b947eae860eddb6068134ddb33dc888ef446b648d72338684d678d2eb2371c61a50734d78da4b7225f83c8b6ab9de74e7da488ef02645c5a16a6652c3c71a15dc37fe3a5dcb7cb122acdedd6308e3bb230d226d16a105295f523a8a02bfc5e8bd2da135ac4c245d065bbad92e7c4e31bf3757f1fe7362a63fbfee50e7dc68da116e67d600d9bf6806d302580dc0661002994e7cd3a7f224e7ddc27802777486bf80f40e4ca3cfdb186bac5188a98c45e6016873d107f5cd131f3a3e339d0375e58bd6219347b008122ae2b09e539e152ec5364e7e2204b03d11d3caa038bfc7cd499f8176aacbee1f39e4e4afc4bc74790a4a028aff2c3d2538731fb755edefd8cb48d6ea589b5e283f150794b6736f670d6a1033f9b46c6f5204f50813eb85c8dc4b59db1c5d39140d97ee4d2b36d99bc49974d18ecca3e7ad51011956051b464d9e27d46cc25e0764bb98575bd466d32db7b15f582b2d5c452b36aa394b789366e5e3ca5aabd415794ab061441e51d01e94640b7e3084a07e02c78cf3103c542bc5b298669f211b88da1679b0b64a63b7e0e7bfe52aae524f73a55be7fe70c7e9bfc94b4cf0da1213d2149b006137fcfb23036606f848d638d576a120ca981b5b1a5f9300b3ee2276cf730cf493cd95d64677bbb75fc42db72513a4c1e387b476d056f80aa75f21ee6226d31426322afcda621464d0611d226783262e21bb3bc86b537e986237096df1f82dff337dd5972e32a8ad43e28a78a96a823ef1cd4debe12b6552ea5f"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#jeff5
    (
        &hex!("20a754d2071d4d53903e3b31a7e98ad6882d58aec240ef981fdf0a9d22c5926a29c853fcea789887315916bbeb89ca37edb355b4f980c9a12a94f30deeed30211213d2149b006137fcfb23036606f848d638d576a120ca981b5b1a5f9300b3ee2276cf730cf493cd95d64677bbb75fc42db72513a4c1e387b476d056f80aa75f21ee6226d31426322afcda621464d0611d226783262e21bb3bc86b537e986237096df1f82dff337dd5972e32a8ad43e28a78a96a823ef1cd4debe12b6552ea5f1abb4a25eb9379ae96c84fff9f0540abcfc0a0d11aeda02d4f37e4baf74cb0c11073b3ff2cdbb38755f8691ea59e9606696b3ff278acfc098fa8226470d03869217cee0a9ad79a4493b5253e2e4e3a39fc2df38419f230d341f60cb064a0ac290a3d76f140db8418ba512272381446eb73958670f00cf46f1d9e64cba057b53c26f64a8ec70387a13e41430ed3ee4a7db2059cc5fc13c067194bcc0cb49a98552fd72bd9edb657346127da132e5b82ab908f5816c826acb499e22f2412d1a2d70f25929bcb43d5a57391564615c9e70a992b10eafa4db109709649cf48c50dd2198a1f162a73261f112401aa2db79c7dab1533c9935c77290a6ce3b191f2318d198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#jeff6
    (
        &hex!("1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c103188585e2364128fe25c70558f1560f4f9350baf3959e603cc91486e110936198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000000")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#empty_data
    (
        &hex!(""),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#one_point
    (
        &hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000000")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#two_point_match_2
    (
        &hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#two_point_match_3
    (
        &hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#two_point_match_4
    (
        &hex!("105456a333e6d636854f987ea7bb713dfd0ae8371a72aea313ae0c32c0bf10160cf031d41b41557f3e7e3ba0c51bebe5da8e6ecd855ec50fc87efcdeac168bcc0476be093a6d2b4bbf907172049874af11e1b6267606e00804d3ff0037ec57fd3010c68cb50161b7d1d96bb71edfec9880171954e56871abf3d93cc94d745fa114c059d74e5b6c4ec14ae5864ebe23a71781d86c29fb8fb6cce94f70d3de7a2101b33461f39d9e887dbb100f170a2345dde3c07e256d1dfa2b657ba5cd030427000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000021a2c3013d2ea92e13c800cde68ef56a294b883f6ac35d25f587c09b1b3c635f7290158a80cd3d66530f74dc94c94adb88f5cdb481acca997b6e60071f08a115f2f997f3dbd66a7afe07fe7862ce239edba9e05c5afff7f8a1259c9733b2dfbb929d1691530ca701b4a106054688728c9972c8512e9789e9567aae23e302ccd75"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // go-ethereum/core/vm/testdata/precompiles/bn256Pairing.json#ten_point_match_3
    (
        &hex!("105456a333e6d636854f987ea7bb713dfd0ae8371a72aea313ae0c32c0bf10160cf031d41b41557f3e7e3ba0c51bebe5da8e6ecd855ec50fc87efcdeac168bcc0476be093a6d2b4bbf907172049874af11e1b6267606e00804d3ff0037ec57fd3010c68cb50161b7d1d96bb71edfec9880171954e56871abf3d93cc94d745fa114c059d74e5b6c4ec14ae5864ebe23a71781d86c29fb8fb6cce94f70d3de7a2101b33461f39d9e887dbb100f170a2345dde3c07e256d1dfa2b657ba5cd030427000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000021a2c3013d2ea92e13c800cde68ef56a294b883f6ac35d25f587c09b1b3c635f7290158a80cd3d66530f74dc94c94adb88f5cdb481acca997b6e60071f08a115f2f997f3dbd66a7afe07fe7862ce239edba9e05c5afff7f8a1259c9733b2dfbb929d1691530ca701b4a106054688728c9972c8512e9789e9567aae23e302ccd75"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // sepolia.scrollscan.com transaction test case
    (
        &hex!("f736f6f8fdc7839c518a5011ad11a8384742778efbd1377dc0473b3600d301edef18eb4967eb80278618f9e0cd0f9f33ce9fc45ab15a5dae351dd6d968e478e98ede98c55a0d9d536f9869c3701e8b3b9dff6fe6dae3df018e063d49e57d4b4f30149301c3ac411e846a243cc806e9cd0eddb96cab07940ac269636f2116bf67273e6de4c42dfae5b1dd36c729577924e54d13d43152482d0df4d1539a9504b89a7ea6b627f4b6b6b922cdd5f08a8fe27296e14bc2bb47895c0c30ca861affc3"),
        Err(PrecompileErrors::Error(PrecompileError::Bn128FieldPointNotAMember)),
    ),
    // scrollscan.com transaction test case
    (
        &hex!("02c686c20d662af0be0acd53c967bb5ab0e394b98f85aeed49d076c6d52c83820411ea3fc94c94032e7791b23cdb8368380d7a00762b125f89c5490feace0f60198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d1ca5e9d498a83550c0a8e13e7beb0be41c076771f3cf782903d3688a55e042cf28cb7fca9af9a391dd8ab71d05239774e514a386de99c92e2b3ad5577d567dac163bf85f8725147a1a51e30b590d5d7cc757f549f48c5a9fa51a95128bd05d312bebfe77682663e6b96505f944e1d000068da22edec493f11812b45f1c22c2022ca23a8c391fde4f8579c70435c052dda337d64e68e23779382cf56749d34c072ea17456774f28510add4ae5bed915fdc6556b206d5618c9f197b49134596806"),
        Ok(&hex!("0000000000000000000000000000000000000000000000000000000000000001")),
    ),
    // sepolia.scrollscan.com transaction error case
    (
        &hex!("06a7b64af8f414bcbeef455b1da5208c9b592b83ee6599824caa6d2ee9141a76277d002f54436e7da803601aec9cf8740ce99198c18a74286d979cbc9695cd8b1014772f57bb9742735191cd5dcfe4ebbc04156b6878a0a7c9824f32ffb66e8506064e784db10e9051e52826e192715e8d7e478cb09a5e0012defa0694fbc7f5021e2335f3354bb7922ffcc2f38d3323dd9453ac49b55441452aeaca147711b2058e1d5681b5b9e0074b0f9c8d2c68a069b920d74521e79765036d57666c559709f4ca411a3f52f4e0792fd9e792779856719215d3b32a762afe3d5b8c684af90d8ef3d795acd4b35d4366ab22e4ad335273aa59429e26929d0f64583474d9c8203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e2e83fcb0416df2e0599079ad4358237ebbcc051c1ef925d1bbe59ea035bb04bb099ab042fbcc9f89fa4ce34fafb910263660c6d9fff8bf13f5bf456a42547c1918ae4f2015274a7d0d8142d845f7470e120fc800a759415089bc8616b6237c0910485bfaa12fa2c55ec86d7543b3f1884807bb9f8d42a1dd32a4cfe7ac0956520edc9ef9a928b77bea47eacfa88ccbc7db57a01a72dceca1a5fba69947c87f84180f7b5123c02164cebec652f5d75505ea37815294b1e10ad935d116fcf906381332e52cfcf7d8d5e817243c7bd145cc10108c0c005844e0cf93ed0b2ba5d3970451154fd78a0fc8165ca7738d30141cc50fcff56f568c3c9ce3dd831a5357b8248dd1cc1133cb0acdfcd50f4382b5014bcb06c6f06a87a1387e67b5fcd8dcbe22304c3f6044fb716a80985d42c8d90eff4ee51b804ba6d0f3747a8871abd22c0c2e79f5c39b34325fcb28fd86fef7c0959ccb45066a2dd0987ff1ecaa56de902633fe81d90d8002a06c501bcf5d564deb593969c83b8ad013d194f107a1c71d"),
        Err(PrecompileErrors::Error(PrecompileError::Bn128AffineGFailedToCreate)),
    )
];

/// Run all ecPairing test cases
pub fn run_ecpairing_tests() {
    for (input, expected) in ECPAIRING_CASES {
        let result = run_pair(input, 0, 0, u64::MAX);

        match (result, expected) {
            (Ok(output), Ok(expected_bytes)) => {
                assert_eq!(output.bytes.as_ref(), *expected_bytes);
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
