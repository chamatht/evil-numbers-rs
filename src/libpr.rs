use libc::{c_int, c_void, size_t};
use std::mem;
#[allow(non_snake_case)]
#[no_mangle]
#[repr(C)]
pub struct primesieve_iterator {
    i_: size_t,
    last_idx_: size_t,
    start_: u64,
    stop: u64,
    stop_hint: u64,
    dist: u64,
    primes: *mut u64,
    vector: *mut c_void,
    primeGenerator: *mut c_void,
    is_error_: c_int,
}

extern "C" {
    //pub fn primesieve_nth_prime(n: i64, start: u64) -> u64;
    fn primesieve_next_prime_wa(it: *mut primesieve_iterator) -> u64;
    fn primesieve_init(it: *mut primesieve_iterator);
    fn primesieve_free_iterator(it: *mut primesieve_iterator);
    fn primesieve_skipto(it: *mut primesieve_iterator, start: u64, stop_hint: u64);
    fn primesieve_get_max_stop() -> u64;
}

impl primesieve_iterator {
    pub fn new(start: u64) -> primesieve_iterator {
        Self::new_with_stop(start, unsafe{primesieve_get_max_stop()})
    }

    pub fn new_with_stop(start: u64, stop_hint: u64) -> primesieve_iterator {
        let mut itr: primesieve_iterator = unsafe { mem::zeroed() };
        unsafe { primesieve_init(&mut itr) };
        unsafe { primesieve_skipto(&mut itr, start, stop_hint) };

        itr
    }
}

impl Iterator for primesieve_iterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let p: u64 = unsafe { primesieve_next_prime_wa(self) };
        
        if p >= self.stop_hint {
            None
        } else {
            Some(p)
        }
    }
}

impl Drop for primesieve_iterator {
    fn drop(&mut self) {
        unsafe { primesieve_free_iterator(self) };
    }
}
