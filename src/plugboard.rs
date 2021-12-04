use crate::utils;

/// Enigma reflector, connects swaps a character with another one
/// Information pulled from Wikipedia:
/// https://en.wikipedia.org/wiki/Enigma_machine#Plugboard
pub struct Plugboard {
    wiring: Vec<char>
}

impl Plugboard {
    /// Create a new plugboard given the pairs
    /// pairs is a sring of two chars separated by whitespace
    /// Ex: AJ BK, will swap (A with J) and (B with K)
    pub fn new(pairs: &str) -> Plugboard {
        // create the wiring A-Z (No substitutions)
        let mut wiring: Vec<char> = ('A'..='Z').collect();

        for pair in pairs.split_whitespace() {
            let pair: Vec<char> = pair.chars().collect();
            let a = pair[0];
            let b = pair[1];

            // swap the characters of the pair
            wiring[utils::char_to_usize(a)] = b;
            wiring[utils::char_to_usize(b)] = a;
        }

        return Plugboard { 
            wiring 
        };
    }

    /// Return the char mapped to the input
    pub fn map(&self, c: char) -> char {
        return self.wiring[utils::char_to_usize(c)];
    }
}