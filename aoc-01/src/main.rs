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

    let mut calibration_total: u32=0;

    for line in reader.lines() {
        let line = line.unwrap();
        calibration_total += get_calibration_value(line);
    }

    applog!("Calibration value: {}", calibration_total);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    let mut calibration_total: u32=0;

    for line in reader.lines() {
        let line = line.unwrap();
        let simplified = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");

            calibration_total += get_calibration_value(simplified);
    }

    applog!("Calibration value: {}", calibration_total);
}

/*-------------------------------------------------- get_calibration_value - */

fn get_calibration_value(line: String) -> u32 {
    let numerics: String = line.chars().filter(|c| c.is_numeric()).collect();
    let first_char: char = numerics.chars().next().unwrap();
    let last_char: char = numerics.chars().last().unwrap();

    if startup::is("debug") {
        applog!("line: {}, numerics: {}", line, numerics);
    }

    (first_char as u32 - '0' as u32)*10 + (last_char as u32 - '0' as u32)
}

/*--------------------------------------------------------- End of main.rs - */