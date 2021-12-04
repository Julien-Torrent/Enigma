/// Engima rotors information from Wikipeida:
/// https://en.wikipedia.org/wiki/Enigma_rotor_details#Rotor_wiring_tables
pub const ROTORS: [&str; 5] = [
    "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
    "AJDKSIRUXBLHWTMCQGZNPYFVOE",
    "BDFHJLCPRTXVZNYEIWGAKMUSQO",
    "ESOVPZJAYQUIRHXLNFTGKDCMWB",
    "VZBRGITYUPSDNHLXAWMJQOFECK",
];

/// Enigma rotors turnover positions from Wikipedia:
/// https://en.wikipedia.org/wiki/Enigma_rotor_details#Turnover_notch_positions
pub const NOTCHES: [char; 5] = [
    'Q', 'E', 'V', 'J', 'Z',
];

/// Enigma reflectors information from Wikipedia:
/// https://en.wikipedia.org/wiki/Enigma_rotor_details
pub const REFLECTORS: [&str; 3] = [
    "EJMZALYXVBWFCRQUONTSPIKHGD",
    "YRUHQSLDPXNGOKMIEBFZCWVJAT",
    "FVPJIAOYEDRZXWGCTKUQSBNMHL",
];
