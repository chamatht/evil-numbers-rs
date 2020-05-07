mod libpr;

use crate::libpr::*;
use num_cpus;
use std::env;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

fn main() {
    let (stop, start) = get_args();

    let cpus = num_cpus::get() as u64;
    let dist = stop - start;
    let step = dist / cpus;
    let mut mainvec: Vec<Vec<u64>> = Vec::with_capacity(cpus as usize);

    let (tx, rx) = mpsc::channel();

    for i in 0..cpus {
        let start = i * step;
        let stop = std::cmp::min(start + step, stop);
        let tmptx = tx.clone();

        thread::spawn(move || {
            run_evil_generator(i as usize, tmptx, start, stop);
        });

        mainvec.push(Vec::new());
    }

    drop(tx);

    for (i, evec) in rx {
        //println!("item recieved {} with length {} : {:?}", i, evec.len(), evec.get(0));
        mainvec.insert(i, evec);
    }

    let evilvec: Vec<&u64> = mainvec.iter().flat_map(|v| v.iter()).collect();

    println!("Largets evil number: {}", evilvec.last().unwrap());
}

fn run_evil_generator(identifier: usize, tx: Sender<(usize, Vec<u64>)>, start: u64, stop: u64) {
    let itr = primesieve_iterator::new_with_stop(start, stop);
    let mut v: Vec<u64> = Vec::new();
    for p in itr {
        let digit_sum: u32 = p.to_string().chars().map(|d| d.to_digit(10).unwrap()).sum();

        if digit_sum % 7 == 0 {
            v.push(p);
        }
    }

    tx.send((identifier, v)).unwrap();
}

fn get_args() -> (u64, u64) {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let t = (
            args[1].parse::<u64>().unwrap(),
            args[2].parse::<u64>().unwrap(),
        );
        if t.0 <= t.1 {
            panic!("Stop value must be bigger than start.")
        }

        t
    } else if args.len() == 2 {
        (args[1].parse::<u64>().unwrap(), 0)
    } else {
        panic!("USAGE: {} [STOP_VALUE]", args[0])
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_single_thread() {
        let itr = primesieve_iterator::new_with_stop(0, 3012);
        let v: Vec<u64> = itr.collect();
        assert_eq!(Some(&2), v.first());
        assert_eq!(Some(&3011), v.last());
    }

    #[test]
    fn test_single_thread_with_start() {
        let itr = primesieve_iterator::new_with_stop(3000, 5000);
        let v: Vec<u64> = itr.collect();
        assert_eq!(Some(&3001), v.first());
        assert_eq!(Some(&4999), v.last());
    }

    #[test]
    fn test_multi_thread() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            run_evil_generator(0, tx, 0, 3002);
        });

        for (_i, v) in rx {
            assert_eq!(Some(&7), v.first());
            assert_eq!(Some(&2903), v.last());
        }
    }
}
