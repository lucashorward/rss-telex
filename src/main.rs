use clap::Parser;
use core::time;
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::collections::HashSet;
use std::io::{self, Write};
use std::thread;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 5)]
    delay: u64,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 10)]
    speed: u64,

    #[arg(short, long)]
    url: String,
}

#[derive(Deserialize, Debug)]
struct Item {
    title: String,
    description: String,
    link: String,
    guid: String,
}
#[derive(Deserialize, Debug)]
struct Channel {
    title: String,
    description: String,
    item: Vec<Item>,
}
#[derive(Deserialize, Debug)]
struct Rss {
    channel: Channel,
}

fn slow_type(input: &String, speed: u64) {
    let time_to_wait = time::Duration::from_millis(speed);
    for char in input.chars() {
        print!("{}", char);
        io::stdout().flush().unwrap();
        thread::sleep(time_to_wait);
    }
    print!("\n");
    io::stdout().flush().unwrap();
}

fn read_title(channel: &Channel, speed: u64) {
    slow_type(&String::from("---------------"), speed);
    slow_type(
        &format!(
            "{}{}",
            "The channel you're listening to is: ", channel.title
        )
        .to_string(),
        speed,
    );
    slow_type(&channel.description, speed);
}

fn read_items(items: Vec<Item>, guids: &mut HashSet<String>, speed: u64) {
    let filtered: Vec<Item> = items
        .into_iter()
        .filter(|item| !guids.contains(&item.guid))
        .collect();
    for item in filtered {
        slow_type(&String::from("---------------"), speed);
        slow_type(&item.title, speed);
        slow_type(&item.description, speed);
        slow_type(&item.link, speed);
        guids.insert(item.guid.to_string());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // TODO verify whether this is a valid URL regex
    let rss_url = &args.url;
    let speed = args.speed;
    println!("{:?}", rss_url);

    let mut guids: HashSet<String> = HashSet::new();

    let time_between_req = time::Duration::from_secs(args.delay);
    let mut should_read_title = true;
    loop {
        let resp = reqwest::get(rss_url).await?.text().await?;
        let parsed: Rss = from_str(&resp).unwrap();
        if should_read_title {
            read_title(&parsed.channel, speed);
            should_read_title = false;
        }
        read_items(parsed.channel.item, &mut guids, speed);
        slow_type(&String::from("---------------"), speed);
        slow_type(&String::from("Waiting 5 seconds"), speed);
        thread::sleep(time_between_req);
    }
}
