const CART_TYPES: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const CART_TYPES_PART2: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

use std::{cmp::Ordering, fs::read_to_string};

#[derive(Debug)]
struct Card {
    seme: char,
    value: usize
}

impl Card {

    fn new(seme: &char) -> Card {
        if let Some(valore) = Self::value(seme) {
            Card {
                seme: seme.clone(),
                value: valore,
            }
        } else {
            Card {seme: ' ', value: 0}
        }
    }

    fn value(seme: &char) -> Option<usize> {
        // part 1
        // CART_TYPES.iter().position(|el| el.eq(&seme))
        //part 2
        CART_TYPES_PART2.iter().position(|el| el.eq(&seme))
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    hand_type: u16,
    bid: u64
}

impl Hand {
    fn new(cards: [Card; 5], bid: u64) -> Self {
        let mut hand = Hand { cards, hand_type: 0, bid};
        // part1
        // hand.calc_value();
        // part2
        hand.calc_value_part2();
        hand
    }

    fn calc_value_part2(&mut self) {
        let mut hand_points_map: [usize; 13] = [0; 13];
        self.cards.iter().for_each(|card| if card.value < 13 { hand_points_map[card.value] += 1} else {});

        if hand_points_map.get(0).unwrap() > &0 {
            let j_occurences = hand_points_map[0];
            hand_points_map[0] = 0;
            let max_pos = hand_points_map.iter().position(|el| el == hand_points_map.iter().max().unwrap()).unwrap();
            hand_points_map[max_pos] += j_occurences;
        }

        hand_points_map.sort();
        hand_points_map.reverse();
        self.hand_type = match hand_points_map.get(0) {
            Some(5) => 600, // FIve of a kind
            Some(4) => 500, //Four of a kind
            Some(3) => if *hand_points_map.get(1).unwrap() == 2 { 400 } else { 300 }, // Full House OR Three of a kind
            Some(2) => if *hand_points_map.get(1).unwrap() == 2 { 200 } else { 100 }, // Two pair or One pair
            _ => 0  // Two pair or One pair
        };
    }

    fn calc_value(&mut self) {
        let mut hand_points_map: [usize; 13] = [0; 13];
        self.cards.iter().for_each(|card| if card.value < 13 { hand_points_map[card.value] += 1} else {});
        hand_points_map.sort();
        hand_points_map.reverse();
        self.hand_type = match hand_points_map.get(0) {
            Some(5) => 600, // FIve of a kind
            Some(4) => 500, //Four of a kind
            Some(3) => if *hand_points_map.get(1).unwrap() == 2 { 400 } else { 300 }, // Full House OR Three of a kind
            Some(2) => if *hand_points_map.get(1).unwrap() == 2 { 200 } else { 100 }, // Two pair or One pair
            _ => 0  // Two pair or One pair
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.hand_type > self.hand_type {
            Ordering::Less
        } else if other.hand_type < self.hand_type {
            Ordering::Greater
        } else {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .fold(Ordering::Equal, |acc, (this_card, other_card)| { 
                    if acc != Ordering::Equal {return acc}
                    if this_card.value > other_card.value { Ordering::Greater } else if this_card.value < other_card.value { Ordering::Less } else {Ordering::Equal}
                } )
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}


fn main() {
    let inputs: String = read_to_string("input.txt").expect("File with inputs");
    // let sum = part1(&inputs);
    let sum2 = part2(&inputs);
    // println!("Sum 1: {sum}");
    println!("Sum 2: {sum2}");
}

fn part1(inputs: &str) -> u64 {
    let mut hands = parse_file(&inputs);
    // println!("{:?}", hands);
    hands.sort();
    // println!("{:?}", hands);
    hands.iter().enumerate().fold(0, |acc, e_h| acc + ((e_h.0 + 1)  as u64)* e_h.1.bid)
}

fn part2(inputs: &str) -> u64 {
    let mut hands = parse_file(&inputs);
    // println!("{:?}", hands);
    hands.sort();
    // println!("{:?}", hands);
    hands.iter().enumerate().fold(0, |acc, e_h| acc + ((e_h.0 + 1)  as u64)* e_h.1.bid)
}

fn parse_file(inputs: &str) -> Vec<Hand> {
    inputs
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|game| {
            let cards = game[0].chars().map(|c| Card::new(&c)).collect::<Vec<Card>>();
            Hand::new(cards.try_into().expect("AAAAAA non ci sono 5 carte in mano"), game[1].parse().expect("AAAAAAA la puntata non è valida"))
        })
        .collect::<Vec<Hand>>()
        
}