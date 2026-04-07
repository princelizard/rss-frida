use std::fs::{self, File};
use std::io::Write;
use std::collections::HashMap;
use rss::Channel;

slint::include_modules!();

#[derive(serde::Serialize, serde::Deserialize)]
struct Feed {
    title: String,
    link: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), slint::platform::PlatformError> {
    let ui = MainWindow::new().unwrap();
    let mut channels_map = generate_hashmap();

    ui.on_submit_feed(move |feed_url| {
        let feed_url = feed_url.to_string();
        tokio::spawn(async move {
            add_feed(feed_url).await;
        });
    });
    ui.run()
}

async fn add_feed(feed_url: String){
    let xml = reqwest::get(feed_url).await.unwrap().text().await.unwrap();
    let channel = Channel::read_from(xml.as_bytes()).unwrap();
    let feed = Feed {
        title: channel.title().to_string(),
        link: channel.link().to_string(),
        description: channel.description().to_string(),
    };
    
    let json = serde_json::to_string(&feed).unwrap();
    let mut file = File::options().append(true).open("feeds.json").unwrap();
    writeln!(file, "{}", json).unwrap();
}

fn generate_hashmap() -> HashMap<String, String> {
    let contents = fs::read_to_string("feeds.json").unwrap();
    let mut hashmap: HashMap<String, String> = HashMap::new();
    for line in contents.lines() {
        let feed: Feed = serde_json::from_str(line).unwrap();
        hashmap.insert(feed.title, feed.link);

    }
    hashmap
}