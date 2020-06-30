use rand::seq::SliceRandom;
use rand::thread_rng;
//use std::collections::HashMap;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;
extern crate web_sys;
mod arrays;

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    //pub num: u8,
    pub pool: u32,
    pub flop: Vec<Card>,
    pub winner: Player,
}

pub fn shuffle_deck(deck: &mut Vec<Card>) {
    deck.shuffle(&mut thread_rng());
}

fn create_url(num: &u32) -> String {
    let mut s = "http://willsloan.com/cards/".to_string();
    let a = num.clone();
    match a {
        x if (x >> 16) & 0x0001 != 0 => s.push_str("two"), //1 -> 2
        x if (x >> 16) & 0x0002 != 0 => s.push_str("three"), //2 -> 3
        x if (x >> 16) & 0x0004 != 0 => s.push_str("four"), //4 -> 4
        x if (x >> 16) & 0x0008 != 0 => s.push_str("five"), //8 -> 5
        x if (x >> 16) & 0x0010 != 0 => s.push_str("six"), //16 -> 6
        x if (x >> 16) & 0x0020 != 0 => s.push_str("seven"), //32 -> 7
        x if (x >> 16) & 0x0040 != 0 => s.push_str("eight"), //64 -> 8
        x if (x >> 16) & 0x0080 != 0 => s.push_str("nine"), // 128 -> 9
        x if (x >> 16) & 0x0100 != 0 => s.push_str("ten"), // 256 -> 10
        x if (x >> 16) & 0x0200 != 0 => s.push_str("jack"), // 512 -> J
        x if (x >> 16) & 0x0400 != 0 => s.push_str("queen"), // 1024 -> Q
        x if (x >> 16) & 0x0800 != 0 => s.push_str("king"), // 2048 -> K
        x if (x >> 16) & 0x1000 != 0 => s.push_str("ace"), // 4096 -> A
        _ => panic!("Error trying to add card number to url!"),
    }
    s.push_str("of");
    let b = num.clone();
    match b {
        x if (x & 0x8000) != 0 => s.push_str("clubs"),
        x if (x & 0x4000) != 0 => s.push_str("diamonds"),
        x if (x & 0x2000) != 0 => s.push_str("hearts"),
        x if (x & 0x1000) != 0 => s.push_str("spades"),
        _ => panic!("Error trying to add suit to url!"),
    }
    s.push_str(".png");
    s
}
fn findit(key: u32) -> u32 {
    // let mut low: usize = 0;
    // let mut high: usize = 4887;
    // let mut mid: usize;
    // CODE BELOW WORKED AT SOME POINT!!!
    // let spot = arrays::products.binary_search(&key).unwrap();
    // spot as u32
    // CODE BELOW MIGHT NOT WORK!!!
    arrays::products.binary_search(&key).unwrap() as u32
}

fn eval_5cards(c1: u32, c2: u32, c3: u32, c4: u32, c5: u32) -> u32 {
    // not my comments!!! These are the comments that were in Cactus Kev's c program

    // High the value, the lower the hand value!
    let mut q: u32;
    let s: u32;

    q = (c1 | c2 | c3 | c4 | c5) >> 16;

    // check for Flushes and StraightFlushes
    if c1 & c2 & c3 & c4 & c5 & 0xF000 != 0 {
        return arrays::flushes[q as usize];
    }
    // check for Straights and HighCard hands
    s = arrays::unique5[q as usize];
    if s != 0 {
        return s;
    };

    // let's do it the hard way
    q = (c1 & 0xFF) * (c2 & 0xFF) * (c3 & 0xFF) * (c4 & 0xFF) * (c5 & 0xFF);
    q = findit(q);

    return arrays::values[q as usize];
}

