#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use serde::{
    de::{DeserializeOwned, Error, MapAccess, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, str::FromStr};

trait VisitorHelper<'de>: Visitor<'de> {
    fn get_any<T, D>(&self, key: &str, value: serde_json::Value) -> Result<T, D::Error>
    where
        D: MapAccess<'de>,
        T: DeserializeOwned,
    {
        serde_json::from_value::<T>(value.clone()).map_err(|_| {
            D::Error::invalid_type(
                Unexpected::Other(format!("value for {}: {:?}", key, value).as_ref()),
                self,
            )
        })
    }

    fn get_from_str<T, D>(&self, key: &str, value: serde_json::Value) -> Result<Option<T>, D::Error>
    where
        D: MapAccess<'de>,
        T: DeserializeOwned + FromStr,
    {
        match serde_json::from_value::<Option<T>>(value.clone()) {
            Ok(val) => Ok(val),
            _ => match serde_json::from_value::<String>(value.clone()) {
                Ok(val) => val
                    .parse::<T>()
                    .map(|v| Some(v))
                    .map_err(|_| D::Error::invalid_type(Unexpected::Str(val.as_ref()), self)),
                _ => Err(D::Error::invalid_value(
                    Unexpected::Other(format!("value for {}: {:?}", key, value).as_ref()),
                    self,
                )),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct Galaxy {
    name: Option<String>,
    total_stars: Option<i32>,
}

impl<'de> Deserialize<'de> for Galaxy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GalaxyVisitor;

        impl<'de> VisitorHelper<'de> for GalaxyVisitor {}

        impl<'de> Visitor<'de> for GalaxyVisitor {
            type Value = Galaxy;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("valid Galaxy")
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
            where
                D: MapAccess<'de>,
            {
                let mut galaxy = Galaxy {
                    name: None,
                    total_stars: None,
                };

                while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
                    match &key[..] {
                        "name" => {
                            galaxy.name =
                                VisitorHelper::get_any::<Option<String>, D>(&self, "name", value)?;
                        }
                        "total_stars" => {
                            galaxy.total_stars =
                                VisitorHelper::get_from_str::<i32, D>(&self, "total_stars", value)?;
                        }
                        _ => {}
                    }
                }
                Ok(galaxy)
            }
        }

        deserializer.deserialize_struct("Galaxy", &["name", "total_stars"], GalaxyVisitor)
    }
}

fn main() {
    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        serde_json::from_value::<Galaxy>(json!({ "total_stars": "3131231", "name": "Backyard" })),
        serde_json::from_value::<Galaxy>(json!({ "total_stars": 989031213, "name": "Rooftop" })),
        serde_json::from_value::<Galaxy>(json!({ "name": "Unknown" })),
        serde_json::from_value::<Galaxy>(json!({ "total_stars": 0 })),
        serde_json::from_value::<Galaxy>(json!({ "total_stars": "aa" })),
        serde_json::from_value::<Galaxy>(json!({ "name": 0 })),
    )
}
