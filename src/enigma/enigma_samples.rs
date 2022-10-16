// Simply a collection of samples for the enigma machine
// Rotors

use std::collections::HashMap;
use crate::char_util::char_to_i32;
use crate::enigma::Rotor;

pub fn sample_rot_i() -> Rotor {
    /*
    Rotor I implementation according to https://www.codesandciphers.org.uk/enigma/rotorspec.htm
    Relation:
    A -> E (1, 5)
    B -> K (2, 11)
    C -> M (3, 13)
    D -> F (4, 6)
    E -> L (5, 12)
    F -> G (6, 7)
    G -> D (7, 4)
    H -> Q (8, 17)
    I -> V (9, 22)
    J -> Z (10, 26)
    K -> N (11, 14)
    L -> T (12, 20)
    M -> O (13, 15)
    N -> W (14, 23)
    O -> Y (15, 25)
    P -> H (16, 8)
    Q -> X (17, 24)
    R -> U (18, 21)
    S -> S (19, 19)
    T -> P (20, 16)
    U -> A (21, 1)
    V -> I (22, 9)
    W -> B (23, 2)
    X -> R (24, 18)
    Y -> C (25, 3)
    Z -> J (26, 10)
     */
    let mut rotor = Rotor::empty();
    rotor
        .add_relation(1, 5)
        .add_relation(2, 11)
        .add_relation(3, 13)
        .add_relation(4, 6)
        .add_relation(5, 12)
        .add_relation(6, 7)
        .add_relation(7, 4)
        .add_relation(8, 17)
        .add_relation(9, 22)
        .add_relation(10, 26)
        .add_relation(11, 14)
        .add_relation(12, 20)
        .add_relation(13, 15)
        .add_relation(14, 23)
        .add_relation(15, 25)
        .add_relation(16, 8)
        .add_relation(17, 24)
        .add_relation(18, 21)
        .add_relation(19, 19)
        .add_relation(20, 16)
        .add_relation(21, 1)
        .add_relation(22, 9)
        .add_relation(23, 2)
        .add_relation(24, 18)
        .add_relation(25, 3)
        .add_relation(26, 10);
    rotor.step = char_to_i32('R');
    rotor
}

pub fn sample_rot_ii() -> Rotor {
    /*
    Rotor II implementation according to https://www.codesandciphers.org.uk/enigma/rotorspec.htm
    Relation:
    A -> A (1, 1)
    B -> J (2, 10)
    C -> D (3, 4)
    D -> K (4, 11)
    E -> S (5, 19)
    F -> I (6, 9)
    G -> R (7, 18)
    H -> U (8, 21)
    I -> X (9, 24)
    J -> B (10, 2)
    K -> L (11, 12)
    L -> H (12, 8)
    M -> W (13, 23)
    N -> T (14, 20)
    O -> M (15, 13)
    P -> C (16, 3)
    Q -> Q (17, 17)
    R -> G (18, 7)
    S -> Z (19, 26)
    T -> N (20, 14)
    U -> P (21, 16)
    V -> Y (22, 25)
    W -> F (23, 6)
    X -> V (24, 22)
    Y -> O (25, 15)
    Z -> E (26, 5)
     */
    let mut rotor = Rotor::empty();
    rotor
        .add_relation(1, 1)
        .add_relation(2, 10)
        .add_relation(3, 4)
        .add_relation(4, 11)
        .add_relation(5, 19)
        .add_relation(6, 9)
        .add_relation(7, 18)
        .add_relation(8, 21)
        .add_relation(9, 24)
        .add_relation(10, 2)
        .add_relation(11, 12)
        .add_relation(12, 8)
        .add_relation(13, 23)
        .add_relation(14, 20)
        .add_relation(15, 13)
        .add_relation(16, 3)
        .add_relation(17, 17)
        .add_relation(18, 7)
        .add_relation(19, 26)
        .add_relation(20, 14)
        .add_relation(21, 16)
        .add_relation(22, 25)
        .add_relation(23, 6)
        .add_relation(24, 22)
        .add_relation(25, 15)
        .add_relation(26, 5);
    rotor.step = char_to_i32('F');
    rotor
}

