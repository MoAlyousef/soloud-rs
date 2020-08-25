// use fltk::{app::*, button::*, window::*};
// use soloud::*;

// fn main() {
//     let app = App::default();
//     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
//     let mut but = Button::new(160, 210, 80, 40, "Click me!");

//     let mut sl = soloud::Soloud::new().unwrap();
//     sl.set_global_volume(4.0);


//     wind.end();
//     wind.show();

//     but.set_callback(Box::new(move || {
//     	let speech = wav::Wav::from_path(&std::path::Path::new("sample.wav")).unwrap();
//         sl.play(&speech);
//         while sl.get_active_voice_count() > 0 {
//             app.wait().unwrap();
//         }
//     }));

//     app.run().unwrap();
// }
