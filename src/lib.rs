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
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Suit {
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
struct Card {
    suit: Suit,
    number: u8,
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
struct Player {
    cards: Vec<Card>,
    chips: u32,
    ip: String,
    folded: bool,
    hand: u8, // value of players hand
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
struct Game {
    players: Vec<Player>,
    deck: Vec<Card>,
    num: u8,
    pool: u32,
    flop: Option<Vec<Card>>,
}
impl Game {
    fn shuffle_deck(&self, deck: &mut Vec<Card>) {
        deck.shuffle(&mut thread_rng());
    }
    // used only at the start of the game
    fn new_game(&self) -> Game {
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
        self.shuffle_deck(&mut deck);
        for _ in 0..num_players {
            players.push(Player {
                cards: [deck.pop().unwrap(), deck.pop().unwrap()].to_vec(),
                chips: 500,
                ip: String::from("localhost"),
                folded: false,
                hand: 0,
            });
        }

        Game {
            deck,
            players,
            num: num_players,
            pool: 0,
            flop: None,
        }
    }
    // Used after the hand is done
    fn cont_game(&self, players: Vec<Player>, deck: &mut Vec<Card>, pool: u32) -> Game {
        let counter = 0;
        let mut new_player: Vec<Player> = Vec::new();
        for player in players {
            deck.push(player.cards[0]);
            deck.push(player.cards[1]);
        }
        self.shuffle_deck(deck);

        for player_num in 0..counter {
            new_player[player_num].cards.push(deck.pop().unwrap());
        }
        Game {
            num: counter as u8,
            deck: deck.to_vec(),
            players: new_player,
            pool,
            flop: None,
        }
    }

    fn check_cards(&self, players: &Vec<Player>, flop: &Vec<Card>) -> Player {
        let mut points: HashMap<String, u8> = HashMap::new();
        for player in players {
            let flush = check_flush(&player.cards, flop);
            let straight = check_straight(&player.cards, flop);
            let pairs = check_pairs(&player.cards, flop);
            if flush == 17 && straight == 18 {}
        }
        Player {
            cards: [Card {
                suit: Suit::Diamonds,
                number: 12,
            }]
            .to_vec(),
            chips: 500,
            ip: String::from("Home"),
            folded: false,
            hand: 0,
        }
    }
}
fn check_pairs(hand: &Vec<Card>, flop: &Vec<Card>) -> u8 {
    let mut card_nums: Vec<u8> = Vec::new();
    for card in flop {
        card_nums.push(card.number);
    }
    let mut counter = 0;
    let mut oneCard: bool = false;
    // Check if same, so later checks are easier
    let card_one;
    let card_two;
    if hand.first().unwrap().number == hand.last().unwrap().number {
        oneCard = true;
    }
    // If there is only one unique card
    if oneCard {
        counter += flop
            .iter()
            .filter(|x| x.number == hand.first().unwrap().number)
            .collect::<Vec<&Card>>()
            .len()
            + 1;
        match counter {
            _ => 0,
            1 => 15,
            2 => 19,
        }
    } else {
        card_one = flop
            .iter()
            .filter(|x| x.number == hand.first().unwrap().number)
            .collect::<Vec<&Card>>()
            .len();
        card_two = flop
            .iter()
            .filter(|x| x.number == hand.last().unwrap().number)
            .collect::<Vec<&Card>>()
            .len();
        if card_one == 1 || card_two == 1 {
            return 13;
        } else if card_one == 1 && card_two == 1 {
            return 14;
        } else if card_one == 1 && card_two == 2 {
            return 18;
        } else if card_one == 2 && card_two == 1 {
            return 18;
        } else if card_one == 2 || card_two == 2 {
            return 15;
        } else if card_one == 3 || card_two == 3 {
            return 19;
        } else {
            return 0;
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
