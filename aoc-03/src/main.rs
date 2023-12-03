use std::io::{BufRead, BufReader};
use std::fs::File;

extern crate libcommon;

use libcommon::applog;
use libcommon::startup;

/*------------------------------------------------------------------- main - */

fn main() {
    let reader = startup::get_reader().unwrap();

    if startup::is("part1") {  
        part1(reader);
    } else {
        part2(reader);
    }  

    applog::end_timestamp(startup::get_start_time());
}

/*------------------------------------------------------------------ part1 - */

fn part1(reader: BufReader<File>) {

    let mut sum_of_adjacent_numbers: u32 = 0;

    let bytes: Vec<Vec<u8>> = read_input(reader);
    let col_count = bytes.get(0).unwrap().len();

    let mut index: usize = 0;
    loop {
        let (mut row, mut col) = get_coords(&bytes, index);
        let number = get_next_number(&bytes, &mut row, &mut col);
        let size = get_number_size(number);

        if number>0 {
            let adjacent: bool = is_gear_adjacent(&bytes, row, col, size);

            if startup::is("debug") {
                applog!("({}, {}): {} adjacent={}", row, col, number, adjacent);
            }

            if adjacent {
                sum_of_adjacent_numbers += number;
            }
        } else {
            break;
        }

        // New index
        index = row*col_count + col + size;
    }

    applog!("Sum of adjacent numbers: {}", sum_of_adjacent_numbers)
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    let mut sum_of_gear_ratios: u32 = 0;

    let bytes: Vec<Vec<u8>> = read_input(reader);
    let col_count = bytes.get(0).unwrap().len();

    let mut index: usize = 0;
    loop {
        let (mut row, mut col) = get_coords(&bytes, index);
        if get_next_gear(&bytes, &mut row, &mut col) {

            let ratio = get_gear_ratio(&bytes, row, col);

            if startup::is("debug") {
                applog!("({}, {}): ratio={}", row, col, ratio);
            }

            if ratio>0 {
                sum_of_gear_ratios += ratio;
            }
        } else {
            break;
        }

        // New index
        index = row*col_count + col + 1;
    }

    applog!("Sum of gear ratios: {}", sum_of_gear_ratios)
}

/*------------------------------------------------------------- read_input - */

fn read_input(reader: BufReader<File>) -> Vec<Vec<u8>> {
    let mut byte_array: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let byte_line = line.bytes().collect();
        byte_array.push(byte_line);
    }

    byte_array
}

/*--------------------------------------------------------------- get_byte - */

fn get_byte(bytes: &Vec<Vec<u8>>, row_index: usize, col_index: usize) -> u8 {

    let row_count = bytes.len();
    let col_count = bytes.get(0).unwrap().len();

    // Check bounds
    if row_index>=row_count || col_index>col_count {
        return 0;
    }

    match bytes.get(row_index) {
        Some(row) => {
            match row.get(col_index) {
                Some(byte) => {
                    return byte.clone();
                }
                _ => {
                    return 0;    
                }
            }
        }
        _ => {
            return 0;    
        }
    }
}

/*------------------------------------------------------------- get_coords - */

fn get_coords(bytes: &Vec<Vec<u8>>, index: usize) -> (usize, usize) {
    let col_count = bytes.get(0).unwrap().len();
    let row = index / col_count;
    let col = index % col_count;
    (row, col)
}

/*---------------------------------------------------------- get_next_byte - */

fn get_next_byte(bytes: &Vec<Vec<u8>>, row_index: &mut usize, col_index: &mut usize) -> u8 {
    let row_count = bytes.len();
    let col_count = bytes.get(0).unwrap().len();

    if *col_index < (col_count-2) {
        *col_index += 1;
    } else {
        *row_index += 1;
        *col_index = 0;
    }
    
    // out of range
    if *row_index >= row_count {
        return 0;
    }

    get_byte(bytes, *row_index, *col_index)
}

/*-------------------------------------------------------- get_next_number - */

// Advance to next number
// Don't advance if already at a number
// Return the number value
// Return 0 if no more numbers

fn get_next_number(bytes: &Vec<Vec<u8>>, row_index: &mut usize, col_index: &mut usize) -> u32 {

    let mut number_value: u32 = 0;

    // Jump to next number
    let mut byte = get_byte(&bytes, *row_index, *col_index);
    while byte.is_ascii_digit()==false {
        byte = get_next_byte(&bytes, row_index, col_index);
        if byte == 0 {
            return 0;
        } 
    }

    // Read the number at (row_index, col_index)
    let (row, mut col) = (row_index.clone(), col_index.clone());
    let col_count = bytes.get(0).unwrap().len();
    
    while byte.is_ascii_digit() && col<col_count {
        number_value *= 10;
        number_value += byte as u32 - '0' as u32;
        col += 1;
        byte = get_byte(&bytes, row, col);
    }

    number_value
}

