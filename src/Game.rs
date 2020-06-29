use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    //pub num: u8,
    pub pool: u32,
    pub flop: Vec<Card>,
    pub winner: String,
}

pub fn shuffle_deck(deck: &mut Vec<Card>) {
    deck.shuffle(&mut thread_rng());
}

fn create_url(suit: u8, number: u8) -> String {
    let mut s = "http://willsloan.com/cards/".to_string();
    match number {
        0 => s.push_str("two"),
        1 => s.push_str("three"),
        2 => s.push_str("four"),
        3 => s.push_str("five"),
        4 => s.push_str("six"),
        5 => s.push_str("seven"),
        6 => s.push_str("eight"),
        7 => s.push_str("nine"),
        8 => s.push_str("ten"),
        9 => s.push_str("jack"),
        10 => s.push_str("queen"),
        11 => s.push_str("king"),
        12 => s.push_str("ace"),
        _ => panic!("Error trying to add card number to url!"),
    }
    s.push_str("of");
    match suit {
        0 => s.push_str("hearts"),
        1 => s.push_str("clubs"),
        2 => s.push_str("diamonds"),
        3 => s.push_str("spades"),
        _ => panic!("Error trying to add suit to url!"),
    }

    s
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
                        suit: 0,
                        number: j,
                        link: format!(
                            "http://willsloan.com/cards/{}of{}",
                            match j {
                                _ => "aaa",
                            },
                            "diamonds",
                        ),
                    }),
                    1 => deck.push(Card {
                        suit: 1,
                        number: j,
                        link: format!(
                            "http://willsloan.com/cards/{}of{}",
                            match j {
                                _ => "aaa",
                            },
                            "diamonds",
                        ),
                    }),
                    2 => deck.push(Card {
                        suit: 2,
                        number: j,
                        link: format!(
                            "http://willsloan.com/cards/{}of{}",
                            match j {
                                _ => "aaa",
                            },
                            "diamonds",
                        ),
                    }),
                    3 => deck.push(Card {
                        suit: 3,
                        number: j,
                        link: format!(
                            "http://willsloan.com/cards/{}of{}",
                            match j {
                                _ => "aaa",
                            },
                            "diamonds",
                        ),
                    }),
                    _ => deck.push(Card {
                        suit: 4,
                        number: 69,
                    }),
                }
            }
        }

        //shuffle_deck(&mut deck);
        Game {
            deck,
            players,
            pool: 0,
            flop: Vec::new(),
            winner: String::from(""),
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
            winner: String::from(""),
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
        }
    }

    pub fn check_cards(&mut self) -> String {
        let mut points: HashMap<Player, u8> = HashMap::new();
        for index in 0..self.players.len() {
            let cardone = self.players[index].cards.first().unwrap().number;
            let cardtwo = self.players[index].cards.last().unwrap().number;
            if cardone > cardtwo {
                self.players[index].highcard = cardone;
            } else {
                self.players[index].highcard = cardtwo;
            }
            let flush = check_flush(&self.players[index].cards, &self.flop.clone());
            let straight = check_straight(&self.players[index].cards, &self.flop.clone());
            let pairs = check_pairs(&self.players[index].cards, &self.flop.clone());

            let value_of_hand = [flush, straight, pairs, cardone, cardtwo]
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
    let straight = check_straight(hand, flop) == 16;
    let mut cards: Vec<u8> = [hand.first().unwrap().suit, hand.last().unwrap().suit].to_vec();
    for i in flop {
        cards.push(i.suit);
    }
    let mut count_of_each_card: HashMap<u8, u8> = HashMap::new();

    for card in cards {
        let count = count_of_each_card.entry(card).or_insert(0);
        *count += 1;
    }

    let mut vals = count_of_each_card.values().collect::<Vec<&u8>>();
    vals.sort_by(|a, b| b.cmp(a));

    if vals.is_empty() {
        0
    } else {
        match *vals[0] {
            x if x >= 5 && straight => {
                println!("{:?}", vals);
                21
            }
            x if x >= 5 => {
                println!("{:?}", vals);
                17
            }
            _ => 0,
        }
    }
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
        s.serialize_field("winner", &self.winner)?;
        //s.serialize_field("num", &self.num)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Game {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Players,
            Deck,
            Pool,
            Flop,
            Winner,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`players` or `deck` or `pool` or `flop` or `winner`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "players" => Ok(Field::Players),
                            "deck" => Ok(Field::Deck),
                            "pool" => Ok(Field::Pool),
                            "flop" => Ok(Field::Flop),
                            "winner" => Ok(Field::Winner),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Game;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Game, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut players = None;
                let mut deck = None;
                let mut pool = None;
                let mut flop = None;
                let mut winner = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Players => {
                            if players.is_some() {
                                return Err(de::Error::duplicate_field("players"));
                            }
                            players = Some(map.next_value()?);
                        }
                        Field::Deck => {
                            if deck.is_some() {
                                return Err(de::Error::duplicate_field("deck"));
                            }
                            deck = Some(map.next_value()?);
                        }
                        Field::Pool => {
                            if pool.is_some() {
                                return Err(de::Error::duplicate_field("pool"));
                            }
                            pool = Some(map.next_value()?);
                        }
                        Field::Flop => {
                            if flop.is_some() {
                                return Err(de::Error::duplicate_field("flop"));
                            }
                            flop = Some(map.next_value()?);
                        }
                        Field::Winner => {
                            if winner.is_some() {
                                return Err(de::Error::duplicate_field("winner"));
                            }
                            winner = Some(map.next_value()?);
                        }
                    }
                }
                let players = players.ok_or_else(|| de::Error::missing_field("players"))?;
                let deck = deck.ok_or_else(|| de::Error::missing_field("deck"))?;
                let pool = pool.ok_or_else(|| de::Error::missing_field("pool"))?;
                let flop = flop.ok_or_else(|| de::Error::missing_field("flop"))?;
                let winner = winner.ok_or_else(|| de::Error::missing_field("winner"))?;
                Ok(Game {
                    players,
                    deck,
                    pool,
                    flop,
                    winner,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["players", "deck", "pool", "flop", "winner"];
        deserializer.deserialize_struct("Game", FIELDS, DurationVisitor)
    }
}

