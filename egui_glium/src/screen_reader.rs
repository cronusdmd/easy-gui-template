pub struct ScreenReader {
    #[cfg(feature = "screen_reader")]
    tts: Option<tts::TTS>,
}

#[cfg(not(feature = "screen_reader"))]
impl Default for ScreenReader {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(feature = "screen_reader")]
impl Default for ScreenReader {
    fn default() -> Self {
        let tts = match tts::TTS::default() {
            Ok(screen_reader) => {
                eprintln!("Initialized screen reader.");
                Some(screen_reader)
            }
            Err(err) => {
                eprintln!("Failed to load screen reader: {}", err);
                None
            }
        };
        Self { tts }
    }
}

impl ScreenReader {
    #[cfg(not(feature = "screen_reader"))]
    pub fn speak(&mut self, _text: &str) {}

    #[cfg(feature = "screen_reader")]
    pub fn speak(&mut self, text: &str) {
        if let Some(tts) = &mut self.tts {
            eprintln!("Speaking: {:?}", text);
            let interrupt = true;
            if let Err(err) = tts.speak(text, interrupt) {
                eprintln!("Failed to read: {}", err);
            }
        }
    }
}
