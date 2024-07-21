use serenity::all::{ChannelId, MessageId};
use serenity::prelude::*;

pub async fn send_message(context: &Context, channel_id: ChannelId, msg: &str) {
    let _ = channel_id.say(&context.http, msg).await;
}

pub async fn delete_message(context: &Context, channel_id: ChannelId, message_id: MessageId) {
    let _ = channel_id.delete_message(&context.http, message_id).await;
}