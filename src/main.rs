
use libc::{c_char, c_int, c_void, size_t};

extern {
    pub fn primesieve_nth_prime(n: i64, start: u64) -> u64;
}

fn main() {
    println!("5th prime is {}", unsafe {primesieve_nth_prime(5, 0)});
}
