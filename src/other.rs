/* use libc::{c_char, c_int, c_void, size_t};

#[allow(non_snake_case)]
#[no_mangle]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
    //pub fn primesieve_nth_prime(n: i64, start: u64) -> u64;
    pub fn primesieve_nth_prime(n: i64, start:u64) -> u64;
    pub fn primesieve_init(it: *mut primesieve_iterator);
    pub fn primesieve_free_iterator(it: *mut primesieve_iterator);
    pub fn primesieve_skipto(it: *mut primesieve_iterator, start: u64, stop_hint: u64);
    pub fn primesieve_next_prime_wa(it: *mut primesieve_iterator) -> u64;
    pub fn primesieve_prev_prime_wa(it: *mut primesieve_iterator) -> u64;

}

// workaround for static inline functions
 */