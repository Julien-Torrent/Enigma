use crate::utils;

/// Enigma rotor, turns on each character input
/// https://en.wikipedia.org/wiki/Enigma_machine#Rotors
#[allow(dead_code)]
pub struct Rotor {
    forward_wiring: Vec<char>,
    backward_wiring: Vec<char>,
    noch_position: usize,
    current_rotor_position: usize,

    // Key setting is never read, because the machine cannot be reset
    // Used when the machine reset, returns the rotor to the key position
    key_setting: usize,
    ring_setting: usize,
}

impl Rotor {
    /// Create a new Rotor given the mapping, notch, key & ring
    pub fn new(mapping: &str, notches: usize, key: char, ring: char) -> Rotor {
        // Create the mapping vector given the input mapping
        let mapping: Vec<char> = mapping.chars().collect();

        // Create the inverse mapping
        let mut inverse = vec!['A'; 26];
        for (i, &c) in mapping.iter().enumerate() {
            inverse[utils::char_to_usize(c)] = utils::usize_to_char(i);
        }

        return Rotor {
            forward_wiring: mapping,
            backward_wiring: inverse,
            noch_position: notches,
            current_rotor_position: utils::char_to_usize(key),
            key_setting: utils::char_to_usize(key),
            ring_setting: utils::char_to_usize(ring),
        };
    }

    /// returns the substitution for the current postion
    /// choosen in the mapping (forward / backward)
    fn map(&self, c: char, mapping: &[char]) -> char {
        let offset = 26 + self.current_rotor_position - self.ring_setting;
        let index = mapping[(utils::char_to_usize(c) + offset) % 26];
        return utils::usize_to_char(52 + utils::char_to_usize(index) - offset);
    }

    /// Returns the substitution of a given character
    /// based on the current offset of the rotor.
    pub fn substitute(&self, c: char) -> char {
        return self.map(c, &self.forward_wiring);
    }

    /// Returns the substitution of a given character when run through
    /// the rotor in reverse (on the path back from the reflector).
    pub fn invert(&self, c: char) -> char {
        return self.map(c, &self.backward_wiring);
    }

    /// Advances this rotor by one position
    pub fn advance(&mut self) {
        self.current_rotor_position = (self.current_rotor_position + 1) % 26;
    }

    /// Returns true if the rotor is currently at the notch
    pub fn at_notch(&self) -> bool {
        return self.noch_position == self.current_rotor_position;
    }
}
