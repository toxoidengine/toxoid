use serde::{Deserialize, Serialize};
pub use flexbuffers;
use core::result::Result;
// Assuming `flexbuffers` and `NetworkMessages` are compatible with `no_std`
// You might need a custom error type since `String` and other std types are not available
#[derive(Debug)]
pub enum DeserializeError {
    FlexbufferError,
    // Other error types as needed
}

impl From<flexbuffers::SerializationError> for DeserializeError {
    fn from(_: flexbuffers::SerializationError) -> Self {
        DeserializeError::FlexbufferError
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkMessageComponent {
    pub name: String,
    pub object: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkMessageEntity {
    pub id: u64,
    pub event: String,
    pub components: Vec<NetworkMessageComponent>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkMessages {
    pub messages: Vec<NetworkMessageEntity>
}

pub fn serialize(network_messages: NetworkMessages) -> Result<Vec<u8>, DeserializeError> {
    let mut s = flexbuffers::FlexbufferSerializer::new();
    network_messages.serialize(&mut s)?;
    Ok(s.view().to_vec())
}


pub fn deserialize(data: &[u8]) -> Result<NetworkMessages, DeserializeError> {
    let d = flexbuffers::Reader::get_root(data).map_err(|_| DeserializeError::FlexbufferError)?;
    NetworkMessages::deserialize(d).map_err(|_| DeserializeError::FlexbufferError)
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

#[no_mangle]
pub fn toxoid_network_recv(network_messages: NetworkMessages) {
}
