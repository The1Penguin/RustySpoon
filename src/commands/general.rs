use once_cell::sync::OnceCell;
use reqwest::{cookie::Jar, Url};
use select::{
    document::Document,
    predicate::Class,
};
use serenity::{
    framework::standard::{
        macros::command,
        Args, CommandResult,
    },
    http::Http,
    model::{
        channel::Message,
        id::ChannelId,
    },
    prelude::*,
};

use std::{
    collections::HashMap,
    env, fs,
    net::{SocketAddr, TcpStream},
    sync::Arc,
};

use roux::User;

use chrono::{DateTime, Local, NaiveDateTime, Utc};

use serde_json::*;

static NODES: OnceCell<HashMap<(chrono::NaiveTime, chrono::NaiveTime), (String, String)>> =
    OnceCell::new();
static FC_NUMBER: OnceCell<String> = OnceCell::new();
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

pub fn init(fc_number: String) -> &'static String {
    FC_NUMBER.get_or_init(|| fc_number)
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

pub struct ChestItem {
    item: String,
    amount: String,
}

#[command]
#[bucket = "requests"]
pub async fn chest(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let chest_number = args.single::<u64>()?;
    if !(1 <= chest_number && chest_number <= 5) {
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, "Choose a number between 1 and 5")
            .await
        {
            println!("Error sending message: {:?}", why);
            ()
        }
        return Ok(());
    }
    let items = get_items(chest_number).await;

    let mut message = format!("Items in chest {} are:\n", chest_number);
    for i in items {
        message.push_str(&format!("{}, amount: {}\n", i.item, i.amount));
    }
    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
        println!("Error sending message: {:?}", why);
        ()
    };

    Ok(())
}

pub async fn get_items(chest_number: u64) -> Vec<ChestItem> {
    let cookie = format!("ldst_sess={}", SESS.as_str());
    let url = format!(
        "https://na.finalfantasyxiv.com/lodestone/freecompany/{}/chest/{}",
        match FC_NUMBER.get() {
            Some(v) => v,
            None => {
                println!("Error getting FC_NUMBER");
                return vec![];
            }
        },
        chest_number
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
            return vec![];
        }
    };
    let res = client.get(url).send().await;

    let res = match res {
        Ok(v) => v,
        Err(why) => {
            println!("Error sending message: {:?}", why);
            return vec![];
        }
    };

    let document = Document::from(res.text().await.unwrap().as_str());

    let mut items = vec![];

    for node in document.find(Class("item-list__list")) {
        let item = ChestItem {
            item: node
                .find(Class("db-tooltip__item__name"))
                .next()
                .unwrap()
                .first_child()
                .expect("Expected a child")
                .text(),
            amount: node
                .find(Class("item-list__number"))
                .next()
                .unwrap()
                .first_child()
                .expect("Expected a child")
                .text(),
        };
        items.push(item);
    }

    items
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
    if let Err(why) = channel_id.say(http, &link).await {
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
