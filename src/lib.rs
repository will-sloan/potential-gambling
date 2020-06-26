mod utils;

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::prelude::*;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

extern crate js_sys;

// use serde::Serialize;
// #[macro_use]
// extern crate serde_derive;
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
/*
#[wasm_bindgen]
pub fn get_value_from_js(value: JsValue) -> Result<(), JsValue> {
    let value: Card = serde_wasm_bindgen::from_value(value)?;
    Ok(())
}

// use js_sys::Array;

// pub fn card_game() -> Array {
//     Game::new_game().into_iter()
// }
// use rand::seq::SliceRandom;
// use rand::thread_rng;
/*

heart = 0
club = 1
diamond = 2
spades= 3
error = 4

*/

#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Suit {
    Heart = 0,
    Club = 1,
    Diamonds = 2,
    Spades = 3,
    Error = 4,
}
fn suit_as_val(s: &Suit) -> &u8 {
    match s {
        Suit::Heart => &0,
        Suit::Club => &1,
        Suit::Diamonds => &2,
        Suit::Spades => &3,
        Suit::Error => &4,
    }
}
#[wasm_bindgen]
pub fn pass_value_to_js() -> Result<JsValue, JsValue> {
    //serde_wasm_bindgen::Error
    // let p = Person {
    //     name: String::from("Hello"),
    //     age: 13,
    //     phones: vec![
    //         String::from("phone"),
    //         String::from("AAAAA"),
    //         String::from("Guacamole"),
    //     ],
    // };
    serde_wasm_bindgen::to_value(&Game::new_game()).map_err(|err| err.into())
}
#[wasm_bindgen]
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
#[wasm_bindgen]
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
    //pub num: u8,
    pub pool: u32,
    pub flop: Vec<Card>,
}

pub fn shuffle_deck(deck: &mut Vec<Card>) {
    deck.shuffle(&mut thread_rng());
}
impl Serialize for Game {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Game", 5)?;
        s.serialize_field("players", &self.players)?;
        s.serialize_field("deck", &self.deck)?;
        s.serialize_field("pool", &self.pool)?;
        s.serialize_field("flop", &self.flop)?;
        //s.serialize_field("num", &self.num)?;
        s.end()
    }
}
impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Player", 5)?;
        s.serialize_field("cards", &self.cards)?;
        s.serialize_field("chips", &self.chips)?;
        s.serialize_field("ip", &self.ip)?;
        s.serialize_field("folded", &self.folded)?;
        s.serialize_field("hand", &self.hand)?;
        //s.serialize_field("num", &self.num)?;
        s.end()
    }
}
impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Card", 2)?;
        s.serialize_field("number", &self.number)?;
        s.serialize_field("suit", suit_as_val(&self.suit))?;
        //s.serialize_field("num", &self.num)?;
        s.end()
    }
}

//#[derive(Serialize)]
*/

// A working example of wasm
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

// This is what #[derive(Serialize)] would generate.
impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Person", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("age", &self.age)?;
        s.serialize_field("phones", &self.phones)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Person {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Name,
            Age,
            Phones,
        };

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`name` or `age` or `phones`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "name" => Ok(Field::Name),
                            "age" => Ok(Field::Age),
                            "phones" => Ok(Field::Phones),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Person;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            // fn visit_seq<V>(self, mut seq: V) -> Result<Person, V::Error>
            // where
            //     V: SeqAccess<'de>,
            // {
            //     let name = seq
            //         .next_element()?
            //         .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            //     let age = seq
            //         .next_element()?
            //         .ok_or_else(|| de::Error::invalid_length(1, &self))?;
            //     let phones = seq
            //         .next_element()?
            //         .ok_or_else(|| de::Error::invalid_length(2, &self))?;
            //     Ok(Person { name, age, phones })
            // }

            fn visit_map<V>(self, mut map: V) -> Result<Person, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut age = None;
                let mut phones = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Age => {
                            if age.is_some() {
                                return Err(de::Error::duplicate_field("age"));
                            }
                            age = Some(map.next_value()?);
                        }
                        Field::Phones => {
                            if phones.is_some() {
                                return Err(de::Error::duplicate_field("phones"));
                            }
                            phones = Some(map.next_value()?);
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let age = age.ok_or_else(|| de::Error::missing_field("age"))?;
                let phones = phones.ok_or_else(|| de::Error::missing_field("phones"))?;
                Ok(Person { name, age, phones })
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "age", "phones"];
        deserializer.deserialize_struct("Person", FIELDS, DurationVisitor)
    }
}
//use std::io::Error;
#[wasm_bindgen]
pub fn pass_value_to_js() -> Result<JsValue, JsValue> {
    //serde_wasm_bindgen::Error
    let p = Person {
        name: String::from("Hello"),
        age: 13,
        phones: vec![
            String::from("phone"),
            String::from("AAAAA"),
            String::from("Guacamole"),
        ],
    };
    serde_wasm_bindgen::to_value(&p).map_err(|err| err.into())
}

#[wasm_bindgen]
pub fn increment_num(value: JsValue) -> Result<JsValue, JsValue> {
    let mut value: Person = serde_wasm_bindgen::from_value(value)?;
    value.age += 1;
    serde_wasm_bindgen::to_value(&value).map_err(|err| err.into())
}

// #[wasm_bindgen]
// pub fn get_value_from_js(value: JsValue) -> Result<(), JsValue> {
//     let value: Person = serde_wasm_bindgen::from_value(value)?;

//     Ok(())
// }

/*
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
        shuffle_deck(&mut deck);
        Game {
            deck,
            players,
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
*/
