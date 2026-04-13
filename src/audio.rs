use std::io::{Cursor, Read};
use rodio::{Decoder, Player, DeviceSinkBuilder, MixerDeviceSink};
use std::time::Duration;

pub struct AudioPlayer {
    player: Player,
    _handle: rodio::MixerDeviceSink,  // must stay alive!
}

impl AudioPlayer {
    pub fn new() -> Self {
        let handle = DeviceSinkBuilder::open_default_sink().unwrap();
        let player = Player::connect_new(&handle.mixer());
        Self { player, _handle: handle }
    }

    pub fn load_url(&self, url: &str) {
        let mut buf = Vec::new();
        ureq::get(url).call().unwrap()
            .into_body().into_reader()
            .read_to_end(&mut buf).unwrap();

        let source = Decoder::new(Cursor::new(buf)).unwrap();
        self.player.append(source);
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn play(&self) {
        self.player.play();
    }

    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    pub fn seek(&self, pos: Duration) {
        match self.player.try_seek(pos) {
            Ok(()) => {}
            Err(e) => eprintln!("Seek error: {e}"),
        }
    }

    pub fn position(&self) -> Duration {
        self.player.get_pos()
    }
}