extern crate serde_yaml;
extern crate indexmap;

use std::{collections::HashMap};
use std::fs;
use serde::{Deserialize, Serialize};
use std::io::Write;

mod serialization_skippers;
mod acls;
mod meter;
mod router;
mod dps_interface;
mod vlan_route;



//Example of implementing you own serializer
// impl Serialize for Router {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//             S: serde::Serializer {
//         let mut output = String::new();
//         output += "[";
//         output += &self.vlans.join(", ");
//         output += "]";    
//         let mut state = serializer.serialize_struct("Router",1)?;
//         state.serialize_field("vlans", &output);
//         state.end()

//     }
// }



#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FaucetDocument {
    #[serde(skip_serializing_if = "serialization_skippers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    acls: HashMap<String,acls::ACL>,
    #[serde(skip_serializing_if = "serialization_skippers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    dps: HashMap<String,dps_interface::DP>,
    #[serde(skip_serializing_if = "serialization_skippers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    meters: HashMap<String,meter::Meter>,
    #[serde(skip_serializing_if = "serialization_skippers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    routers: HashMap<String,router::Router>,
    #[serde(skip_serializing_if = "serialization_skippers::skip_serializing_if_empty_int")]
    #[serde(default)]
    version: u32,
    #[serde(skip_serializing_if = "serialization_skippers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    include: Vec<String>,
    #[serde(skip_serializing_if = "serialization_skippers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    vlans: HashMap<String,vlan_route::VLAN>,
}

fn main() {
    let s=read_file(String::from("./examples/ex_2"));
    let deserialized_yaml: FaucetDocument = serde_yaml::from_str(&s).unwrap();
    println!("{:?}",deserialized_yaml);

    //assert_eq!(deserialized_yaml.include[0],"acls.yaml");
    //assert_eq!(deserialized_yaml.vlans.get("office").unwrap().routes[0].route.get("ip_dst").unwrap(),"192.168.0.0/24");

    let dones = serde_yaml::to_string(&deserialized_yaml).unwrap();

   let mut file = std::fs::File::create("data.yaml").expect("create failed");
   file.write_all(dones.as_bytes()).expect("write failed");
   println!("data written to file" );


}

fn read_file(file_path: String) -> String {   
    let read_file_result = fs::read_to_string(file_path);
    match read_file_result {
        Ok(x) => {
            return x;
        }
        Err(e) => {
            eprintln!("Could not read file. The error was \"{}\". Exiting.",e);
            std::process::exit(1);
        }
    }
}