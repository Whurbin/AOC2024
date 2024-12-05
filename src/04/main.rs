#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(debug_assertions, allow(unused_imports))]
#![cfg_attr(debug_assertions, allow(unused_mut))]
#![cfg_attr(debug_assertions, allow(unused_variables))]

use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Read, Error};

fn read_square_matrix(fname: &str) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];

    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        res.push(line.expect("Failed to read line").chars().collect::<Vec<char>>());
        let res_len = res.len();
        res[res_len - 1].push('Y');
    }
    res.push(vec![]);
    let res_len = res.len();

    for _ in 0..res[0].len() {
        res[res_len - 1].push('Y'); 
    }

    res
}

fn reverse_square_matrix(square_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];

    for row in square_matrix {
        res.push(row.iter().rev().cloned().collect());
    }

    res
}

fn right_rotate_square_matrix(square_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];

    for _ in 0..square_matrix.len() {
        res.push(vec![]);
    }

    for row in (0..square_matrix.len()).rev() {
        for (index, value) in square_matrix[row].iter().enumerate() {
            res[index].push(value.clone());
        }
    }

    res
}

fn half_left_shift_matrix(square_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];

    res = square_matrix.clone(); 

    for row in 0..res.len() {
        for _ in 0..row {
            let front = res[row].remove(0);
            res[row].push(front);
        }
    }

    res
}

fn half_right_shift_matrix(square_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];

    res = square_matrix.clone();

    for row in 0..res.len() {
        for _ in 0..row {
            let last = res[row].pop().unwrap();
            res[row].insert(0, last);
        }
    }

    res
}

fn _find_str_in_square_matrix(str: &str, square_matrix: &Vec<Vec<char>>) -> u32 {
    let mut counter: u32 = 0;

    let re = Regex::new(str).unwrap();

    for row in square_matrix {
        let row_as_string = String::from_iter(row);

        for _ in re.find_iter(&row_as_string).map(|m| m.as_str()) {
            counter += 1;
        }
    }

    counter
}

fn find_str_in_square_matrix(str: &str, square_matrix: &Vec<Vec<char>>) -> u32 {
    let mut counter: u32 = 0;

    counter += _find_str_in_square_matrix(str, &square_matrix);
    counter += _find_str_in_square_matrix(str, &reverse_square_matrix(&square_matrix));

    let right_square_matrix: Vec<Vec<char>> = right_rotate_square_matrix(&square_matrix);

    counter += _find_str_in_square_matrix(str, &right_square_matrix);
    counter += _find_str_in_square_matrix(str, &reverse_square_matrix(&right_square_matrix));

    let hls_square_matrix: Vec<Vec<char>> = right_rotate_square_matrix(&half_left_shift_matrix(&square_matrix));

    counter += _find_str_in_square_matrix(str, &hls_square_matrix);
    counter += _find_str_in_square_matrix(str, &reverse_square_matrix(&hls_square_matrix));

    let hrs_square_matrix: Vec<Vec<char>> = right_rotate_square_matrix(&half_right_shift_matrix(&square_matrix));

    counter += _find_str_in_square_matrix(str, &hrs_square_matrix);
    counter += _find_str_in_square_matrix(str, &reverse_square_matrix(&hrs_square_matrix));

    counter
}

fn find_X_MAS_in_square_matrix(square_matrix: &Vec<Vec<char>>) -> u32 {
    let mut counter: u32 = 0;

    for row in 1..(square_matrix.len()-2) {
        for col in 1..(square_matrix[row].len()-2) {
            if square_matrix[row][col] == 'A' {

                let axis_1: String = [
                    square_matrix[row-1][col-1],
                    square_matrix[row][col],
                    square_matrix[row+1][col+1]
                ].iter().collect();

                let axis_2: String = [
                    square_matrix[row+1][col-1],
                    square_matrix[row][col],
                    square_matrix[row-1][col+1]
                ].iter().collect();

                if axis_1 == "MAS" || axis_1 == "SAM" {
                    if axis_2 == "MAS" || axis_2 == "SAM" {
                        counter += 1;
                    }
                }
            }
        }
    }

    counter
}

fn main() -> io::Result<()> {

    let square_matrix: Vec<Vec<char>> = read_square_matrix("input.txt");
    let num_found: u32 = find_str_in_square_matrix("XMAS", &square_matrix);

    println!("Found XMAS {} times!", num_found);

    let xmas_found = find_X_MAS_in_square_matrix(&square_matrix);
    println!("Found X-MAS {} times!", xmas_found);

    Ok(())
}
