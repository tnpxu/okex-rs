#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    #[serde(with = "string_or_float")] pub price: f64,
    #[serde(with = "string_or_float")] pub qty: f64,

    // Never serialized.
    #[serde(skip_serializing)]
    ignore: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    #[serde(with = "string_or_float")] pub price: f64,
    #[serde(with = "string_or_float")] pub qty: f64,

    // Never serialized.
    #[serde(skip_serializing)]
    ignore: Vec<String>,
}

mod string_or_float {
    use std::fmt;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where T: fmt::Display,
              S: Serializer
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }
        
        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}