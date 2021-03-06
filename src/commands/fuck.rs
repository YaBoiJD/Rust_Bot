use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args,
    CommandResult,
    macros::command,
};

#[command]
#[aliases("foaas")]
async fn fuck(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    if args.len() == 0 {
        msg.reply(&ctx, "Please specify an endpoint, such as `!fuck version`").await?;
    }

    // Get url
    let mut url = "https://www.foaas.com/".to_string();
    for arg in args.raw() {
        url.push_str(arg);
        url.push_str("/");
    }
    
    if !(args.rest() == "version" || args.rest() == "operations") {
        url.push_str(&msg.author.name);
    }

    // Create response content
    let client = Client::new();
    let response = client.get(&url).header("Accept", "application/json").send().await?;

    // Send body to discord
    if args.rest() == "operations" {
        let body: FuckOffList = response.json().await?;

        // Send operations to discord
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Operations");
                e.description("Have your fucking operations");
                for operation in body {
                    e.field(operation.name, operation.url, true);
                }
                e.footer(|f| {
                    f.text(format!("Brought to you by {}", url))
                });
                e
            });
            m
        }).await?;

    } else {
        let body: ResponseContent = response.json().await?;
        // Send fuck off to discord
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(body.message);
                e.description(body.subtitle);
                e.footer(|f| {
                    f.text(format!("Brought to you by {}", url))
                });
                e
            });
            m
        }).await?;
    }
    Ok(())
}

#[derive(Deserialize)]
struct ResponseContent {
    message: String,
    subtitle: String,
}

pub type FuckOffList = Vec<FuckOffListElement>;

#[derive(Serialize, Deserialize)]
pub struct FuckOffListElement {
    name: String,
    url: String,
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    name: Option<String>,
    field: String,
    from: Option<String>,
}
