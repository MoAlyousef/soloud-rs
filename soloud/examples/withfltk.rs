// use fltk::{app::*, button::*, window::*};
// use soloud::*;

// fn main() {
//     let app = App::default();
//     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
//     let mut but = Button::new(160, 210, 80, 40, "Click me!");

//     let mut sl = Soloud::default().unwrap();
//     sl.set_global_volume(4.0);


//     wind.end();
//     wind.show();

//     but.set_callback(Box::new(move || {
//     	   let mut wav = audio::Wav::default();
//         wav.load(&std::path::Path::new("sample.wav")).unwrap();
//         sl.play(&wav);
//         while sl.active_voice_count() > 0 {
//             app.wait().unwrap();
//         }
//     }));

//     app.run().unwrap();
// }
