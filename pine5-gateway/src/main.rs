mod model;
mod plugin;
mod plugins;
mod service;
mod web_api;

use hyper::Server;

use crate::plugin::decoder::decoder_plugin::Base64Packet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let addr = "127.0.0.1:1337".parse().unwrap();
    let server = Server::bind(&addr).serve(web_api::MakeSvc);

    let mut dpm = plugin::decoder::DecoderPluginManager::new();

    dpm.load_path("aabb")?;

    dpm.decode(Base64Packet {data: b"abc"})?;

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
