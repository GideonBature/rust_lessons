use rand::Rng;
use std::collections::HashMap;
// use std::io;
// use std::io::Write;

// use std::{self, collections::HashMap, io, io::write};
// use std::collections::*;


fn main() {
    let number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}\n", number);

    let mut map = HashMap::new();
    map.insert('G', 32);
    map.insert('B', 30);

    print!("{:#?}\n", map);

}