pub use flexbuffers;
use serde::{Deserialize, Serialize};

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
    pub data: Vec<u8>,
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
