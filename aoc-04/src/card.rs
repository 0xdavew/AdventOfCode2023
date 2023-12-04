use libcommon::applog;
use libcommon::startup;

#[derive(Debug, Clone)]
pub struct Card {
    pub number: u32,
    pub winning_numbers: Vec<u32>,
    pub my_numbers: Vec<u32>,
}
impl Default for Card {
    fn default () -> Card {
        Card { number: 0, winning_numbers: Vec::new(), my_numbers: Vec::new()}
    }
}
impl Card {
    pub fn read(&mut self, card_info: &str) {

        if startup::is("debug") {
            applog!("Card: [{}]", card_info);
        }

        let mut card = card_info.split(": ");
        let mut card_title = card.next().unwrap().split_whitespace();
        let _name = card_title.next();
        self.number = card_title.next().unwrap().parse::<u32>().unwrap();

        let card_numbers: Vec<&str> = card.next().unwrap().split(" | ").collect();

        self.winning_numbers = card_numbers.get(0).unwrap().split_whitespace().
            map(|s| s.parse::<u32>().unwrap()).collect();

        self.my_numbers = card_numbers.get(1).unwrap().split_whitespace().
            map(|s| s.parse::<u32>().unwrap()).collect();
    }

    pub fn count_winning_numbers(&self) -> u32 {
        let mut winning_numbers: u32 = 0;
        for number in self.my_numbers.iter() {
            if self.winning_numbers.contains(number) {
                winning_numbers += 1;
            }
        }
        winning_numbers
    }

    pub fn calculate_points(&self) -> u32 {
        let mut points: u32 = 0;
        let winning_numbers = self.count_winning_numbers();
        if winning_numbers>0 {
            points = 1;
            if winning_numbers>1 {
                points <<= winning_numbers-1;
            }
        }
        points
    }
}