impl Game {
    // used only at the start of the game
    pub fn new_game() -> Game {
        //, num_players: u8
        let mut deck: [u32; 52] = [0; 52];
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        let mut n: u32 = 0;
        let mut suit: u32 = 0x8000;
        while i < 4 {
            //println!();
            //println!("I is: {}", i);
            while j < 13 {
                //println!("J is: {}", j);
                deck[n as usize] =
                    (arrays::primes[j as usize] | ((j as u32) << 8) | suit | (1 << (16 + j)))
                        .into();
                j += 1;
                n += 1;
            }
            j = 0;
            i += 1;
            suit >>= 1;
        }
        //println!("Done in init_deck");
        let mut d: Vec<Card> = Vec::new();
        for a in deck.iter() {
            d.push(Card {
                card: a.clone(),
                link: create_url(&a),
            })
        }
        //shuffle_deck(&mut deck);
        Game {
            deck: d,
            players: Vec::new(),
            pool: 0,
            flop: Vec::new(),
            winner: Player {
                cards: Vec::new(),
                chips: 0,
                ip: String::from("Not A Player"),
                folded: true,
                handvalue: 9999,
            },
        }
    }
    /*
        pub cards: Vec<Card>,
        pub chips: u32,
        pub ip: String,
        pub folded: bool,
        pub handvalue: u32,
    */
    pub fn deal_to_players(&mut self) {
        // goes puts all cards from players into deck
        // Shuffles the cards
        // then redeals
        for index in 0..self.players.len() {
            //println!("Index of players");
            match self.players[index].cards.pop() {
                Some(card) => self.deck.push(card),
                None => {
                    //println!("No cards in player");
                    continue;
                }
            }
            match self.players[index].cards.pop() {
                Some(card) => self.deck.push(card),
                None => {
                    //println!("No cards in player");
                    continue;
                }
            }
        }

        shuffle_deck(&mut self.deck);

        for index in 0..self.players.len() {
            //println!("Index of deck");
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
    /*

    short
    eval_7hand(int *hand)
    {
        int i, j, q, best = 9999, subhand[5];

        for (i = 0; i < 21; i++)
        {
            for (j = 0; j < 5; j++)
                subhand[j] = hand[ perm7[i][j] ];
            q = eval_5hand(subhand);
            if (q < best)
                best = q;
        }
        return best;
    }

    */
    pub fn check_cards(&mut self) {
        // both the player cards and the flop cards
        let mut winning_player = Player {
            cards: Vec::new(),
            chips: 0,
            ip: String::from("Not A Player"),
            folded: true,
            handvalue: 9999,
        };
        let mut current_best = 9999;
        let mut p_and_f: Vec<Card> = Vec::new();
        //web_sys::console::log_1(&"after winning_player".into());
        for index in 0..self.players.len() {
            p_and_f = self.players[index].cards.clone();
            p_and_f.append(&mut self.flop.clone());
            let mut personal_best = 9999; // high is worse!
            let mut sub_hand = vec![0, 0, 0, 0, 0];
            //web_sys::console::log_1(&format!("after creating hand: {:#?}", &p_and_f).into());
            for x in 0..21 {
                //web_sys::console::log_1(&format!("X: {}", &x).into());
                for y in 0..5 {
                    //web_sys::console::log_1(&format!("Y: {}", &y).into());
                    sub_hand[y] = p_and_f[arrays::perm7[x][y] as usize].card;
                }
                //web_sys::console::log_1(&format!("Subhand: {:#?}", &sub_hand).into());
                let v = eval_5cards(
                    sub_hand[0],
                    sub_hand[1],
                    sub_hand[2],
                    sub_hand[3],
                    sub_hand[4],
                );
                //web_sys::console::log_1(&format!("V: {}", &v).into());
                if v < personal_best {
                    personal_best = v;
                    //web_sys::console::log_1(&format!("New best: {}", &personal_best).into());
                }
            }
            /*
            web_sys::console::log_1(&format!("in for loop: {}", index).into());
            p_and_f = self.players[index].cards.clone();
            p_and_f.append(&mut self.flop);
            web_sys::console::log_1(&format!("after creating one hand: {:#?}", p_and_f).into());
            let v = eval_5cards(
                p_and_f[0].card,
                p_and_f[1].card,
                p_and_f[2].card,
                p_and_f[3].card,
                p_and_f[4].card,
            );
            */
            self.players[index].handvalue = personal_best;
            p_and_f.clear();
            if personal_best < current_best {
                winning_player = self.players[index].clone();
                current_best = personal_best;
            }
        }
        self.winner = winning_player.clone();
    }

    pub fn do_flop(&mut self) {
        for _ in 0..3 {
            self.flop.push(self.deck.pop().unwrap());
        }
    }

    pub fn flip_one(&mut self) {
        self.flop.push(self.deck.pop().unwrap());
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
    pub card: u32,
    pub link: String,
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Card", 2)?;
        s.serialize_field("card", &self.card)?;
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
            Card,
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
                        formatter.write_str("`card` or `link`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "card" => Ok(Field::Card),
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
                let mut card = None;
                let mut link = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Card => {
                            if card.is_some() {
                                return Err(de::Error::duplicate_field("card"));
                            }
                            card = Some(map.next_value()?);
                        }
                        Field::Link => {
                            if link.is_some() {
                                return Err(de::Error::duplicate_field("link"));
                            }
                            link = Some(map.next_value()?);
                        }
                    }
                }
                let card = card.ok_or_else(|| de::Error::missing_field("card"))?;
                let link = link.ok_or_else(|| de::Error::missing_field("link"))?;
                Ok(Card { card, link })
            }
        }

        const FIELDS: &'static [&'static str] = &["card", "link"];
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
    pub handvalue: u32,
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
        s.serialize_field("handvalue", &self.handvalue)?;
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
            Handvalue,
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
                        formatter.write_str("`cards` or `chips` or `ip` or `folded` or `handvalue`")
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
                            "handvalue" => Ok(Field::Handvalue),
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
                let mut handvalue = None;
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
                        Field::Handvalue => {
                            if handvalue.is_some() {
                                return Err(de::Error::duplicate_field("handvalue"));
                            }
                            handvalue = Some(map.next_value()?);
                        }
                    }
                }
                let cards = cards.ok_or_else(|| de::Error::missing_field("cards"))?;
                let chips = chips.ok_or_else(|| de::Error::missing_field("chips"))?;
                let ip = ip.ok_or_else(|| de::Error::missing_field("ip"))?;
                let folded = folded.ok_or_else(|| de::Error::missing_field("folded"))?;
                let handvalue = handvalue.ok_or_else(|| de::Error::missing_field("handvalue"))?;
                Ok(Player {
                    cards,
                    chips,
                    ip,
                    folded,
                    handvalue,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["cards", "chips", "ip", "folded", "handvalue"];
        deserializer.deserialize_struct("Player", FIELDS, DurationVisitor)
    }
}
