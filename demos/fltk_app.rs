// Will need fltk as a dependency

use fltk::{app::*, button::*, window::*};
use std::cell::RefCell;
use std::rc::Rc;
use soloud::*;

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");

    let mut sl = Soloud::default().unwrap();
    sl.set_global_volume(4.0);


    wind.end();
    wind.show();

    let sl = Rc::from(RefCell::from(sl));
    let sl_c = sl.clone();
    but.set_callback(Box::new(move || {
        if sl_c.borrow().active_voice_count() > 0 { // Checks that no active audio is playing
            return;
        }
    	let mut wav = audio::Wav::default();
        wav.load(&std::path::Path::new("sample.wav")).unwrap();
        sl_c.borrow().play(&wav);
        while sl_c.borrow().active_voice_count() > 0 {
            app.wait().unwrap();
        }
    }));

    wind.set_callback(Box::new(move || { // Callback when an app closes
        sl.borrow().stop_all(); // Stop any playing audio before quitting
        app.quit();
    }));

    app.run().unwrap();
}