use libc::{c_int, c_void, size_t};
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

extern "C" {
    pub fn primesieve_nth_prime(n: i64, start: u64) -> u64;
    pub fn primesieve_next_prime_wa(it: *mut primesieve_iterator) -> u64;
    pub fn primesieve_init(it: *mut primesieve_iterator);
    pub fn primesieve_free_iterator(it: *mut primesieve_iterator);
}

impl primesieve_iterator {
    pub fn new() -> primesieve_iterator {
        let mut itr: primesieve_iterator = unsafe { mem::zeroed() };
        unsafe { primesieve_init(&mut itr) };

        itr
    }

    pub fn next_prime(&mut self) -> u64 {
        let p: u64 = unsafe { primesieve_next_prime_wa(self) };

        p
    }
}

impl Drop for primesieve_iterator {
    fn drop(&mut self) {
        unsafe { primesieve_free_iterator(self) };
    }
}
