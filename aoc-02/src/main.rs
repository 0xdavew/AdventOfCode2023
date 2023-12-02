use std::io::{BufRead, BufReader};
use std::fs::File;

extern crate libcommon;
mod round;
mod game;

use libcommon::applog;
use libcommon::startup;
use game::Game;

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

    let games = read_games(reader);

    let (red, green, blue) = (12, 13, 14);

    let mut sum_of_possible: u32 = 0;
    for game in games {
        if game.is_possible(red, green, blue) {
            sum_of_possible += game.number;
            if startup::is("debug") {
                applog!("Game {} is possible", game.number);
            }
        } else {
            if startup::is("debug") {
                applog!("Game {} is impossible", game.number);
            }
        }
    }

    applog!("Sum of possible: {}", sum_of_possible);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    let games = read_games(reader);
    let mut sum_of_powers: u32 = 0;
    for game in games {
        sum_of_powers += game.get_power();
    }

    applog!("Sum of powers: {}", sum_of_powers);
}

/*------------------------------------------------------------- read_games - */

fn read_games(reader: BufReader<File>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut game = Game::default();
        game.read(&line);
        games.push(game);
    }

    games
}

/*--------------------------------------------------------- End of main.rs - */