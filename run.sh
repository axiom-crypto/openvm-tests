set -ex -o pipefail

# build and transpile
cargo openvm build
# generate proving and verification keys
cargo openvm keygen
# execute the program
cargo openvm run
# prove the program execution
cargo openvm prove app | tee prove-app.log
# recursively prove the verification of proof
cargo openvm prove evm | tee prove-evm.log
