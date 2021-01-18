use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sl = Soloud::default()?;

    let mut wav = audio::Wav::default();

    let bytes = std::fs::read("sample.wav")?;

    wav.load_mem(bytes)?;

    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
