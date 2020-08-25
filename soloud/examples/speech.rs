use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = soloud::Soloud::default();
    let mut speech = speech::Speech::default();
    let txt = "Hello World";
    speech.set_text(txt)?;
    sl.init()?;
    sl.set_global_volume(4.0);
    sl.play(&speech);
    while sl.get_active_voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    sl.deinit();
    Ok(())
}
