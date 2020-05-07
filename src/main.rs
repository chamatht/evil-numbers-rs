
use libc::{c_char, c_int, c_void, size_t};
use std::mem;

#[allow(non_snake_case)]
#[no_mangle]
#[repr(C)]
pub struct primesieve_iterator {
    pub i_: size_t,
    pub last_idx_: size_t,
    pub start_: u64,
    pub stop: u64,
    pub stop_hint: u64,
    pub dist: u64,
    pub primes: *mut u64,
    pub vector: *mut c_void,
    pub primeGenerator: *mut c_void,
    pub is_error_: c_int,
}

extern {
    pub fn primesieve_nth_prime(n: i64, start: u64) -> u64;
    pub fn primesieve_next_prime_wa(it: *mut primesieve_iterator) -> u64;
    pub fn primesieve_init(it: *mut primesieve_iterator);
    pub fn primesieve_free_iterator(it: *mut primesieve_iterator);
}

fn main() {
    println!("5th prime is {}", unsafe {primesieve_nth_prime(5, 0)});

    unsafe {
        let mut itr: primesieve_iterator = mem::zeroed();
        primesieve_init(&mut itr);
        for i in 1..10 {
            println!("{}: {}", i, primesieve_next_prime_wa(&mut itr));
        }

        primesieve_free_iterator(&mut itr);
    }
    
}
