use soloud::*;

fn main() {
    let mut sl = soloud::Soloud::default();
    let mut wav = wav::Wav::default();
    let path = std::path::Path::new("ds_china.wav");
    wav.load(&path).unwrap();
    sl.init().unwrap();
    sl.play(&wav).unwrap();
    sl.deinit();
}
