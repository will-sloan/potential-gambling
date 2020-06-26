// use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
// use serde::ser::{Serialize, SerializeStruct, Serializer};
// use std::fmt;
// #[path = "Suit.rs"]
// mod Suit;
// #[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
// pub struct Card {
//     pub suit: u8,
//     pub number: u8,
// }

// impl Serialize for Card {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_struct("Card", 2)?;
//         s.serialize_field("number", &self.number)?;
//         s.serialize_field("suit", &self.suit)?;
//         //s.serialize_field("num", &self.num)?;
//         s.end()
//     }
// }
