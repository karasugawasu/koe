use crate::status::VoiceConnectionStatusMap;
use crate::voice_client::VoiceClient;
use anyhow::{Context, Result};
use koe_speech::SpeechProvider;
use serenity::Client;
use songbird::SerenityInit;

mod command;
mod context_store;
mod handler;
mod speech;
mod status;
mod voice_client;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = koe_config::load()?;

    let speech_provider = SpeechProvider::new(config.google_application_credentials).await?;
    let voice_client = VoiceClient::new();
    let status_map = VoiceConnectionStatusMap::new();

    let mut client = Client::builder(config.discord_bot_token)
        .event_handler(handler::Handler)
        .application_id(config.discord_client_id)
        .register_songbird()
        .await
        .context("Failed to build serenity client")?;

    context_store::insert(&client, speech_provider).await;
    context_store::insert(&client, voice_client).await;
    context_store::insert(&client, status_map).await;

    client.start().await.context("Client error occurred")?;

    Ok(())
}