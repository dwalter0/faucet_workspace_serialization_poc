use serde::{Deserialize, Serialize, de::DeserializeSeed, ser::SerializeStruct};

#[derive(Debug, PartialEq, Serialize,  Deserialize)]
pub struct Router {
    vlans : Vec<String>
}



