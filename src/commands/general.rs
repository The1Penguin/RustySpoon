use once_cell::sync::OnceCell;
use reqwest::{cookie::Jar, header::COOKIE, Method, Response, Url};
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
    env, fs,
    net::{SocketAddr, TcpStream},
    path::Path,
    sync::Arc,
    time::{Duration, SystemTime},
};

use roux::{responses, User};

use chrono::{DateTime, Local, NaiveDateTime, Utc};

use serde_json::*;

static NODES: OnceCell<HashMap<(chrono::NaiveTime, chrono::NaiveTime), (String, String)>> =
    OnceCell::new();
static FC_NUMBER: OnceCell<String> = OnceCell::new();
static CHEST_NUMBER: OnceCell<String> = OnceCell::new();
lazy_static! {
    static ref SESS: String = env::var("LDST_SESS").expect("Expected a token in the environment");
}

pub fn node_generate() -> &'static HashMap<(chrono::NaiveTime, chrono::NaiveTime), (String, String)>
{
    NODES.get_or_init(|| {
        let json = fs::read_to_string("./out.json").expect("Error reading json");
        let vals: Value = serde_json::from_str(&json).expect("Error converting json");
        let mut ret: HashMap<(chrono::NaiveTime, chrono::NaiveTime), (String, String)> =
            HashMap::new();
        for i in vals["items"].as_array().unwrap() {
            ret.insert(
                (
                    chrono::NaiveTime::parse_from_str(
                        i["start"].as_str().expect("Error parsing json query"),
                        "%H:%M",
                    )
                    .expect("Error converting time to DateTime"),
                    chrono::NaiveTime::parse_from_str(
                        i["end"].as_str().expect("Error parsing json query"),
                        "%H:%M",
                    )
                    .expect("Error converting time to DateTime"),
                ),
                (
                    i["name"].as_str().unwrap().to_string(),
                    i["location"].as_str().unwrap().to_string(),
                ),
            );
        }
        ret
    })
}

pub fn init(fc_number: String, chest_number: String) -> &'static String {
    FC_NUMBER.get_or_init(|| fc_number);
    CHEST_NUMBER.get_or_init(|| chest_number)
}

#[command]
pub async fn help(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("General Commands");
            e.description(
                "help - This command which shows all available commands \n
                 down - Pings the Louisoix server to see if it is available \n
                 fashion - Shows the latest fashion report requirements for the week \n
                 nodes - Shows the materials able to be harvested at timed nodes and where to find them \n
                "
                );
            e
        });
        m
    }).await {
        println!("Error sending message: {:?}", why);
        ()
    }
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Sentinels Commands");
            e.description(
                "
                 reminder - takes a unix timestamp, then an interval and lastly a command, will then on run that command every inteval after the unix timestamp.
                 Example `<reminder 0 10 fashion` will create a fashion report post every 10 seconds.
                 Available commands there are fashion, cactpot or any string you want. \n
                 disable_reminder - uses the uuid given from reminder, and disables it
                 Example `<disable_reminder f6cb4bc5-ea8a-4bcb-b21a-97b03ab56dba` \n
                "
                );
            e
        });
        m
    }).await {
        println!("Error sending message: {:?}", why);
        ()
    }

    Ok(())
}

#[command]
#[bucket = "requests"]
pub async fn chest(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let cookie = format!("ldst_sess={}", SESS.as_str());
    let url = format!(
        "https://na.finalfantasyxiv.com/lodestone/freecompany/{}/chest/{}",
        match FC_NUMBER.get() {
            Some(v) => v,
            None => {
                println!("Error getting FC_NUMBER");
                return Ok(());
            }
        },
        match CHEST_NUMBER.get() {
            Some(v) => v,
            None => {
                println!("Error getting CHEST_NUMBER");
                return Ok(());
            }
        }
    )
    .parse::<Url>()
    .unwrap();
    let jar = Arc::new(Jar::default());
    jar.add_cookie_str(cookie.as_str(), &url);
    let client = match reqwest::ClientBuilder::new()
        .cookie_store(true)
        .cookie_provider(jar)
        .build()
    {
        Ok(v) => v,
        Err(why) => {
            println!("Error creating reqwest client: {:?}", why);
            return Ok(());
        }
    };
    let res = client.get(url).send().await;

    let res = match res {
        Ok(v) => v,
        Err(why) => {
            println!("Error sending message: {:?}", why);
            return Ok(());
        }
    };

    for i in res.cookies() {
        println!("{:?}", i);
    }

    Ok(())
}

#[command]
#[bucket = "requests"]
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
#[bucket = "requests"]
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
#[bucket = "requests"]
pub async fn nodes(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let eorzea_time = time_to_eorzea(Local::now()).await;
    let mut message = "All active nodes right now are:\n".to_owned();
    let hash = match NODES.get() {
        Some(v) => v,
        None => return Ok(()),
    };
    for (times, (name, location)) in hash.iter() {
        if within_time(*times, eorzea_time).await {
            message.push_str(&format!("{} in {}\n", &name, &location));
        }
    }
    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
        println!("Error sending message: {:?}", why);
        ()
    };

    Ok(())
}

pub async fn within_time(
    times: (chrono::NaiveTime, chrono::NaiveTime),
    now: chrono::NaiveTime,
) -> bool {
    let mut temp = times.1;
    if times.1 < times.0 {
        temp = times.1 + chrono::Duration::days(1);
    }

    times.0 <= now && now <= temp
}

pub async fn time_to_eorzea(date: chrono::DateTime<chrono::Local>) -> chrono::NaiveTime {
    let temp: chrono::DateTime<Utc> = DateTime::from_utc(
        NaiveDateTime::from_timestamp(date.timestamp() * 3600 / 175, 0),
        Utc,
    );
    temp.time()
}
