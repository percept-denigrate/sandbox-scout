use anyhow::Result;
use serenity::{
    http::Http,
    model::{
        channel::Embed,
        webhook::Webhook,
    },
};

mod consts;

fn main() {
  
}

async fn send(data: String) -> Result<()> {
    let http = Http::new("token");
    let url = consts::WEBHOOK;

    // Create webhook.
    let webhook = Webhook::from_url(&http, url).await?;

    // Embed [data] in a discord message.
    let embed = Embed::fake(|e| {
        e.title("System info");
        e.description(data);
        e
    });

    // Execute Webhook, add metadata, content and files.
    webhook
        .execute(&http, true, |w| {
            w.content("content")
                .username("username")
                .embeds(vec![embed]);
            w
        })
        .await?;
    Ok(())
}