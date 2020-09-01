use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;
    sl.set_global_volume(4.0);
    let mut speech = audio::Speech::default();

    let strings: Vec<String> = std::env::args().collect();

    if strings.len() < 2 {
        speech.set_text("Please provide command line arguments!")?;
        sl.play(&speech);
        while sl.active_voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    } else {
        for i in 1..strings.len() {
            speech.set_text(&strings[i])?;

            sl.play(&speech);
            while sl.active_voice_count() > 0 {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    Ok(())
}
