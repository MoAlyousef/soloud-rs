use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = soloud::Soloud::new()?;  
    let path = std::path::Path::new("sample.wav");
    let wav = wav::Wav::from_path(path)?;
    sl.set_global_volume(2.0);
    sl.play(&wav);
    while sl.get_voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    sl.deinit();
    Ok(())
}
