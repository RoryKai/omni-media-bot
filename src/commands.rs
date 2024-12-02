use serenity::all::{ChannelId, Message, MessageId};
use serenity::utils::MessageBuilder;
use serenity::prelude::*;

pub async fn send_message(context: &Context, channel_id: ChannelId, msg: &str) {
    if let Err(why) = channel_id.say(&context.http, msg).await {
    println!("Error sending message: {why:?}")}
}

pub async fn delete_message(context: &Context, channel_id: ChannelId, message_id: MessageId) {
    if let Err(why) = channel_id.delete_message(&context.http, message_id).await {
    println!("Error sending message: {why:?}")}
}

pub async fn where_message(context: &Context, message: &Message) {
    let response = MessageBuilder::new()
    .push("User ")
    .push_bold_safe(&message.author.name)
    .push(" used the 'ping' command in the ")
    .mention(&message.channel_id)
    .push(" channel")
    .build();
    if let Err(why) = message.channel_id.say(&context.http, &response).await {
    println!("Error sending message: {why:?}");}
}