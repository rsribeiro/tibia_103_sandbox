use anyhow::Result;
use config::CONFIG;
use std::path::Path;
use tokio::net::TcpListener;
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use world::World;

mod chat;
mod config;
mod constants;
mod io;
mod map;
mod network;
mod player;
mod world;

#[tokio::main]
async fn main() -> Result<()> {
    /********************************************************************************
     * 
     * Read config and initialize map.
     * 
     ********************************************************************************/
    config::init(Path::new("server.toml"))?;
    map::init(&CONFIG.get().unwrap().world.map_file)?;
    let config = CONFIG.get().unwrap();


    /********************************************************************************
     * 
     * Bind TCP listener (start the server).
     * 
     ********************************************************************************/
    let addr = std::net::SocketAddrV4::new(config.server.ip, config.server.port);
    let listener = TcpListener::bind(addr).await?;
    println!("[{}] Tibia game server is running on: {addr}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));


    /********************************************************************************
     * 
     * Initialize the game world.
     * 
     ********************************************************************************/
    let world = World::new();
    World::start(&world);
    let world_sender = world.read().await.sender();


    /********************************************************************************
     * 
     * Accept incoming connections.
     * 
     ********************************************************************************/
    let mut incoming = TcpListenerStream::new(listener);

    while let Some(stream) = incoming.next().await {
        match stream {
            Ok(stream) => {
                let sender = world_sender.clone();

                /********************************************************************************
                 * 
                 * Spawn a task to handle this connection so the listener stays responsive
                 * 
                 ********************************************************************************/
                tokio::spawn(async move {

                    /********************************************************************************
                     * 
                     * Perform login handshake
                     * 
                     ********************************************************************************/
                    match network::connection::Connection::handle_login(stream, sender).await {

                        /********************************************************************************
                         * 
                         * Authenticated connection
                         * 
                         ********************************************************************************/
                        Ok(Some(mut conn)) => {
                            debug_log!("::main -> Login was successfully authenticated during handshake.");

                            match conn.run().await {
                                Ok(_) => debug_log!("::main -> Connection closed normally."),
                                Err(e) => debug_log!("::main -> Connection error {:?}", e),
                            }
                        }

                        /********************************************************************************
                         * 
                         * Connection closed or login aborted
                         * 
                         ********************************************************************************/
                        Ok(None) => debug_log!("::main -> Login aborted or client disconnected during handshake."),


                        /********************************************************************************
                         * 
                         * Login failure
                         * 
                         ********************************************************************************/
                        Err(e) => debug_log!("::main -> Login error during handshake: {:?}", e),
                    }
                });
            }
            Err(e) => debug_log!("::main -> Accept error (failed to accept incoming TCP connection): {:?}", e),
        }
    }

    Ok(())
}