#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(debug_assertions, allow(unused_imports))]
#![cfg_attr(debug_assertions, allow(unused_mut))]
#![cfg_attr(debug_assertions, allow(unused_variables))]

use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Read, Error};

fn processMultiplication(arg1: u64, arg2: u64) -> u64 {
    arg1 * arg2
}

fn processOperation(inst: &str) -> u64 {
    let mut ret: u64 = 0;

    let re_op = Regex::new("mul").unwrap();
    let re_arg = Regex::new("[0-9]{1,3}").unwrap();

    for op in re_op.find_iter(inst).map(|m| m.as_str()) {

        let args: Vec<u64> = re_arg
            .find_iter(inst)
            .map(|m| m.as_str().parse::<u64>().unwrap())
            .collect();

        if op == "mul" {
            ret = processMultiplication(args[0], args[1]);
        }

    }

    ret 
}

fn main() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let re = Regex::new("(mul[(][0-9]{1,3},[0-9]{1,3}[)]|do[(][)]|don't[(][)])").unwrap();

    reader.read_to_string(&mut buf)?;

    let mut sum: u64 = 0;

    let mut enabled: bool = true;

    for result in re.find_iter(buf.as_str()).map(|m| m.as_str()) {

        if result == "do()" {
            enabled = true;
        }
        else if result == "don't()" {
            enabled = false;
        }

        if enabled {
            sum += processOperation(result);
        }
    }

    println!("Sum: {}", sum);

    Ok(())
}
