use crate::enigma::Cypher;

mod char_util;
mod enigma;

fn main() {
    let test_string = "Shid";
    let test_string = char_util::string_to_i32(char_util::filter_string(test_string).as_str());
    let mut rotors = vec![
        enigma::enigma_samples::sample_rot_i(),
        enigma::enigma_samples::sample_rot_i(),
        enigma::enigma_samples::sample_rot_i(),
    ];
    let reflector = enigma::enigma_samples::sample_reflector_b();
    let mut enigma_1 = enigma::Cypher::from_rotors_and_reflector(
        rotors,
        reflector,
    );
    let mut enigma_2 = enigma_1.clone();

    println!("Original string: {}", char_util::i32_to_string(&test_string));
    let mut encoded_string = enigma_1.encode_num_string(&test_string);
    println!("Encoded string: {}", char_util::i32_to_string(&encoded_string));
    let mut decoded_string = enigma_2.encode_num_string(&encoded_string);
    println!("Decoded string: {}", char_util::i32_to_string(&decoded_string));
}
