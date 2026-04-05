use std::fs::{self, File};

use rss::Channel;
slint::include_modules!();

#[derive(serde::Serialize)]
struct Feed {
    title: String,
    link: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), slint::platform::PlatformError> {
    create_file();
    let ui = MainWindow::new().unwrap();

    ui.on_submit_feed(move |feed_url| {
        let feed_url = feed_url.to_string();
        tokio::spawn(async move {
            add_feed(feed_url).await;
        });
    });
    ui.run()
}

fn create_file(){
    if !fs::exists("feeds.json").expect("Should be able to access file system."){
        File::create("feeds.json").expect("Should be able to create file.");
    }
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
    fs::write("feeds.json", json).expect("Failed to write to feeds.json");
}