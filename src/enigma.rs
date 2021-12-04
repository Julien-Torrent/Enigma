use crate::rotor;
use crate::reflector;
use crate::plugboard;


/// Represents an Enigma machine with 3 rotors, 1 reflector and 1 plugboard.
/// Information pulled from Wikipedia :
/// https://en.wikipedia.org/wiki/Enigma_machine
pub struct Enigma {
    pub left: rotor::Rotor,
    pub mid: rotor::Rotor,
    pub right: rotor::Rotor,
    pub reflector: reflector::Reflector,
    pub plugboard: plugboard::Plugboard,
}

impl Enigma {
    /// Encrypt the given message
    pub fn encrypt(&mut self, msg: &str) -> String {
        return msg.chars().map(|c| self.encrypt_char(c)).collect();
    }

    /// Update the state of the rotors and encrypt a single character 
    fn encrypt_char(&mut self, c: char) -> char {
        // Update the rotors
        self.advance();

        let c0 = self.plugboard.map(c.to_ascii_uppercase());

        // Right to left
        let c1 = self.right.substitute(c0);
        let c2 = self.mid.substitute(c1);
        let c3 = self.left.substitute(c2);

        // Reflector
        let c4 = self.reflector.reflect(c3);

        // Left to right
        let c5 = self.left.invert(c4);
        let c6 = self.mid.invert(c5);
        let c7 = self.right.invert(c6);

        // Plugboard out
        return self.plugboard.map(c7);
    }

    /// Update the position of the rotors
    fn advance(&mut self) {
        // Check for double-rotation situation
        if self.mid.at_notch() {
            self.mid.advance();
            self.left.advance();
        } else if self.right.at_notch() {
            self.mid.advance();
        }

        // Finally, advance the right rotor
        self.right.advance();
    }
}