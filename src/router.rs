use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize,  Deserialize)]
pub struct Router {
    vlans : Vec<String>,
    bgp : RouterBGP,
}

#[derive(Debug, PartialEq, Serialize,  Deserialize)]
struct RouterBGP {
    #[serde(rename="as")]
    router_bgp_as : u32,
    connect_mode : String,
    neighbor_addresses : Vec<String>,
    neighbor_as : u32,
    routerid : String,
    server_addresses : Vec<String>,
    port : u32,
    vlan : String,
}