/*-------------------------------------------------------- get_number_size - */

fn get_number_size(number: u32) -> usize {
    let mut size: usize = 0;
    let mut num = number;
    while num>0 {
        num /= 10;
        size += 1;
    }

    size
}

/*------------------------------------------------------- is_gear_adjacent - */

// .......  Previous row: col-1 -> col+size
// ..nnn..  Same row: col-1 and col+size
// .......  Next row: col-1 -> col+size

fn  is_gear_adjacent(bytes: &Vec<Vec<u8>>, row: usize, col: usize, size: usize) -> bool {

    let prev_col = if col>0 {col-1} else {0};
    let prev_row = if row>0 {row-1} else {0};

    // Previous row or next row
    for c in prev_col..col+size+1 {
        if is_engine_part(&bytes, prev_row, c) || is_engine_part(&bytes, row+1, c) {
            return true;
        }
    }

    // Same row
    if is_engine_part(&bytes, row, prev_col) || is_engine_part(&bytes, row, col+size) {
        return true;
    }

    false
}

/*--------------------------------------------------------- is_engine_part - */

fn is_engine_part(bytes: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let byte = get_byte(&bytes, row, col);

    if byte>0 && !byte.is_ascii_digit() && byte != '.' as u8 {
        return true;
    }

    false
}

/*---------------------------------------------------------- get_next_gear - */

// Advance to next gear
// Don't advance if already at a gear
// Return true if gear found
// Return false if no more gears

fn get_next_gear(bytes: &Vec<Vec<u8>>, row_index: &mut usize, col_index: &mut usize) -> bool {

    // Jump to next number
    let mut byte = get_byte(&bytes, *row_index, *col_index);
    while byte != '*' as u8 {
        byte = get_next_byte(&bytes, row_index, col_index);
        if byte == 0 {
            return false;
        } 
    }

    true
}

/*--------------------------------------------------------- get_gear_ratio - */

// 8 spots to check for a number
// ...
// .*.
// ...

fn get_gear_ratio(bytes: &Vec<Vec<u8>>, row: usize, col: usize) -> u32 {

    let mut ratio = 1;
    let mut number_of_numbers = 0;

    let row_count = bytes.len();
    let col_count = bytes.get(0).unwrap().len();

    let prev_col = if col>0 {col-1} else {0};
    let max_col = if col<(col_count-1) {col+1} else {col_count};

    // Previous row
    if row>0 {
        let mut c = prev_col;
        while c <= max_col {
            let mut r = row-1;
            let number = get_number_at(&bytes, &mut r, &mut c);
            if number>0 {
                ratio *= number;
                number_of_numbers += 1;
            } else {
                c += 1;
            }
        }
    }

    // Same row
    if col>0 {
        let mut r = row;
        let mut c = col-1;
        let number = get_number_at(&bytes, &mut r, &mut c);
        if number>0 {
            ratio *= number;
            number_of_numbers += 1;
        }
    }
    if col<(col_count-1) {
        let mut r = row;
        let mut c = col+1;
        let number = get_number_at(&bytes, &mut r, &mut c);
        if number>0 {
            ratio *= number;
            number_of_numbers += 1;
        }
    }

    // Next row
    if row<(row_count-1) {
        let mut r = row+1;
        let mut c = prev_col;
        while c <= max_col {
            let number = get_number_at(&bytes, &mut r, &mut c);
            if number>0 {
                ratio *= number;
                number_of_numbers += 1;
            } else {
                c += 1;
            }
        }
    }

    if startup::is("debug") {
        applog!("#of#: {}, ratio: {}", number_of_numbers, ratio);
    }

    if number_of_numbers != 2 {
        ratio = 0;
    }

    ratio
}

/*---------------------------------------------------------- get_number_at - */

// Look at (row, col)
// Is it part of a number?
// If so, return value of the number (else 0)
// Advance col to just after the number

fn get_number_at(bytes: &Vec<Vec<u8>>, row: &mut usize, col: &mut usize) -> u32 {

    let mut byte = get_byte(&bytes, *row, *col);
    if !byte.is_ascii_digit() {
        return 0;
    }

    while byte.is_ascii_digit() && *col>0 {
        byte = get_byte(&bytes, *row, *col-1);

        if byte.is_ascii_digit() {
            *col = *col-1;
        } else {
            break;
        }
    }

    let number = get_next_number(&bytes, row, col);
    *col += get_number_size(number);

    number
}

/*--------------------------------------------------------- End of main.rs - */