mod libpr;

use crate::libpr::*;
use std::mem;

fn main() {
    println!("5th prime is {}", unsafe { primesieve_nth_prime(5, 0) });

    let mut itr = primesieve_iterator::new();
    for i in 1..10 {
        println!("{}: {}", i, itr.next_prime());
    }
}
