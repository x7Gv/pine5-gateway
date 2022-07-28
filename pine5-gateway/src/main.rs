mod web_api;
mod service;
mod plugins;
mod model;

use hyper::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let addr = "127.0.0.1:1337".parse().unwrap();
    let server = Server::bind(&addr).serve(web_api::MakeSvc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
