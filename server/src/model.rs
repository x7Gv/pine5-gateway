#[derive(Debug, Clone)]
pub struct RawPayload {
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct JsonPayload {
    pub json: String,
}
