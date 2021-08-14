use serde::{Deserialize, Serialize, de::DeserializeSeed, ser::SerializeStruct};
use std::{collections::HashMap, convert::TryInto};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Meter{
    flags: Vec<String>,
    bands:Vec<HashMap<String,MeterBand>>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MeterBand{
    #[serde(rename = "type")]
    meter_band_type: String,
    rate: u32,
    burst_size: u32,
    prec_level: u32,
}

