use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;
    sl.set_global_volume(3.0);

    let mut wav = audio::Wav::default();

    wav.load("sample.wav")?;

    sl.play(&wav);
    while sl.voice_count() > 0 {
        // calls to play are non-blocking, so we put the thread to sleep
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    wav.load("Recording.mp3")?;

    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
