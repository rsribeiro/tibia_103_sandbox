pub mod message;

use crate::debug_log;
use message::{PlayerToWorld, WorldToPlayer};
use std::{collections::BTreeMap, sync::Arc};
use tokio::{
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        RwLock,
    },
    task,
};


/********************************************************************************
 * 
 * World state
 * 
 * Receives messages from all connected players and distributes messages back
 * to them. This acts as the central communication hub between connections.
 * 
 ********************************************************************************/
pub struct World {
    sender:   UnboundedSender<PlayerToWorld>,
    receiver: UnboundedReceiver<PlayerToWorld>,
}


impl World {
    /********************************************************************************
     * 
     * Create a new world instance and communication channel.
     * 
     ********************************************************************************/
    pub fn new() -> Arc<RwLock<Self>> {
        let (sender, receiver) = unbounded_channel();
        Arc::new(RwLock::new(Self { sender, receiver }))
    }


    /********************************************************************************
     * 
     * Get a sender that can be used to send messages to the world.
     * 
     ********************************************************************************/
    pub fn sender(&self) -> UnboundedSender<PlayerToWorld> {
        self.sender.clone()
    }


    /********************************************************************************
     * 
     * Start the world message processing loop.
     * 
     ********************************************************************************/
    pub fn start(world: &Arc<RwLock<Self>>) {
        task::spawn(Self::message_loop(world.clone()));
    }


    /********************************************************************************
     * 
     * Main world message loop.
     * 
     ********************************************************************************/
    async fn message_loop(world: Arc<RwLock<Self>>) {
        let mut senders: BTreeMap<u32, UnboundedSender<WorldToPlayer>> = BTreeMap::new();

        loop {
            let msg = world.write().await.receiver.recv().await;
            match msg {
                /********************************************************************************
                 * 
                 * Player login
                 * 
                 * Register the player's communication channel so the world can send messages back
                 * to that player.
                 * 
                 ********************************************************************************/
                Some(PlayerToWorld::Login(player, sender)) => {
                    // Print the entire Player object (verbose logging)
                    // debug_log!("Player logged in:\n{:#?}", player);
                    
                    senders.insert(player.id, sender);
                    debug_log!("Player {} (id={}) has logged in.", player.name, player.id);
                }


                /********************************************************************************
                 * 
                 * Player logout
                 * 
                 * Remove the player's communication channel from the world.
                 * 
                 ********************************************************************************/
                Some(PlayerToWorld::Logout(player)) => {
                    senders.remove(&player.id);
                    debug_log!("Player {} (id={}) has logged out.", player.name, player.id);
                }


                /********************************************************************************
                 * 
                 * World channel closed.
                 * 
                 ********************************************************************************/
                None => break,
            }
        }
    }
}