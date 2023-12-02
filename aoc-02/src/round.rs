
use libcommon::applog;
use libcommon::startup;

#[derive(Debug, Clone)]
pub struct Round {
    pub red: u32,
    pub blue: u32,
    pub green: u32
}
impl Default for Round {
    fn default () -> Round {
        Round { red: 0, blue: 0, green: 0}
    }
}

impl Round {
    pub fn new(input: &str) -> Round {
        let mut round = Round::default();
        round.read(input);
        round
    }

    fn read(&mut self, round_desc: &str) {
        
        if startup::is("debug") {
            applog!("Round: [{}]", round_desc);
        }
        
        let cubes: Vec<&str> = round_desc.split(", ").collect();

        for cube in cubes {
            if startup::is("debug") {
                applog!("Cube: [{}]", cube);
            }

            let mut colour = cube.split_whitespace();
            let count = colour.next().unwrap().parse::<u32>().unwrap();
            let name = colour.next().unwrap();
            match name {
                "red" => { self.red += count; }
                "green" => { self.green += count; }
                "blue" => { self.blue += count; }
                _ => { applog!("Unknown colour: [{}]", name);}
            }
        }
    }
}
