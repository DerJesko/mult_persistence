extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigUint, ToBigUint};
use num_traits::checked_pow;
use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    let mut current = checked_pow(10.to_biguint().unwrap(), 5).unwrap();
    let mut max = 0;
    loop {
        let res = calc_persistence(current.clone(), &mut m);
        if res > max {
            max = res;
            println!("persistence: {}, number: {}", max, current);
        }
        current = current + 1 as usize;
    }
}

fn calc_persistence(n: BigUint, set: &mut HashMap<BigUint, usize>) -> usize {
    if n < 10.to_biguint().unwrap() {
        return 0;
    }
    let a = match set.get(&n) {
        Some(i) => *i,
        None => calc_persistence(persistence_step(&n), set) + 1,
    };
    // println!("{}: {}", n, a);
    set.insert(n, a);
    a
}

fn persistence_step(n: &BigUint) -> BigUint {
    let digits = n.to_radix_be(10);
    let mut x = 1.to_biguint().unwrap();
    for i in digits {
        x *= i;
    }
    x
}
