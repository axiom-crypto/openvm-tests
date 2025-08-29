openvm::entry!(main);

// This program never terminates.
pub fn main() {
    #[allow(while_true)]
    while true {}
}
