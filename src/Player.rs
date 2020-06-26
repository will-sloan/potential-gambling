// use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
// use serde::ser::{Serialize, SerializeStruct, Serializer};
// use std::fmt;
// #[path = "Card.rs"]
// mod Card;
// #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
// pub struct Player {
//     pub cards: Vec<Card::Card>,
//     pub chips: u32,
//     pub ip: String,
//     pub folded: bool,
//     pub hand: u8, // value of players hand
// }

// impl Serialize for Player {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_struct("Player", 5)?;
//         s.serialize_field("cards", &self.cards)?;
//         s.serialize_field("chips", &self.chips)?;
//         s.serialize_field("ip", &self.ip)?;
//         s.serialize_field("folded", &self.folded)?;
//         s.serialize_field("hand", &self.hand)?;
//         //s.serialize_field("num", &self.num)?;
//         s.end()
//     }
// }
