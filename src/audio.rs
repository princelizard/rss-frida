use std::io::{Cursor, Read};
use rodio::{Decoder, MixerDeviceSink, Source, source};


pub fn play_url(url: String){
    let handle = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
    let player = rodio::Player::connect_new(&handle.mixer());
    let mut res = ureq::get(&url).call().unwrap().into_body().into_reader();
    
    let mut buf = Vec::new();
    let mut res = ureq::get(url).call().unwrap().into_body().into_reader();
    res.read_to_end(&mut buf).unwrap();

    let cursor = Cursor::new(buf);
    let source = Decoder::new(cursor).unwrap();
    player.append(source);
    player.sleep_until_end();
}