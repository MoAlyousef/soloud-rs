# soloud-sys

Raw bindings to soloud. These are generated using bindgen on the soloud C headers.

## Usage
```toml
[dependencies]
soloud-sys = { version = "1", features = ["miniaudio"] }
```

Example code:
```rust,no_run
use soloud_sys::soloud::*;

fn main() {
    unsafe {
        let sl = Soloud_create();
        Soloud_init(sl);
        std::thread::sleep(std::time::Duration::from_millis(100));
        Soloud_setGlobalVolume(sl, 3.0);
    
        let speech = Speech_create();
    
        let ret = Speech_setText(speech, "Hello World\0".as_ptr() as _);
    
        dbg!(ret);

        Soloud_play(sl, speech);
        while Soloud_getVoiceCount(sl) > 0 {
            // calls to play are non-blocking, so we put the thread to sleep
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
```