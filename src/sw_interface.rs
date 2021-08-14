use serde::{Deserialize, Serialize, de::DeserializeSeed, ser::SerializeStruct};
use std::{collections::HashMap, convert::TryInto};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SW {
    dp_id: u32,
    hardware: String,
    interfaces: HashMap<u32,Interface>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Interface {
    name: String,
    #[serde(skip_serializing_if = "crate::serialization_skippers::skip_serializing_if_empty_str")]
    #[serde(default)]
    description: String,
    #[serde(skip_serializing_if = "crate::serialization_skippers::skip_serializing_if_empty_str")]
    #[serde(default)]
    native_vlan: String,
    #[serde(default)]
    acls_in: Vec<String>,
}