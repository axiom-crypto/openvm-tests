set -ex -o pipefail

# build and transpile
cargo openvm build
# execute the program
cargo openvm run
# generate proving and verification keys
cargo openvm keygen
# prove the program execution
cargo openvm prove app
# verify the proof
cargo openvm verify app
# download kzg params and aggreggation key
cargo openvm setup
# generate aggregation proof
# cargo openvm prove evm
# verify the evm proof
# cargo openvm verify evm
