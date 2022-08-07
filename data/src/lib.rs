//! Define main **datatypes** to be used in the
//! pine5 gateway server

/// Represent dynamically sized,
/// `JSON` Payload.
pub struct JsonPacket {
    pub json: Vec<u8>,
    pub length: usize,
}

/// Represent dynamically sized,
/// `base64` payload.
pub struct Base64Packet {
    pub base64: Vec<u8>,
    pub length: usize,
}
