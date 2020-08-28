use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;
    sl.set_global_volume(3.0);

    let mut wav = audio::Wav::default();
    let mut filt = filter::EchoFilter::default();
    filt.set_params(0.2)?; // Here sets the delay by default for echo filters

    wav.load(&std::path::Path::new("sample.wav"))?;
    wav.set_filter(0, Some(&filt));

    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
