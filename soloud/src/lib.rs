pub mod prelude;
pub mod soloud;
pub mod wav;

pub use prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::*;
        let mut sl = soloud::Soloud::default();
        let mut wav = wav::Wav::default();
        let path = std::path::Path::new("../../ds_china.wav");
        wav.load(&path).unwrap();
        sl.init().unwrap();
        sl.play(&wav).unwrap();
        sl.deinit();

    }
}
