use serenity::{
    async_trait,
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args, CommandGroup, CommandOptions, CommandResult, DispatchError, HelpOptions, Reason,
        StandardFramework,
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::{ChannelId, UserId},
        permissions::Permissions,
    },
    prelude::*,
};

use std::{
    collections::{HashMap, HashSet},
    net::{SocketAddr, TcpStream},
    time::{Duration, SystemTime},
    path::Path,
    fs,
};

use roux::User;

use chrono::*;

use serde_json::Value;

struct Node {
    start: chrono::DateTime<Utc>,
    end: chrono::DateTime<Utc>,
}


// lazy_static! {
//     static ref ACTIVE: HashMap<String, Node> = {
//         let json = match fs::read_to_string("./persons.json"){
//             Ok(v) => v,
//             Err(why) => {
//                 println!("Error reading submitted, {}", why);
//             }
//         };
//         sub_values: HashMap<String, Node> = serde_json::from_str(&json)
//         };
// }

#[command]
pub async fn down(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let addrs = [SocketAddr::from(([195, 82, 50, 47], 54992))];
    if let Ok(_) = TcpStream::connect(&addrs[..]) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Louisoix is up").await {
            println!("Error sending message: {:?}", why);
            ()
        }
    } else {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Louisoix is down").await {
            println!("Error sending message: {:?}", why);
            ()
        }
    }

    Ok(())
}

#[command]
pub async fn fashion(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    fashion_helper(&ctx.http, &msg.channel_id).await;
    return Ok(());
}

pub async fn fashion_helper(http: &Http, channel_id: &ChannelId) {
    let user = User::new("kaiyoko");
    let link = match user.submitted().await {
        Ok(v) => {
            let mut templink: String = "".to_owned();
            for i in v.data.children {
                if i.data.title.contains("Fashion Report - Full Details") {
                    templink = match i.data.url {
                        Some(v) => v,
                        None => return,
                    };
                    break;
                }
            }
            templink
        }
        Err(why) => {
            println!("Error reading submitted, {}", why);
            return;
        }
    };
    if let Err(why) = channel_id.say(http, &link as &str).await {
        println!("Error sending message: {:?}", why);
        ()
    }
}

#[command]
pub async fn eorzea(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let eorzea_time = time_to_eorzea(Local::now()).await;
    if let Err(why) = msg.channel_id.say(&ctx.http, format!("{}", eorzea_time)).await {
        println!("Error sending message: {:?}", why);
        ()
    }
    
    Ok(())

}

pub async fn time_to_eorzea(date: chrono::DateTime<chrono::Local>) -> chrono::DateTime<Utc> {
    DateTime::from_utc(
        NaiveDateTime::from_timestamp(date.timestamp() * 3600 / 175, 0),
        Utc,
    )
}
