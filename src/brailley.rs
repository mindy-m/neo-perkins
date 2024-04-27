struct Brailley {
    /// Dots that are present in this cell of the brailler
    dots_present: u8,
    /// Dot keys that are *currently being held down*
    dots_held: u8,
}

impl Brailley {
    /// Create a new Brailley in the initial state (no keys held, no dots
    /// present)
    pub fn new() -> Brailley {
        return Brailley {
            dots_present: 0,
            dots_held: 0,
        };
    }
    /// Respond to a dot key being PRESSED. Panic if the dot is out of
    /// range (greater than or equal to 6)
    pub fn press_dot(&mut self, dot: usize) {
        self.dots_present |= (1 << dot);
        self.dots_held |= (1 << dot);
    }
    /// Respond to a dot key being RELEASED. Panic if the dot is out of range
    /// (greater than or equal to 6). If this was the last dot being held down,
    /// return `Some((the braille character, the Latin character))`
    pub fn release_dot(&mut self, dot: usize) -> Option<(char, char)> {
        todo!()
    }
    /// Respond to the space key being pressed. Return Ok if a space should be
    /// outputted, or Err if a beep should be beeped.
    pub fn press_space(&mut self) -> Result<(), ()> {
        if self.dots_held != 0 {
            Err(())
        } else {
            Ok(())
        }
    }
    /// Respond to the enter key being pressed. Return Ok if a space should be
    /// outputted, or Err if a beep should be beeped.
    pub fn press_enter(&mut self) -> Result<(), ()> {
        if self.dots_held != 0 {
            Err(())
        } else {
            Ok(())
        }
    }
}

//TODO: Remind Solra to tell the story of open doc??
