use std::env;

/*

Note: May have to use bigger data types to support our targer num (384 bits - at least)

BigUint may do the trick:
https://stackoverflow.com/questions/50504503/is-there-a-limit-to-the-size-of-a-bigint-or-biguint-in-rust

For now, I'm just going to use i32

*/

fn main() {
    let until = env::args().collect::<Vec<String>>()[1].parse::<i32>().unwrap();
    print!("Calculating primes up to {}\n", until);

    // This code is meant to be as fast as possible, to do so, we will use lot of contiguos memory (Vec).
    // This consumes more memory, but it's faster than using linked lists, since we cannot access 
    // elements in given index in constant time (O(n) instead of O(1))

    let mut nums: Vec<i32> = Vec::with_capacity((until - 1) as usize);
    let mut primes: Vec<i32> = Vec::with_capacity((until - 1) as usize);

    print!("Allocated {} positions for arrays\n", nums.capacity());


    // Create list with all numbers from 2 to until
    for i in 2..=until {
        nums.push(i);
    }

    print!("Nums: {:?}\n", nums);

    // Remove non-primes (sieve of Eratosthenes)
    let mut i: usize = 0;
    loop {
        if i >= nums.len() {
            break;
        }

        let inc = nums.get(i).unwrap().clone();
        
        // Not prime
        if inc == 0 {
            i += 1;
            continue;
        }

        // Prime
        primes.push(inc);

        // Remove all multiples of this prime
        let mut temp = i + inc as usize;
        while temp < nums.len() {
            nums[temp] = 0;
            temp += inc as usize;
        }

        i += 1;
    }

    print!("Primes: {:?}\n", primes);
    primes.shrink_to_fit();

}
