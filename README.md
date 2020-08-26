# soloud-rs

Rust bindings for the soloud audio engine library.

Supported formats: wav, mp3, ogg. The library also comes with a speech synthesizer.

Still pre-alpha.

- The official soloud [website](https://sol.gfxile.net/soloud/index.html)
- The official soloud [repo](https://github.com/jarikomppa/soloud)

## Usage
```toml
[dependencies]
soloud = { git = "https://github.com/moalyousef/soloud-rs" }
```

To play audio:
```rust
use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;
    sl.set_global_volume(3.0);

    let mut wav = wav::Wav::default();

    wav.load(&std::path::Path::new("sample.wav"))?;

    sl.play(&wav);
    while sl.get_voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    wav.load(&std::path::Path::new("Recording.mp3"))?;
    
    sl.play(&wav);
    while sl.get_voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    sl.deinit();

    Ok(())
}
```

To play speech:
```rust
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
```

## Dependencies
A Rust compiler, a C++ compiler, Cargo, CMake and git (all these need to be in your PATH). This crate uses the miniaudio backend which assumes default sound drivers are functional.
