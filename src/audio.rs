use std::io::{Cursor, Read};
use rodio::{Decoder, MixerDeviceSink, Source, source};


pub fn play_url(url: &str) {
    let handle = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
    let mut buf = Vec::new();
    ureq::get(url).call().unwrap()
        .into_body().into_reader()
        .read_to_end(&mut buf).unwrap();

    let player = rodio::play(&handle.mixer(), Cursor::new(buf)).unwrap();
    player.sleep_until_end();
}