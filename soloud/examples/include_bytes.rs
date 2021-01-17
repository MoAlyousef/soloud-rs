fn main() {
    use soloud::*;
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    // wav.load_mem(include_bytes!("../audio.mp3")).unwrap();
    // sl.play(&wav);
    // while sl.voice_count() > 0 {
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    // }
}
