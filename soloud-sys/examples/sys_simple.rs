use soloud_sys::soloud::*;

fn main() {
    unsafe {
        let sl = Soloud_create();
        Soloud_init(sl);
        Soloud_setGlobalVolume(sl, 3.0);

        let wav = Wav_create();

        let ret = Wav_load(wav, "覆.mp3\0".as_ptr() as _);

        dbg!(ret);

        Soloud_play(sl, wav);
        while Soloud_getVoiceCount(sl) > 0 {
            // calls to play are non-blocking, so we put the thread to sleep
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
