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
    collections::{HashMap},
    time::{Duration, SystemTime},
};

use tokio::{task::JoinHandle, time::sleep};

use uuid::Uuid;

lazy_static! {
    static ref ACTIVE: Mutex<HashMap<uuid::Uuid, JoinHandle<()>>> = Mutex::new(HashMap::new());
}

#[command]
#[allowed_roles("Sentinels")]
pub async fn reminder(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<u64>()?;
    let interval = args.single::<u64>()?;
    let command = args.rest().to_owned();

    loop {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(v) => {
                if v.as_secs() > first {
                    break;
                }
            }
            Err(why) => {
                println!("Something went wrong with time check, {:?}", why);
            }
        }
        sleep(Duration::from_secs(30)).await;
    }
    let uuid = Uuid::new_v4();
    let http = ctx.http.clone();
    let channel_id = msg.channel_id;

    if let Err(why) = channel_id
        .say(&http, format!("Uuid for the message is {}", uuid))
        .await
    {
        println!("Error sending message, {:?}", why);
    }

    let join_handlertask = tokio::task::spawn(async move {
        loop {
            match &command as &str{
                "cactpot" => cactpot (&http, &channel_id).await,
                other => {
                    if let Err(why) = channel_id.say(&http, other).await {
                        println!("Error sending message, {:?}", why);
                    }
                }
            }
            sleep(Duration::from_secs(interval)).await;
        }
    });

    ACTIVE.lock().await.insert(uuid, join_handlertask);

    Ok(())
}

#[command]
#[allowed_roles("Sentinels")]
pub async fn disable_reminder(_ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let command = args.rest();
    let uuid = match Uuid::parse_str(command) {
        Ok(v) => v,
        Err(why) => {
            println!("Error parsing string, {}", why);
            return Ok(());
        }
    };
    match ACTIVE.lock().await.get(&uuid) {
        Some(v) => {
            v.abort();
        }
        None => {
            return Ok(());
        }
    }

    ACTIVE.lock().await.remove(&uuid);

    return Ok(());
}

pub async fn cactpot(http: &Http, channel_id: &ChannelId){
    if let Err(why) = channel_id.send_message(http, |m| {
    m.embed(|e| {
        e.title("The weekly Jumbo Cactpot");
        e.description("Jumbo Cactpot is coming soon, and if you want the early bird bonus, you better get down there");

        e
    });

    m

    }).await {
        println!("Error sending message: {:?}", why);
        return;
    }
    return;
}
