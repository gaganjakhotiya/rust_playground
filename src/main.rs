#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
//#[macro_use]
extern crate diesel;

use serde::{
    de::{Error, MapAccess, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

//#[macro_use]
//pub mod macros {
//    macro_rules! deserialized_struct {
//        ($name: ident, $($field: ident: $ftype: ty where $parser: ident: $default: expr;)+) => {
//            #[derive(Queryable, Clone, Debug, Serialize)]
//            pub struct $name {
//                pub id: i32,
//                $(
//                    __ds_field!($field: $ftype where $parser)
//                )+
//            }
//        };
//    }
//
//    macro_rules! __ds_field {
//        ($field: ident: $ftype: ty where default) => {
//            pub $field: $ftype,
//        };
//        ($field: ident: $ftype: ty where from_str) => {
//            pub $field: Option<$ftype>,
//        };
//    }
//}

#[derive(Clone, Debug, Serialize, PartialEq)]
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
                            galaxy.name = serde_json::from_value::<Option<String>>(value.clone())
                                .map_err(|_| {
                                    D::Error::invalid_type(
                                        Unexpected::Other(
                                            format!("value for {}: {:?}", "name", value).as_ref(),
                                        ),
                                        &self,
                                    )
                                })?;
                        }
                        "total_stars" => {
                            galaxy.total_stars = match serde_json::from_value::<i32>(value.clone())
                            {
                                Ok(val) => Ok(val),
                                _ => match serde_json::from_value::<String>(value.clone()) {
                                    Ok(val) => val.parse::<i32>().map_err(|_| {
                                        D::Error::invalid_type(Unexpected::Str(val.as_ref()), &self)
                                    }),
                                    _ => Err(D::Error::invalid_value(
                                        Unexpected::Other(
                                            format!("value for {}: {:?}", "pincode", value)
                                                .as_ref(),
                                        ),
                                        &self,
                                    )),
                                },
                            }.ok();
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

#[cfg(test)]
mod tests {
    use Galaxy;
    use serde_json;

    #[test]
    fn galaxy_with_two_stars() {
        let galaxy = serde_json::from_value::<Galaxy>(json!({ "total_stars": 2 }));
        let stars = if let Ok(g) = galaxy {
            g.total_stars
        } else {
            None
        };
        assert_eq!(stars, Some(2));
    }

    #[test]
    fn invalid_galaxy() {
        assert_eq!(serde_json::from_value::<Galaxy>(json!({ "name": "Andromeda" })).ok(), None);
    }
}