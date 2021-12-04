mod rotor;
mod reflector;
mod enigma;
mod plugboard;
mod utils;
mod constants;

fn main(){

    // Create all the differents rotors with a key and ring
    let rotor_1 = rotor::Rotor::new(constants::ROTORS[0], utils::char_to_usize(constants::NOTCHES[0]), 'D', 'R');
    let rotor_2 = rotor::Rotor::new(constants::ROTORS[1], utils::char_to_usize(constants::NOTCHES[1]), 'O', 'I');
    let rotor_3 = rotor::Rotor::new(constants::ROTORS[2], utils::char_to_usize(constants::NOTCHES[2]), 'G', 'N');
    let _rotor_4 = rotor::Rotor::new(constants::ROTORS[3], utils::char_to_usize(constants::NOTCHES[3]), 'G', 'G');
    let _rotor_5 = rotor::Rotor::new(constants::ROTORS[4], utils::char_to_usize(constants::NOTCHES[4]), 'Y', 'S');

    // Create all the differents reflectors
    let reflector_1 = reflector::Reflector::new(constants::REFLECTORS[0]);
    let _reflector_2 = reflector::Reflector::new(constants::REFLECTORS[1]);
    let _reflector_3 = reflector::Reflector::new(constants::REFLECTORS[2]);

    // Create a plug board with 3 wiring
    let plugboard = plugboard::Plugboard::new("AB CD VW");

    // Create an enigma machine with the prevously created components
    let mut enigma_machine = enigma::Enigma{
        left: rotor_1,
        mid: rotor_2,
        right: rotor_3,
        reflector: reflector_1,
        plugboard: plugboard,
    };

    println!("Enter the message you want to encrypt/decrypt :");

    // encrypt the message and print to the console
    // WETTERBERICHT -> YURUAYGMUXXWE
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let output = enigma_machine.encrypt(input.trim());
    println!("{}", output);
}