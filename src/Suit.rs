// use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
// use serde::ser::{Serialize, SerializeStruct, Serializer};
// use std::fmt;

// #[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
// pub enum Suit {
//     Heart = 0,
//     Club = 1,
//     Diamonds = 2,
//     Spades = 3,
//     Error = 4,
// }
// pub fn suit_as_val(s: &Suit) -> &u8 {
//     match s {
//         Suit::Heart => &0,
//         Suit::Club => &1,
//         Suit::Diamonds => &2,
//         Suit::Spades => &3,
//         Suit::Error => &4,
//     }
// }
