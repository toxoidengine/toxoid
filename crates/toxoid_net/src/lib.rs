use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkMessageComponent {
    // name: String,
    // object: Vec<u8>
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkMessageEntity {
    pub id: u64,
    pub event: String,
    pub components: Vec<NetworkMessageComponent>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkMessages {
    pub messages: Vec<NetworkMessageEntity>
}

pub fn serialize(network_messages: NetworkMessages) -> Vec<u8> {
    let mut s = flexbuffers::FlexbufferSerializer::new();
    network_messages.serialize(&mut s).unwrap();
    s.view().to_vec()
}

pub fn deserialize(data: &[u8]) -> NetworkMessages {
    let d = flexbuffers::Reader::get_root(data).unwrap();
    let network_messages: NetworkMessages = NetworkMessages::deserialize(d).unwrap();
    network_messages
}

#[no_mangle]
pub fn toxoid_network_send(network_messages: NetworkMessages) {
    // let mut network_messages = NetworkMessages {
    //     messages: Vec::new()
    // };
    // let network_message = NetworkMessageEntity {
    //     id: 1,
    //     event: "PlayerMove".to_string(),
    //     components: vec![
    //         NetworkMessageComponent {
    //             x: 1,
    //             y: 2
    //         }
    //     ]
    // };
    // network_messages.messages.push(network_message);
    // let data = serialize(network_messages);
    // println!("data: {:?}", data);
    // let network_messages = deserialize(&data);
    // println!("network_messages: {:?}", network_messages);
}
