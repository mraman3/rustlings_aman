// TODO: Add the missing type of the argument `num` after the colon `:`.

// Note Use u32 for code that does not require
// negatives as it will compile quicker then i32
fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
