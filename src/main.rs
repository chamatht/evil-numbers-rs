mod libpr;

use crate::libpr::*;

fn main() {
    let mut itr = primesieve_iterator::new();

    for i in 1..10 {
        println!("{}: {:?}", i, itr.next());
    }
}
