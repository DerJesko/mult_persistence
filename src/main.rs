extern crate num_bigint;
extern crate num_traits;

use num_bigint::ToBigUint;
use num_traits::checked_pow;
use std::collections::HashMap;

const BASE: u32 = 10;

fn main() {
    let mut m = HashMap::new();
    let mut current = checked_pow(10.to_biguint().unwrap(), 5).unwrap();
    let mut max = 0;
    loop {
        let res = calc_persistence(current.to_radix_be(BASE), &mut m);
        if res > max {
            max = res;
            println!("persistence: {}, number: {}", max, current);
        }
        current = current + 1 as usize;
    }
}

fn calc_persistence(n: Vec<u8>, set: &mut HashMap<Vec<u8>, usize>) -> usize {
    if n.len() < 2 {
        return 0;
    }
    let mut m = n;
    m.sort();
    let a = match set.get(&m) {
        Some(i) => *i,
        None => calc_persistence(persistence_step(&m), set) + 1,
    };
    // println!("{}: {}", n, a);
    set.insert(m, a);
    a
}

fn persistence_step(n: &Vec<u8>) -> Vec<u8> {
    let mut x = 1.to_biguint().unwrap();
    for i in n {
        x = x * i;
    }
    x.to_radix_be(BASE)
}
