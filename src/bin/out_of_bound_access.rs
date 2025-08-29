use core::hint::black_box;
use openvm_sha2::sha256;

const PTR: *const u8 = (512 * 1024 * 1024) as *const u8;
const DATA_SIZE: usize = 1;
openvm::entry!(main);

// This program should terminate with a segmentation fault.
pub fn main() {
    let data = unsafe { core::slice::from_raw_parts(PTR, DATA_SIZE) };
    black_box(sha256(black_box(data)));
}
