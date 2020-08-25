use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;
    sl.set_global_volume(4.0);

    let mut speech = speech::Speech::default();

    speech.set_text("Hello World")?;

    sl.play(&speech);
    while sl.get_active_voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    speech.set_text("1 2 3")?;

    sl.play(&speech);
    while sl.get_active_voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    speech.set_text("Can you hear me?")?;

    sl.play(&speech);
    while sl.get_active_voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    sl.deinit();
    
    Ok(())
}
