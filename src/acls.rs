use serde::{Deserialize, Serialize, de::DeserializeSeed, ser::SerializeStruct};
use std::{collections::HashMap, convert::TryInto};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ACL {
    allow: bool,
    force_port_vlan: bool,
    cookie: u32,
    meter: String,
    mirror: String,
    //The output field can be a list or a dictionary. Implemented here as a dictionary but this needs to be revisted. Added to backlog
    output: HashMap<String,ACLOutput>
    }


#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ACLOutput {
    set_fields: Vec<HashMap<String,String>>,
    port: String,
    ports: Vec<String>,
    pop_vlans: bool,
    vlan_vid: u32,
    swap_vid: u32,
    //The vlan_vids field is a bunch of bullshit list of integer or a list of objects see doco. added to backlog
    vlan_vids: Vec<u32>,
    failover: HashMap<String,ACLOutputFailover>,
    tunnel: HashMap<String,ACLOutputTunnel>
    }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ACLOutputTunnel{
    #[serde(rename = "type")]
    tunnel_type: String,
    tunnel_id: String,
    dp: String,
    port: String,
    //Needs some analysis on actual type - might not just be string description sounds complex, backlog entry created.
    exit_instructions: Vec<String>,
    maintain_encapsulation: bool,
    bi_directional: bool,
    reverse: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ACLOutputFailover {
    group_id: u32,
    ports: Vec<String>
    }