pub fn sample_rot_iii() -> Rotor {
    /*
    Rotor III implementation according to https://www.codesandciphers.org.uk/enigma/rotorspec.htm
    Relation:
    A -> B (1, 2)
    B -> D (2, 4)
    C -> F (3, 6)
    D -> H (4, 8)
    E -> J (5, 10)
    F -> L (6, 12)
    G -> C (7, 3)
    H -> P (8, 16)
    I -> R (9, 18)
    J -> T (10, 20)
    K -> X (11, 24)
    L -> V (12, 22)
    M -> Z (13, 26)
    N -> N (14, 14)
    O -> Y (15, 25)
    P -> E (16, 5)
    Q -> I (17, 9)
    R -> W (18, 23)
    S -> G (19, 7)
    T -> A (20, 1)
    U -> K (21, 11)
    V -> M (22, 13)
    W -> U (23, 21)
    X -> S (24, 19)
    Y -> Q (25, 17)
    Z -> O (26, 15)
     */
    let mut rotor = Rotor::empty();
    rotor
        .add_relation(1, 2)
        .add_relation(2, 4)
        .add_relation(3, 6)
        .add_relation(4, 8)
        .add_relation(5, 10)
        .add_relation(6, 12)
        .add_relation(7, 3)
        .add_relation(8, 16)
        .add_relation(9, 18)
        .add_relation(10, 20)
        .add_relation(11, 24)
        .add_relation(12, 22)
        .add_relation(13, 26)
        .add_relation(14, 14)
        .add_relation(15, 25)
        .add_relation(16, 5)
        .add_relation(17, 9)
        .add_relation(18, 23)
        .add_relation(19, 7)
        .add_relation(20, 1)
        .add_relation(21, 11)
        .add_relation(22, 13)
        .add_relation(23, 21)
        .add_relation(24, 19)
        .add_relation(25, 17)
        .add_relation(26, 15);
    rotor.step = char_to_i32('W');
    rotor
}

pub fn sample_reflector_b() -> HashMap<i32, i32> {
    /*
    Reflector B implementation according to https://www.codesandciphers.org.uk/enigma/rotorspec.htm
    Relation:
    A -> Y (1, 25)
    B -> R (2, 18)
    C -> U (3, 21)
    D -> H (4, 8)
    E -> Q (5, 17)
    F -> S (6, 19)
    G -> L (7, 12)
    H -> D (8, 4)
    I -> P (9, 16)
    J -> X (10, 24)
    K -> N (11, 14)
    L -> G (12, 7)
    M -> O (13, 15)
    N -> K (14, 11)
    O -> M (15, 13)
    P -> I (16, 9)
    Q -> E (17, 5)
    R -> B (18, 2)
    S -> F (19, 6)
    T -> Z (20, 26)
    U -> C (21, 3)
    V -> W (22, 23)
    W -> V (23, 22)
    X -> J (24, 10)
    Y -> A (25, 1)
    Z -> T (26, 20)
     */
    let mut reflector = HashMap::new();
    reflector.insert(1, 25);
    reflector.insert(2, 18);
    reflector.insert(3, 21);
    reflector.insert(4, 8);
    reflector.insert(5, 17);
    reflector.insert(6, 19);
    reflector.insert(7, 12);
    reflector.insert(8, 4);
    reflector.insert(9, 16);
    reflector.insert(10, 24);
    reflector.insert(11, 14);
    reflector.insert(12, 7);
    reflector.insert(13, 15);
    reflector.insert(14, 11);
    reflector.insert(15, 13);
    reflector.insert(16, 9);
    reflector.insert(17, 5);
    reflector.insert(18, 2);
    reflector.insert(19, 6);
    reflector.insert(20, 26);
    reflector.insert(21, 3);
    reflector.insert(22, 23);
    reflector.insert(23, 22);
    reflector.insert(24, 10);
    reflector.insert(25, 1);
    reflector.insert(26, 20);
    reflector
}