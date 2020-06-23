mod utils;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, poker-game!");
}
// use rand::seq::SliceRandom;
// use rand::thread_rng;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Suit {
    Heart = 0,
    Club = 1,
    Diamonds = 2,
    Spades = 3,
    Error = 4,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Card {
    pub suit: Suit,
    pub number: u8,
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct Player {
    pub cards: Vec<Card>,
    pub chips: u32,
    pub ip: String,
    pub folded: bool,
    pub hand: u8, // value of players hand
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    pub num: u8,
    pub pool: u32,
    pub flop: Vec<Card>,
}

pub fn shuffle_deck(deck: &mut Vec<Card>) {
    deck.shuffle(&mut thread_rng());
}

impl Game {
    // used only at the start of the game
    pub fn new_game() -> Game {
        //, num_players: u8
        let num_players: u8 = 2;
        let mut deck: Vec<Card> = Vec::new();
        let mut players: Vec<Player> = Vec::new();
        for i in 0..4 {
            for j in 0..13 {
                match i {
                    0 => deck.push(Card {
                        suit: Suit::Heart,
                        number: j,
                    }),
                    1 => deck.push(Card {
                        suit: Suit::Club,
                        number: j,
                    }),
                    2 => deck.push(Card {
                        suit: Suit::Diamonds,
                        number: j,
                    }),
                    3 => deck.push(Card {
                        suit: Suit::Spades,
                        number: j,
                    }),
                    _ => deck.push(Card {
                        suit: Suit::Error,
                        number: 69,
                    }),
                }
            }
        }
        //shuffle_deck(&mut deck);
        // for _ in 0..num_players {
        //     players.push(Player {
        //         cards: [deck.pop().unwrap(), deck.pop().unwrap()].to_vec(),
        //         chips: 500,
        //         ip: String::from("localhost"),
        //         folded: false,
        //         hand: 0,
        //     });
        // }

        Game {
            deck,
            players,
            num: num_players,
            pool: 0,
            flop: Vec::new(),
        }
    }
    // Used after the hand is done
    pub fn cont_game(&self, players: Vec<Player>, deck: &mut Vec<Card>, pool: u32) -> Game {
        let counter = 0;
        let mut new_player: Vec<Player> = Vec::new();
        for player in players {
            deck.push(player.cards[0]);
            deck.push(player.cards[1]);
        }
        shuffle_deck(deck);

        for player_num in 0..counter {
            new_player[player_num].cards.push(deck.pop().unwrap());
        }
        Game {
            num: counter as u8,
            deck: deck.to_vec(),
            players: new_player,
            pool,
            flop: Vec::new(),
        }
    }

    pub fn deal_to_players(&mut self) {
        // goes puts all cards from players into deck
        // Shuffles the cards
        // then redeals
        for index in 0..self.players.len() {
            println!("Index of players");
            match self.players[index].cards.pop() {
                Some(card) => self.deck.push(card),
                None => {
                    println!("No cards in player");
                    continue;
                }
            }
            match self.players[index].cards.pop() {
                Some(card) => self.deck.push(card),
                None => {
                    println!("No cards in player");
                    continue;
                }
            }
        }

        shuffle_deck(&mut self.deck);

        for index in 0..self.players.len() {
            println!("Index of deck");
            match self.deck.pop() {
                Some(card) => self.players[index].cards.push(card),
                None => panic!("No cards in deck"),
            }
            match self.deck.pop() {
                Some(card) => self.players[index].cards.push(card),
                None => panic!("No cards in deck"),
            }
            // self.players[index].cards.push(self.deck.pop().unwrap());
            // self.players[index].cards.push(self.deck.pop().unwrap());
        }

        //self.players.cards.push(self.deck.pop().unwrap());
        //self.players.cards.push(self.deck.pop().unwrap());
        //
    }

    pub fn check_cards(&mut self) -> String {
        let mut points: HashMap<Player, u8> = HashMap::new();
        for index in 0..self.players.len() {
            let flush = check_flush(&self.players[index].cards, &self.flop.clone());
            let straight = check_straight(&self.players[index].cards, &self.flop.clone());
            let pairs = check_pairs(&self.players[index].cards, &self.flop.clone());

            let value_of_hand = [
                flush,
                straight,
                pairs,
                self.players[index].cards.last().unwrap().number,
                self.players[index].cards.first().unwrap().number,
            ]
            .iter()
            .max()
            .unwrap()
            .clone();
            self.players[index].hand = value_of_hand;
            points.insert(self.players[index].clone(), value_of_hand);
        }
        let mut max_player = self.players.first().unwrap();
        let mut max_val = 0;
        for (k, v) in points.iter() {
            if *v > max_val as u8 {
                max_player = k;
                max_val = *v;
            }
        }

        return max_player.ip.clone();
        // Player {
        //     cards: [Card {
        //         suit: Suit::Diamonds,
        //         number: 12,
        //     }]
        //     .to_vec(),
        //     chips: 500,
        //     ip: String::from("Home"),
        //     folded: false,
        //     hand: 0,
        // }
    }

    pub fn do_flop(&mut self) {
        for i in 0..3 {
            self.flop.push(self.deck.pop().unwrap());
        }
    }

    pub fn flip_one(&mut self) {
        self.flop.push(self.deck.pop().unwrap());
    }
}
fn check_pairs(hand: &Vec<Card>, flop: &Vec<Card>) -> u8 {
    let mut temp_hand = hand.clone();
    let mut temp_flop = flop.clone();
    temp_hand.append(&mut temp_flop);
    // Hashmap to store the number of occuances of each number
    let mut count_of_each_card: HashMap<u8, u8> = HashMap::new();

    for card in temp_hand {
        let count = count_of_each_card.entry(card.number).or_insert(0);
        *count += 1;
    }

    let mut vals = count_of_each_card.values().collect::<Vec<&u8>>();
    vals.sort_by(|a, b| b.cmp(a));
    println!("{:?}", count_of_each_card);
    /*
    Possible Combos:
    Four of a kind: max_val[0] == 4
    Full house: max_val[0] == 3 and max_val == 2
    3 of a kind: max_val[0] == 3
    2 pair: max_val[0] && max_val[1] == 2
    pair: max_val[0] == 2
    */
    if vals.len() > 1 {
        if *vals[0] == 4 {
            19 as u8
        } else if *vals[0] == 3 && *vals[1] >= 2 {
            // could be another 3 cards
            18 as u8
        } else if *vals[0] == 3 {
            15 as u8
        } else if *vals[0] == 2 && *vals[1] == 2 {
            14 as u8
        } else if *vals[0] == 2 {
            13 as u8
        } else {
            0 as u8
        }
    } else {
        if *vals[0] == 4 {
            19 as u8
        } else if *vals[0] == 3 {
            15 as u8
        } else if *vals[0] == 2 {
            13 as u8
        } else {
            0 as u8
        }
    }
}

fn check_straight(hand: &Vec<Card>, flop: &Vec<Card>) -> u8 {
    let mut temp_group: Vec<u8> =
        [hand.last().unwrap().number, hand.first().unwrap().number].to_vec();
    let a = temp_group.contains(&12); // check 1,2,3,4,5 and 10,11,12,13,14
    for i in flop {
        temp_group.push(i.number);
    }
    if temp_group.len() >= 5 {
        temp_group.sort();
        if a {
            // then check 1,2,3,4,5 explicitly
            for i in 0..5 {
                if i as u8 != temp_group[i] {
                    return 0;
                }
            }
            return 16;
        }
        for i in 0..temp_group.len() - 2 {
            if (i + 1) as u8 != temp_group[i + 1] {
                return 0;
            }
        }
        return 16;
    } else {
        return 0;
    }
}

fn check_flush(hand: &Vec<Card>, flop: &Vec<Card>) -> u8 {
    let mut occurances_card_one: u8 = 0;
    let mut occurances_card_two: u8 = 0;
    let straight = check_straight(hand, flop) == 16;
    let mut cards: Vec<Suit> = [hand.last().unwrap().suit, hand.last().unwrap().suit].to_vec();
    for i in flop {
        cards.push(i.suit);
    }

    if hand.last().unwrap().suit == hand.last().unwrap().suit {
        //suit_to_check_one = hand.last().unwrap().suit;
        if cards
            .iter()
            .filter(|x| **x == hand.last().unwrap().suit)
            .collect::<Vec<&Suit>>()
            .len() as u8
            >= 5
        {
            if straight {
                return 20;
            } else {
                return 17;
            }
        } else {
            return 0;
        }
    } else {
        occurances_card_one = cards
            .iter()
            .filter(|x| **x == hand.last().unwrap().suit)
            .collect::<Vec<&Suit>>()
            .len() as u8;
        occurances_card_two = cards
            .iter()
            .filter(|x| **x == hand.first().unwrap().suit)
            .collect::<Vec<&Suit>>()
            .len() as u8;
        if occurances_card_one >= 5 || occurances_card_two >= 5 {
            return 16;
        } else {
            return 0;
        }
    }
}
