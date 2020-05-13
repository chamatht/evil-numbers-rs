mod libpr;

use crate::libpr::*;
use std::env;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Instant;

fn main() {
    let (stop, start) = get_args();

    let cpus = num_cpus::get() as u64;
    let dist = stop - start;
    let step = dist / cpus;
    let mut mainvec: Vec<Vec<u64>> = Vec::with_capacity(cpus as usize);

    let (tx, rx) = mpsc::channel();

    let itime = Instant::now();
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
        if !evec.is_empty() {
            mainvec.insert(i, evec);
        }
    }
    let ftime = itime.elapsed();

    let mut evilvec: Vec<&u64> = mainvec
        .iter()
        .filter(|v| !v.is_empty())
        .flat_map(|v| v.iter())
        .collect();
    while let None = evilvec.last() {
        evilvec.pop();
    }

    println!("{} evil numbers found.", evilvec.len());
    println!("Largets evil number: {}", evilvec.last().unwrap());
    let (eh, pos) = find_largest_evil_at_heart(&evilvec);
    println!(
        "Largets evil at heart number: {} (its position is {})",
        eh, pos
    );
    println!("*** Took {:.3} seconds ***", ftime.as_secs_f64());
}

fn run_evil_generator(identifier: usize, tx: Sender<(usize, Vec<u64>)>, start: u64, stop: u64) {
    let itr = primesieve_iterator::new_with_stop(start, stop);
    let mut v: Vec<u64> = Vec::new();
    for p in itr {
        let mut i = p;
        let mut sum = 0;
        while i > 0 {
            sum += i % 10;
            i = i / 10;
        }
        
        if sum % 7 == 0 {
            v.push(p);
        }
    }

    tx.send((identifier, v)).unwrap();
}

fn find_largest_evil_at_heart(evilvec: &[&u64]) -> (u64, usize) {
    let mut ehnum: u64 = 0;
    let mut ehpos: usize = 0;

    let mut i: usize = 0;
    while (*evilvec[i] as usize) < evilvec.len() {
        ehpos = *evilvec[i] as usize;
        ehnum = *evilvec[ehpos];
        i = i + 1;
    }

    (ehnum, ehpos)
}

fn get_args() -> (u64, u64) {
    let args: Vec<String> = env::args().collect();
    let t = {
        if args.len() == 3 {
            (
                args[1].parse::<u64>().unwrap(),
                args[2].parse::<u64>().unwrap(),
            )
        } else if args.len() == 2 {
            (args[1].parse::<u64>().unwrap(), 0)
        } else {
            eprintln!(
                "USAGE: {} STOP_VALUE [START_VALUE]
            Note: [START_VALUE, STOP_VALUE) ",
                args[0]
            );
            std::process::exit(2);
        }
    };

    if t.0 <= 7 {
        eprintln!("Start value must be bigger than 7.");
        std::process::exit(1);
    }
    if t.0 <= t.1 {
        eprintln!("Stop value must be bigger than start.");
        std::process::exit(1);
    }

    t
}

#[cfg(test)]
mod tests {
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
