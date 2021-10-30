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
        id::UserId,
        permissions::Permissions,
    },
    prelude::*,
};

use std::sync::Arc;
use std::time::{Duration, SystemTime};

use tokio::time::sleep;

#[command]
#[allowed_roles("Sentinels")]
pub async fn reminder(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<u64>()?;
    let interval = args.single::<u64>()?;
    let command = args.rest();
    println!("Message recieved, first is {}, inteval is {} and command is {}", first, interval, command);

    loop {
        sleep(Duration::from_secs(30)).await;
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(v) => {
                if v.as_secs() > first {
                    break;
                }
            }
            Err(why) => {
                println!("Something went wrong with time check, {:?}", why);
                ()
            }
        }
    }
    loop {
        sleep(Duration::from_secs(interval)).await;
        if let Err(why) = msg.channel_id.say(&ctx.http, command).await {
            println!("Error sending message, {:?}", why);
            ()
        }
    }
    Ok(())
}
