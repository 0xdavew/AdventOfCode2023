use libcommon::applog;
use libcommon::startup;

use crate::round::Round;

#[derive(Debug, Clone)]
pub struct Game {
    pub number: u32,
    pub rounds: Vec<Round>,
}
impl Default for Game {
    fn default () -> Game {
        Game { number: 0, rounds: Vec::new()}
    }
}
impl Game {
    pub fn read(&mut self, game_desc: &str) {

        if startup::is("debug") {
            applog!("Game: [{}]", game_desc);
        }

        let mut game = game_desc.split(": ");
        let mut game_title = game.next().unwrap().split_whitespace();
        let _name = game_title.next();
        self.number = game_title.next().unwrap().parse::<u32>().unwrap();

        let rounds_info = game.next().unwrap();
        let rounds: Vec<&str> = rounds_info.split("; ").collect();

        for round_info in rounds {
            let round = Round::new(round_info);
            self.rounds.push(round);
        }
    }

    pub fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        for round in self.rounds.iter() {
            if round.red > red || round.green > green || round.blue>blue {
                return false;
            }
        }
        true
    }

    pub fn get_power(&self) -> u32 {
        let (mut min_red, mut min_green, mut min_blue): (u32, u32, u32) = (1, 1, 1);

        for round in self.rounds.iter() {
            if round.red > min_red {
                min_red = round.red;
            }
            if round.green > min_green {
                min_green = round.green;
            }
            if round.blue > min_blue {
                min_blue = round.blue;
            }
        }

        min_red * min_green * min_blue
    }
}

