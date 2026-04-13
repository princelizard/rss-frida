use std::fs::{self, File};
use std::io::Write;
use std::collections::HashMap;
use std::process::Command;
use rss::Channel;
use serde::Serialize;
use slint::{VecModel, ModelRc};
use std::sync::{Arc, Mutex};

slint::include_modules!();

#[derive(serde::Serialize, serde::Deserialize)]
struct Feed {
    title: String,
    link: String,
    description: String,
}

fn main() -> Result<(), slint::platform::PlatformError> {
    let ui = MainWindow::new().unwrap();
    let channels_map = Arc::new(Mutex::new(generate_hashmap()));
    populate_channels(channels_map.clone(), &ui); 
    
    ui.on_submit_feed(move |feed_url| { // this needs to generate a ui borrow
        let feed_url = feed_url.to_string();
        let map_clone = Arc::clone(&channels_map);

        if feed_url.ends_with("xml") { //TODO: show the user an angry rectangle on invalid link
            std::thread::spawn(move || {
                if let Ok(feed) = add_feed(feed_url) {
                    let mut map = map_clone.lock().unwrap();
                    map.insert(feed.title, feed.link);
                }
            });
        }
    });
 
    ui.on_select_channel({
        let ui_weak = ui.as_weak();
        move |channel_info| {
            let ui_weak = ui_weak.clone();
            std::thread::spawn( move || {
                populate_episodes(channel_info, ui_weak);
            }); 
        }
    });

    ui.on_select_episode(|episode_info|{
        open_browser(&episode_info.audio_url.into());
    } );
    ui.run()
}

//NOTE: feeds.json is actually a jsonl file. This is improper, but it won't parse otherwise. #whocare 
fn add_feed(feed_url: String) -> Result<Feed, ureq::Error> {
    let feed_url_copy = feed_url.clone();
    let xml = ureq::get(feed_url).call()?.body_mut().read_to_string()?;
    let contents = Channel::read_from(xml.as_bytes()).unwrap();
    
    let feed = Feed {
        title: contents.title().to_string(),
        link: feed_url_copy,
        description: contents.description().to_string(),
    };
    
    let json = serde_json::to_string(&feed).unwrap();
    let mut file = File::options().append(true).create(true).open("feeds.json").unwrap();
    writeln!(file, "{}", json).unwrap();

    Ok(feed)
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

fn populate_channels(map: Arc<Mutex<HashMap<String, String>>>, ui: &MainWindow) {
    let channels: Vec<ChannelData> = map.lock().unwrap().iter().map(|(title, link)| ChannelData {
        channel_title: title.into(),
        channel_url: link.into(),
    }).collect();
    
    let channels = ModelRc::new(VecModel::from(channels));
    ui.set_channels(channels);
}

fn populate_episodes(channel_info: ChannelData, ui: slint::Weak<MainWindow>) {
    let xml = ureq::get(channel_info.channel_url.to_string()).call().unwrap().body_mut().read_to_string().unwrap();
    let contents = Channel::read_from(xml.as_bytes()).unwrap();
    
    let episodes: Vec<EpisodeData> = contents.items().iter().map(|item| EpisodeData {
        audio_url: item.enclosure().map(|e| e.url().to_string()).unwrap_or_default().into(),
        episode_title: item.title().unwrap_or_default().to_string().into(),
    }).collect();

    slint::invoke_from_event_loop(move || {
        if let Some(ui) = ui.upgrade() {
            let episodes = ModelRc::new(VecModel::from(episodes));
            ui.set_episodes(episodes);
        }
    }).unwrap();
}
#[cfg(target_os = "macos")]
fn open_browser(url: &String){Command::new("open").arg(url).spawn().unwrap();}

#[cfg(target_os = "windows")]
fn open_browser(url: &String){Command::new("cmd").args(["/c", "start", url]).spawn().unwrap();}

#[cfg(target_os = "linux")]
fn open_browser(url: &String){Command::new("xdg-open").arg(url).spawn().unwrap();}