use std::collections::LinkedList;
use std::env;
use std::ops::Add;
use std::ops::Div;
use std::thread;
use std::sync::mpsc;
use std::usize;
use num_bigint::BigUint;
use rand::Rng;

/*

Note: May have to use bigger data types to support our targer num (384 bits - at least)

BigUint may do the trick:
https://stackoverflow.com/questions/50504503/is-there-a-limit-to-the-size-of-a-bigint-or-biguint-in-rust

*/

// Probabilistic test for primality
static K: i32 = 5;
fn fermat_test(num: BigUint) -> bool {
    if num < BigUint::from(2 as usize) {
        return false;
    } else if num == BigUint::from(2 as usize) || num == BigUint::from(3 as usize) || num == BigUint::from(5 as usize) {
        return true;
    } 

    // Generate random number
    let a: BigUint = BigUint::from(rand::thread_rng().gen_range(2..u128::MAX));

    for _ in 0..K {
        if a.modpow(&(num.clone() - BigUint::from(1 as usize)), &num) != BigUint::from(1 as usize) {
            return false;
        }
    }

    true
}

fn main() {
    // Input format: nf <nthreads> <num>
    let nthreads = env::args().collect::<Vec<String>>()[1].parse::<i32>().unwrap();
    let num = env::args().collect::<Vec<String>>()[2].parse::<BigUint>().unwrap();
    
    println!("Number of threads: {}", nthreads);
    println!("Number to factor: {}\n", num);

    // Testing multithreading in Rust
    let final_bound = num.clone().div(2 as usize).add(1 as usize);
    let increment = final_bound.clone().div(nthreads as usize);
    let mut start_point = BigUint::from(0 as usize);
    let mut end_point = increment.clone();

    println!("Final bound: {}", final_bound);
    println!("Increment: {}", increment);
    println!("Start point: {}", start_point);
    println!("End point: {}\n", end_point);

    println!("Thread setup:\n");

    let (tx, rx) = mpsc::channel(); 

    for i in 0..nthreads {
        let tx_clone = tx.clone();
        let st_clone = start_point.clone();
        
        let en_clone = if i + 1 == nthreads {
            final_bound.clone()
        } else {
            end_point.clone()
        };

        let num_clone = num.clone();

        println!("Start point: {}", st_clone);
        println!("End point: {}", en_clone);

        thread::spawn(move || {
            let mut factors: LinkedList<BigUint> = LinkedList::new();

            let mut s = st_clone;
            let e = en_clone;

            while s <= e {
                if fermat_test(s.clone()) &&
                    num_clone.modpow(&BigUint::from(1 as usize), &s) == BigUint::from(0 as usize){
                    factors.push_back(s.clone());
                }
                s = s.add(BigUint::from(1 as usize));
            }
            tx_clone.send(factors).unwrap();
        });

        start_point += increment.clone();
        end_point += increment.clone();
    }

    drop(tx);

    let it = rx.iter();
    let mut final_primes: LinkedList<BigUint> = LinkedList::new();
    it.for_each(|a| {
        final_primes.append(&mut a.clone());
    });

    print!("\n{:?}", final_primes);
    
}
