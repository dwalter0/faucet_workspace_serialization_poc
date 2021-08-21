use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Meter{
    meter_id : u32,
    entry : HashMap<String,MeterEntry>
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MeterEntry{
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

