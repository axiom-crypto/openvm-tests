use core::hint::black_box;
openvm::entry!(main);

pub fn main() {
    let n: u32 = black_box(512 * 1024 * 1024 + 100);
    let zero = black_box(0);
    // This program should panic here because of division by zero.
    let v = n / zero;
    black_box(v);
}
