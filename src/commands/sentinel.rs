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

use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use tokio::time::sleep;

use uuid::Uuid;

#[command]
#[allowed_roles("Sentinels")]
pub async fn reminder(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<u64>()?;
    let interval = args.single::<u64>()?;
    let command = args.rest().to_owned();

    println!(
        "Message recieved, first is {}, interval is {} and command is {}",
        first, interval, &command as &str
    );
    loop {
        println!("In first loop");
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(v) => {
                if v.as_secs() > first {
                    println!("Finished first loop");
                    break;
                }
            }
            Err(why) => {
                println!("Something went wrong with time check, {:?}", why);
                ()
            }
        }
        sleep(Duration::from_secs(30)).await;
    }
    let http = ctx.http.clone();
    let channel_id = msg.channel_id;
    tokio::task::spawn(async move {
        loop {
            if let Err(why) = channel_id.say(&http, &command as &str).await {
                println!("Error sending message, {:?}", why);
            }
            sleep(Duration::from_secs(interval)).await;
        }
        
    });

    Ok(())
}

#[command]
#[allowed_roles("Sentinels")]
pub async fn disable_reminder(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let command = args.rest();
    return Ok(());
}
