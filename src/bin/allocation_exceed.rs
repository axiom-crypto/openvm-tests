use core::hint::black_box;
openvm::entry!(main);

// Allocator should terminate this program when it exceeds the memory limit.
pub fn main() {
    // Allocate 512MB + 100B, which is larger than the default limit(512MB).
    let n = black_box(512 * 1024 * 1024 + 100);
    let v = vec![black_box(0u8); n];
    black_box(v[n - 1]);
}
