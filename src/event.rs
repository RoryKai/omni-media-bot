//   ___                            _       
//  |_ _|_ __ ___  _ __   ___  _ __| |_ ___ 
//   | || '_ ` _ \| '_ \ / _ \| '__| __/ __|
//   | || | | | | | |_) | (_) | |  | |_\__ \
//  |___|_| |_| |_| .__/ \___/|_|   \__|___/
//                |_|                      


use serenity::all::{ChannelId, MessageId};
use serenity::async_trait;
//use serenity::futures::channel::oneshot::channel;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::commands;



//     ____                _       
//    / ___|___  _ __  ___| |_ ___ 
//   | |   / _ \| '_ \/ __| __/ __|
//   | |__| (_) | | | \__ \ |_\__ \
//    \____\___/|_| |_|___/\__|___/

const SAY_COMMAND: &str = "!say";
const DELETE_COMMAND: &str = "!delete";

const SAY_MESSAGE: &str = "!say";

const TEMP_ID_1: u64 = 1264615266713862177;
const TEMP_ID_2: u64 = 1264325128599109644;


//    _   _                 _ _           
//   | | | | __ _ _ __   __| | | ___ _ __ 
//   | |_| |/ _` | '_ \ / _` | |/ _ \ '__|
//   |  _  | (_| | | | | (_| | |  __/ |   
//   |_| |_|\__,_|_| |_|\__,_|_|\___|_|   


pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    // Called when a new nessage is received. Thread-based multiple event handling.
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if message wasn't sent by the bot
        if msg.author.id == ctx.cache.current_user().id {return ()}
        let message = msg.content.clone();
        match msg.content.as_str() {
            SAY_COMMAND => {
                commands::send_message(&ctx, ChannelId::new(TEMP_ID_1), SAY_MESSAGE).await;
            },
            DELETE_COMMAND => {
                commands::delete_message(&ctx, ChannelId::new(TEMP_ID_1), MessageId::new(TEMP_ID_2)).await;
            },
            _ => println!("No command"),
        }

        println!("{} by {} | {}", msg.channel_id, msg.author, message);
    }

    // Ready event when a shard is booted. Contains data like the current 
    // user's guild, ids, current user data, private channels, and more.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}