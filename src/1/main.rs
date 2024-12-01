#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(debug_assertions, allow(unused_imports))]
#![cfg_attr(debug_assertions, allow(unused_mut))]
#![cfg_attr(debug_assertions, allow(unused_variables))]

use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn add_safe(a: u32, b: u32) -> Result<u32, &'static str> {
    a.checked_add(b).ok_or("Addition would overflow!")
}

fn distance(a: u32,b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn main() -> io::Result<()> {

    let mut lista: Vec<u32> = Vec::new();
    let mut listb: Vec<u32> = Vec::new();
    
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let numbers: Vec<u32> = line.trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                lista.push(numbers[0]);
                listb.push(numbers[1]);
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    lista.sort();
    listb.sort();

    let mut diff: u32 = 0;

    for (itema, itemb) in lista.iter().zip(listb.iter()) {
        diff = match add_safe(diff, distance(*itema, *itemb)) {
            Ok(value) => value,
            Err(e) => {
                panic!("Overflow error: {}", e);
            }
        }
    }

    println!("Difference Score: {}", diff);

    let mut sim: u32 = 0;

    for itema in lista.iter() {
        let occurances: u32 = listb.iter().fold(0, |acc, &x| acc + (x == *itema) as u32);

        let addend = occurances * *itema;
        sim = match add_safe(sim, addend) {
            Ok(value) => value,
            Err(e) => {
                panic!("Overflow error: {}", e);
            }
        }
    }

    println!("Similarity Score: {}", sim);

    Ok(())
}
