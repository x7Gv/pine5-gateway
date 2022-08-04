wit_bindgen_rust::export!("../../interfaces/decoder_plugin.wit");

use serde::{Deserialize, Serialize};

const ID: &'static str = "rust_test_decoder";

struct DecoderPlugin;

#[derive(Serialize)]
struct JsonPacket {
    data: Vec<u8>,
}

impl decoder_plugin::DecoderPlugin for DecoderPlugin {
    fn identifier() -> String {
        ID.to_string()
    }

    fn name() -> String {
        "test_decoder_rust".to_string()
    }

    fn on_plugin_load() -> () {
        println!("test_decoder_rust loaded");
    }

    fn on_plugin_unload() -> () {
        println!("test_decoder_rust unloaded");
    }

    fn decode(
        source: decoder_plugin::Base64Packet,
    ) -> Result<decoder_plugin::JsonPacket, decoder_plugin::DecodingError> {
        let p = JsonPacket { data: source.data };

        let json =
            serde_json::to_string(&p).map_err(|_| decoder_plugin::DecodingError::InvalidPacket)?;

        let p = decoder_plugin::JsonPacket { json };

        println!("{:?}", p);

        Ok(p)
    }
}
