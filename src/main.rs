use std::fs::{self, File};
use std::io::Write;
use std::collections::HashMap;
use rss::Channel;
use slint::{VecModel, ModelRc};

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

    populate_channels(&channels_map, &ui);

    ui.on_submit_feed(move |feed_url| {
        let feed_url = feed_url.to_string();
        if feed_url.ends_with("xml") {
            tokio::spawn(async move {
                let _ = add_feed(feed_url).await;
            });
        }
    });
    ui.run()
}

//NOTE: feeds.json is actually a jsonl file. This is improper, but it won't parse otherwise, and I can't be assed to fix it. 
async fn add_feed(feed_url: String) -> Result<(), reqwest::Error> {
    let xml = reqwest::get(feed_url).await?.text().await?;
    let channel = Channel::read_from(xml.as_bytes()).unwrap();
    let feed = Feed {
        title: channel.title().to_string(),
        link: channel.link().to_string(),
        description: channel.description().to_string(),
    };
    
    let json = serde_json::to_string(&feed).unwrap();
    let mut file = File::options().append(true).create(true).open("feeds.json").unwrap();
    writeln!(file, "{}", json).unwrap();
    // should either regenerate hashmap or update it. do this by returning a true or hashmap reference or something. let the closure decide.
    Ok(())
}

fn generate_hashmap() -> HashMap<String, String> {
    let contents = fs::read_to_string("feeds.json").unwrap_or_default();
    let hashmap = contents.lines().filter_map(|line| {
            let feed: Feed = serde_json::from_str(line).unwrap();
            Some((feed.title, feed.link))
        })
        .collect();

    hashmap
}

fn populate_channels(map: &HashMap<String, String>, ui: &MainWindow) {
    let channels: Vec<ChannelData> = map.iter().map(|(title, link)| ChannelData {
        channel_title: title.into(),
        channel_url: link.into(),
    }).collect();
    let model = ModelRc::new(VecModel::from(channels));
    ui.set_channels(model);
}
