use serde::{Deserialize, Serialize, de::DeserializeSeed, ser::SerializeStruct};
use std::{collections::HashMap, convert::TryInto};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VLAN {
    vid : u32,
    #[serde(skip_serializing_if = "crate::serialization_skippers::skip_serializing_if_empty_str")]
    #[serde(default)]
    description : String,
    #[serde(default)]
    acls_in : Vec<String>,
    #[serde(skip_serializing_if = "crate::serialization_skippers::skip_serializing_if_empty_str")]
    #[serde(default)]
    faucet_mac : String,
    #[serde(skip_serializing_if = "crate::serialization_skippers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    faucet_vips : Vec<String>,
    #[serde(default)]
    routes : Vec<Route>
    }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Route{
    route: HashMap<String,String>
}
