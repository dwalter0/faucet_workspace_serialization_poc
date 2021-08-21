use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VLAN {
    acl_in : String,
    #[serde(default)]
    acls_in : Vec<String>,
    #[serde(skip_serializing_if = "crate::serialization_skippers::skip_serializing_if_empty_str")]
    #[serde(default)]
    description : String,
    dot1x_assigned : bool,
    #[serde(skip_serializing_if = "crate::serialization_skippers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    faucet_vips : Vec<String>,
    #[serde(skip_serializing_if = "crate::serialization_skippers::skip_serializing_if_empty_str")]
    #[serde(default)]
    faucet_mac : String,
    max_hosts : u32,
    minimum_ip_size_check : bool,
    name : String,
    proactive_arp_limit : u32,
    proactive_nd_limit : u32,
    #[serde(default)]
    routes : Vec<VLANRoute>,
    targeted_gw_resolution : bool,
    unicast_flood : bool,
    vid : u32,
    }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct VLANRoute{
    route: HashMap<String,String>
}
