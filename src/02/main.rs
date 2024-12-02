#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(debug_assertions, allow(unused_imports))]
#![cfg_attr(debug_assertions, allow(unused_mut))]
#![cfg_attr(debug_assertions, allow(unused_variables))]

use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn recordDampener(record: &Vec<i32>) -> Vec<i32> {
    let mut change: i32 = 0;
    let mut safe: bool = true;
    let mut direction: bool = false;

    let mut ret: Vec<i32> = vec![];

    for idx in 0..record.len() {
        if idx == 0 {
            change = 0;
            direction = false;
            continue;
        }

        change = record[idx] - record[idx - 1];

        if idx == 1 && change < 0 {
            direction = true;
        }

        if change == 0 {
            let mut recordRemoveIdx: Vec<i32> = record.clone();
            recordRemoveIdx.remove(idx);
            ret = recordRemoveIdx;
            break;
        }
        else if change.abs() > 0 && change.abs() < 4 {
            if change < 0 && direction {
                continue;
            }
            else if change > 0 && !direction {
                continue;
            }
        }

        // There is either too big of a change... or the direction changed

        let mut recordRemoveA: Vec<i32> = record.clone();
        recordRemoveA.remove(idx - 1);

        let mut recordRemoveB: Vec<i32> = record.clone();
        recordRemoveB.remove(idx);

        if recordIsSafe(&recordRemoveA)
        {
            ret = recordRemoveA;
            break;
        }

        if recordIsSafe(&recordRemoveB)
        {
            ret = recordRemoveB;
            break;
        }

        if idx > 1 {
            let mut recordRemoveC: Vec<i32> = record.clone();
            recordRemoveC.remove(idx - 2);

            if recordIsSafe(&recordRemoveC)
            {
                ret = recordRemoveC;
                break;
            }
        }

        break;
        
    }

    ret
}

fn recordIsSafe(record: &Vec<i32>) -> bool {
    let mut change: i32 = 0;
    let mut safe: bool = true;
    let mut direction: bool = false;
    for (idx, level) in record.iter().enumerate() {
        if idx == 0 {
            change = 0;
            safe = true;
            direction = false;
            continue;
        }

        change = *level - record[idx - 1];

        if idx == 1 && change < 0 {
            direction = true;
        }

        if change == 0 {
            safe = false;
            break;
        }
        else if change.abs() > 0 && change.abs() < 4 {
            if change < 0 && !direction {
                safe = false;
                break;
            }
            else if change > 0 && direction {
                safe = false;
                break;
            }
        }
        else {
            safe = false;
            break;
        }
    }

    safe
}

fn main() -> io::Result<()> {

    let mut records: Vec<Vec<i32>> = Vec::new();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for record in reader.lines() {
        match record {
            Ok(record) => {
                let levels: Vec<i32> = record.trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                records.push(levels);
            },
            Err(e) => eprintln!("Error {}", e),
        }
    }

    let mut numSafe: usize = 0;
    let mut numUnsafe: usize = 0;

    for record in records.iter() {
        if recordIsSafe(record) {
            numSafe += 1;
        }
        else {
            let dampenedRecord: Vec<i32> = recordDampener(record);

            if dampenedRecord.len() == 0 {
                numUnsafe += 1;
                continue;
            }

            if recordIsSafe(&dampenedRecord) {
                numSafe += 1;
            }
            else {
                numUnsafe += 1;
            }
        }
    }
    
    println!("Safe Records: {}", numSafe);
    println!("Unsafe Records: {}", numUnsafe);

    Ok(())
}
