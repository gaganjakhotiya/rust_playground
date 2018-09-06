#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::fmt;
use serde::{
    de::{Error, MapAccess, Visitor, Unexpected},
    Deserialize, Deserializer,
};

#[derive(Clone, Debug, Serialize)]
struct A {
    a: Option<i32>,
    b: Option<String>,
}


impl<'de> Deserialize<'de> for A {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct AVisitor;

        impl<'de> Visitor<'de> for AVisitor {
            type Value = A;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("valid A")
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
                where
                    D: MapAccess<'de>,
            {
                let mut a = A { a: None, b: None };
                while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
                    match &key[..] {
                        "a" => {
                            a.a = match serde_json::from_value::<i32>(value.clone()) {
                                Ok(val) => Some(val),
                                _ => match serde_json::from_value::<String>(value) {
                                    Ok(val) => val.parse::<i32>().ok(),
                                    _ => None
                                }
                            };
                        },
                        "b" => {
                            a.b = serde_json::from_value::<String>(value).ok();
                        }
                        _ => {}
                    }
                }
                Ok(a)
                // Err(D::Error::invalid_value(Unexpected::Map, &self))
            }
        }

        deserializer.deserialize_struct("A", &["a", "b"], AVisitor)
    }
}


fn main() {
    println!(
        "{:?}", serde_json::from_value::<A>(json!({ "a": "12", "b": "abc" }))
    )
}
