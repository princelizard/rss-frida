use std::fs::{self, File};
slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), slint::platform::PlatformError> {
    create_file();
    let ui = MainWindow::new().unwrap();

    ui.run()
}

fn create_file(){
    if !fs::exists("feeds.json").expect("Should be able to access file system."){
        File::create("feeds.json").expect("Should be able to create file.");
    }
}

async fn add_feed(feed_url: String){
    let body = reqwest::get(feed_url).await.unwrap().text().await.unwrap();
}
