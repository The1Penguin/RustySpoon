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

lazy_static!(
    static ref ACTIVE_LIST: [bool; 16] = [false;16];
    static ref COMMAND_LIST: [&'static str;16] = ["";16];
);

async fn reoccur(id: u8, ctx: &Context, msg: &Message, first: u64, interval: u64){
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
    loop {
        println!("In second loop");
        if let Err(why) = msg.channel_id.say(&ctx.http, (*COMMAND_LIST)[id as usize]).await {
            println!("Error sending message, {:?}", why);
            ()
        }
        sleep(Duration::from_secs(interval)).await;
    }

}

#[command]
#[allowed_roles("Sentinels")]
pub async fn reminder(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<u64>()?;
    let interval = args.single::<u64>()?;
    let command = args.rest();
    println!("Message recieved, first is {}, inteval is {} and command is {}", first, interval, command);

    let mut active = *ACTIVE_LIST;
    let mut commands = *COMMAND_LIST;
    
    for i in 0..16 {
        if active[i] == false {
            active[i] = true;
            commands[i] = command;
            println!("Put on index {}", i);
            reoccur(i as u8, ctx, msg, first, interval).await;
            break;
        }
    }

    Ok(())
}

#[command]
#[allowed_roles("Sentinels")]
pub async fn disable_reminder(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let command = args.rest();
    let mut active = *ACTIVE_LIST;
    let mut commands = *COMMAND_LIST;
    for i in 1..16 {
        if commands[i] == command {
            active[i] = false;
            commands[i] = "";
            return Ok(());
        }
    }
    return Ok(());
}