/*



The Start the area for Card




*/

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct Card {
    pub suit: u8,
    pub number: u8,
    pub link: String,
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Card", 3)?;
        s.serialize_field("number", &self.number)?;
        s.serialize_field("suit", &self.suit)?;
        s.serialize_field("link", &self.link)?;
        //s.serialize_field("num", &self.num)?;
        s.end()
    }
}
impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Suit,
            Number,
            Link,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`suit` or `number` or `link`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "suit" => Ok(Field::Suit),
                            "number" => Ok(Field::Number),
                            "link" => Ok(Field::Link),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Card;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Card, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut suit = None;
                let mut number = None;
                let mut link = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Suit => {
                            if suit.is_some() {
                                return Err(de::Error::duplicate_field("suit"));
                            }
                            suit = Some(map.next_value()?);
                        }
                        Field::Number => {
                            if number.is_some() {
                                return Err(de::Error::duplicate_field("number"));
                            }
                            number = Some(map.next_value()?);
                        }
                        Field::Link => {
                            if link.is_some() {
                                return Err(de::Error::duplicate_field("link"));
                            }
                            link = Some(map.next_value()?);
                        }
                    }
                }
                let suit = suit.ok_or_else(|| de::Error::missing_field("suit"))?;
                let number = number.ok_or_else(|| de::Error::missing_field("number"))?;
                let link = link.ok_or_else(|| de::Error::missing_field("link"))?;
                Ok(Card { suit, number, link })
            }
        }

        const FIELDS: &'static [&'static str] = &["suit", "number", "link"];
        deserializer.deserialize_struct("Card", FIELDS, DurationVisitor)
    }
}
/*



The Start the area for Player




*/

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct Player {
    pub cards: Vec<Card>,
    pub chips: u32,
    pub ip: String,
    pub folded: bool,
    pub hand: u8, // value of players hand
    pub highcard: u8,
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Player", 6)?;
        s.serialize_field("cards", &self.cards)?;
        s.serialize_field("chips", &self.chips)?;
        s.serialize_field("ip", &self.ip)?;
        s.serialize_field("folded", &self.folded)?;
        s.serialize_field("hand", &self.hand)?;
        s.serialize_field("highcard", &self.highcard)?;
        //s.serialize_field("num", &self.num)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Player {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Cards,
            Chips,
            Ip,
            Folded,
            Hand,
            Highcard,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(
                            "`cards` or `chips` or `ip` or `folded` or `hand` or `highcard`",
                        )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "cards" => Ok(Field::Cards),
                            "chips" => Ok(Field::Chips),
                            "ip" => Ok(Field::Ip),
                            "folded" => Ok(Field::Folded),
                            "hand" => Ok(Field::Hand),
                            "highcard" => Ok(Field::Highcard),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Player;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Player, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut cards = None;
                let mut chips = None;
                let mut ip = None;
                let mut folded = None;
                let mut hand = None;
                let mut highcard = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Cards => {
                            if cards.is_some() {
                                return Err(de::Error::duplicate_field("cards"));
                            }
                            cards = Some(map.next_value()?);
                        }
                        Field::Chips => {
                            if chips.is_some() {
                                return Err(de::Error::duplicate_field("chips"));
                            }
                            chips = Some(map.next_value()?);
                        }
                        Field::Ip => {
                            if ip.is_some() {
                                return Err(de::Error::duplicate_field("ip"));
                            }
                            ip = Some(map.next_value()?);
                        }
                        Field::Folded => {
                            if folded.is_some() {
                                return Err(de::Error::duplicate_field("folded"));
                            }
                            folded = Some(map.next_value()?);
                        }
                        Field::Hand => {
                            if hand.is_some() {
                                return Err(de::Error::duplicate_field("hand"));
                            }
                            hand = Some(map.next_value()?);
                        }
                        Field::Highcard => {
                            if highcard.is_some() {
                                return Err(de::Error::duplicate_field("highcard"));
                            }
                            highcard = Some(map.next_value()?);
                        }
                    }
                }
                let cards = cards.ok_or_else(|| de::Error::missing_field("cards"))?;
                let chips = chips.ok_or_else(|| de::Error::missing_field("chips"))?;
                let ip = ip.ok_or_else(|| de::Error::missing_field("ip"))?;
                let folded = folded.ok_or_else(|| de::Error::missing_field("folded"))?;
                let hand = hand.ok_or_else(|| de::Error::missing_field("hand"))?;
                let highcard = highcard.ok_or_else(|| de::Error::missing_field("highcard"))?;
                Ok(Player {
                    cards,
                    chips,
                    ip,
                    folded,
                    hand,
                    highcard,
                })
            }
        }

        const FIELDS: &'static [&'static str] =
            &["cards", "chips", "ip", "folded", "hand", "highcard"];
        deserializer.deserialize_struct("Player", FIELDS, DurationVisitor)
    }
}
