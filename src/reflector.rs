use crate::utils;

/// Enigma reflector, connects chars in pairs and send the output back through the rotors
/// Information pulled from Wikipedia:
/// https://en.wikipedia.org/wiki/Enigma_machine#Reflector
pub struct Reflector{
    wiring: Vec<char>
}

impl Reflector {
    /// create a new Reflector given the wiring sequence
    /// [0 - 26] -> char at 0 is mapped to A, 26 mapped to Z
    pub fn new(wriring: &str) -> Reflector {
        return Reflector { 
            wiring: wriring.chars().collect(),
        }
    }

    /// Return the char mapped to the input
    pub fn reflect(&self, c: char) -> char {
        return self.wiring[utils::char_to_usize(c)];
    }
}