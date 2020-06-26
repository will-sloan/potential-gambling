mod utils;

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::prelude::*;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

extern crate js_sys;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
