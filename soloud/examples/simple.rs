use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = soloud::Soloud::default();
    let mut wav = wav::Wav::default();
    let path = std::path::Path::new("ds_china.wav");
    wav.load(&path)?;
    sl.init()?;
    sl.play(&wav)?;
    sl.deinit();
    Ok(())
}
