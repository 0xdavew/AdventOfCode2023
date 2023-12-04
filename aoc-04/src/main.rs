use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::fs::File;

extern crate libcommon;
mod card;

use libcommon::applog;
use libcommon::startup;
use card::Card;

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

    let cards = read_cards(reader);

    let mut card_points: u32 = 0;

    for card in cards {
        card_points += card.calculate_points();
    }

    applog!("Total card points: {}", card_points)
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    let cards = read_cards(reader);

    let mut total_scratchcards: u32 = 0;

    // Queue keeping track of extra copies of each upcoming scratchcard
    let mut extra_cards: VecDeque<u32> = VecDeque::new();

    for card in cards {
        let extra_copies = extra_cards.pop_front().unwrap_or(0);
        let copies_of_this_card = extra_copies + 1; // +1 for current card

        let cards_won = card.count_winning_numbers();
        if cards_won>0 {
            add_cards_won(&mut extra_cards, copies_of_this_card, cards_won);
        }
        total_scratchcards += copies_of_this_card;
    }

    applog!("Total scratchcards: {}", total_scratchcards);
}

/*------------------------------------------------------------- read_cards - */

fn read_cards(reader: BufReader<File>) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut card = Card::default();
        card.read(&line);
        cards.push(card);
    }

    cards
}

/*---------------------------------------------------------- add_cards_won - */

// Add cards won to the VecDeque
// Add number_of_cards to each slot
// Next item in queue is first
// If you get to end of queue, add another item

fn add_cards_won(extra_cards: &mut VecDeque<u32>, number_of_cards: u32, number_of_slots: u32) {

    for s in 0..number_of_slots {
        if let Some(slot) = extra_cards.get_mut(s as usize) {
            *slot += number_of_cards;
        } else {
            extra_cards.push_back(number_of_cards);
        }
    }
}

/*--------------------------------------------------------- End of main.rs - */