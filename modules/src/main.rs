use std::collections::HashMap;
use rand::Rng;
use std::{ cmp::Ordering, io };
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);
    let _secret_number = rand::thread_rng().gen_range(1..=100);
}
