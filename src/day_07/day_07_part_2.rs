use crate::helper_functions::io::*;
use core::cmp::Ordering;
use core::cmp::Ordering::Equal;
use std::collections::HashMap;
use regex::Regex;


#[derive(Debug)]
struct PokerHand {
    hand :[char; 5],
    hand_type : char,
}

// Determine an order for poker hands
impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = HashMap::from([ ('A', 13), ('K', 12), ('Q', 11), ('J', 0),
                                                        ('T', 9), ('9', 8), ('8', 7), ('7', 6), 
                                                        ('6', 5), ('5', 4), ('4', 3), ('3', 2), ('2', 1)]);
        let type_order = HashMap::from([ ('5', 6), ('4', 5), ('F', 4), ('3', 3), ('2', 2), ('1', 1), ('H', 0) ]);
        // If they have different types, compare the higher one
        if type_order[&self.hand_type] != type_order[&other.hand_type] {
            return type_order[&self.hand_type].cmp(&type_order[&other.hand_type]);
        }

        // Compare each value, finding the highest
        for i in 0..5 {
            let our_card = self.hand[i];
            let their_card = other.hand[i];
            if order[&our_card].cmp(&order[&their_card]) != Equal {
                return order[&our_card].cmp(&order[&their_card]);
            }
        }
        // Otherwise, they are equal
        Equal
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for PokerHand {}

// Determine the hand type from the hand
fn determine_hand_type(hand : [char; 5]) -> char {
    let mut map : HashMap<char, u8> = HashMap::new();
    let mut contains_joker = false;
    for card in hand {
        let mut count = 0;
        if map.contains_key(&card) {
            count = map[&card];
        }
        if card == 'J' {
            contains_joker = true;
        }
        map.insert(card, count + 1);
    }
    // Add in Joker 
    if contains_joker  && map.len() != 1 {
        // Find the maximum value in the map
        let mut max_count = 0;
        let mut max_card = 'J';
        for (card, count) in &map {
            if *card != 'J' && *count > max_count {
                max_count = *count;
                max_card = *card;
            }
        }
        map.insert(max_card, max_count + map[&'J']);
        map.remove(&'J');
    }
    if map.len() == 1 {
        return '5';
    }
    else if map.len() == 2 {
        for (_, count) in map {
            if count == 3 {
                return 'F';
            }
            else if count == 4 {
                return '4';
            }
        }
    }
    else if map.len() == 3 {
        for (_, count) in map {
            if count == 3 {
                return '3';
            }
        }
        return '2';
    }
    else if map.len() == 4 {
        return '1';
    }
    // Otherwise, we have just a high card
    'H'
}

// Get the hand and bid from the input
fn get_hand_bid(line_string : String) -> (PokerHand, u32) {
    let re = match Regex::new(r"(\w+)\s*(\d+)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };

    let mut hand : [char; 5] = ['2'; 5];
    let mut hand_type : char = 'H';
    let mut bid : u32 = 0;
    for (_, [hand_str, bid_str]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        let letters :Vec<char> = hand_str.chars().collect();
        hand = letters.try_into().unwrap();
        bid = bid_str.parse::<u32>().unwrap();
        hand_type = determine_hand_type(hand);
    }
    let pkr_hnd = PokerHand {hand, hand_type};
    (pkr_hnd, bid)
}

// Main function for day 7
pub fn main() {
    // filenames for input
    let filename = "src/day_07/day_07_input.txt";
    // let filename = "src/day_07/test_02.txt";

    let mut hands_bids = vec![];

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                hands_bids.push(get_hand_bid(ip));
            }
        }
        // Sort the hands and their bids based on the hands
        hands_bids.sort_by(|h1, h2| h1.0.cmp(&h2.0));

        // Find result:
        let mut result = 0;
        for (i, (_, bid)) in hands_bids.iter().enumerate() {
            result+= (i as u32 + 1) * bid;
        }
        println!("The result is: {}", result)
    }
}
