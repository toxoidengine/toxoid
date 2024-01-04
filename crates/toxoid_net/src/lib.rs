use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkMessageComponent {
    // name: String,
    // object: Vec<u8>
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkMessageEntity {
    pub id: u64,
    pub event: String,
    pub components: Vec<NetworkMessageComponent>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkMessages {
    pub messages: Vec<NetworkMessageEntity>
}

pub fn serialize(network_messages: NetworkMessages) -> Vec<u8> {
    let mut s = flexbuffers::FlexbufferSerializer::new();
    network_messages.serialize(&mut s).unwrap();
    s.view().to_vec()
}

pub fn deserialize() {

}