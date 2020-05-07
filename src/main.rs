mod libpr;

use crate::libpr::*;

fn main() {
    let itr = primesieve_iterator::new();

    for pit in itr {
        if pit > 100 {
            break;
        }
        println!("{:?}", pit);
    }
}
