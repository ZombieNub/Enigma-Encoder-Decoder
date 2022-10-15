pub(crate) mod enigma_samples;

use std::collections::HashMap;
use crate::char_util::{constrain_char, i32_to_char};

/*
In order to make the cypher methods composable, we will use a general cypher struct which will contain various step methods.
That way we can just call the cypher methods in order to get the desired result.
 */

#[derive(Debug, Clone)]
pub struct Rotor {
    pub rotor: HashMap<i32, i32>,
    pub reflected_rotor: HashMap<i32, i32>,
    pub rot: i32,
    pub step: i32,
}

impl Rotor {
    // The Rotor struct presumes that the rotor dictionary is "complete", i.e. it contains all 26 letters.
    // If it doesn't, god help us.
    pub fn new(rotor: HashMap<i32, i32>, reflected_rotor: HashMap<i32, i32>, rot: i32, step: i32) -> Rotor {
        Rotor {
            rotor,
            reflected_rotor,
            rot,
            step,
        }
    }

    pub fn empty() -> Rotor {
        Rotor {
            rotor: HashMap::new(),
            reflected_rotor: HashMap::new(),
            rot: 0,
            step: 0,
        }
    }

    pub fn add_relation(&mut self, key: i32, value: i32) -> &mut Self { // Returns a mutable reference to self, so we can chain methods
        self.rotor.insert(key, value);
        self.reflected_rotor.insert(value, key);
        self
    }

    pub fn encode(&self, i: i32) -> i32 {
        let i = &constrain_char(i + self.rot);
        let i = self.rotor.get(i).expect(format!("Rotor encoding incomplete for {}", i32_to_char(*i)).as_str());
        let i = constrain_char(i - self.rot);
        //println!("After Encode: {} ({})", i32_to_char(i), i);
        i
    }

    pub fn reflected_encode(&self, i: i32) -> i32 {
        let i = &constrain_char(i + self.rot);
        let i = self.reflected_rotor.get(i).expect(format!("Reflected rotor encoding incomplete for {}", i32_to_char(*i)).as_str());
        let i = constrain_char(i - self.rot);
        //println!("After Reflected Encode: {} ({})", i32_to_char(i), i);
        i
    }

    pub fn step(&mut self) {
        self.rot += 1;
        //self.rot = constrain_char(self.rot) - 1;
    }

    pub fn should_step_next(&self) -> bool {
        self.rot == self.step - 1
    }

    pub fn reset(&mut self) {
        self.rot = 0;
    }
}

#[derive(Debug, Clone)]
pub struct Cypher {
    pub plugboard: HashMap<i32, i32>,
    pub reflected_plugboard: HashMap<i32, i32>,
    pub rotors: Vec<Rotor>,
    pub reflector: HashMap<i32, i32>,
}

impl Cypher {
    pub fn new(plugboard: HashMap<i32, i32>, reflected_plugboard: HashMap<i32, i32>, rotors: Vec<Rotor>, reflector: HashMap<i32, i32>) -> Self {
        Cypher {
            plugboard,
            reflected_plugboard,
            rotors,
            reflector,
        }
    }

    pub fn empty() -> Self {
        Cypher {
            plugboard: HashMap::new(),
            reflected_plugboard: HashMap::new(),
            rotors: Vec::new(),
            reflector: HashMap::new(),
        }
    }

    pub fn from_rotors_and_reflector(rotors: Vec<Rotor>, reflector: HashMap<i32, i32>) -> Self {
        Cypher {
            plugboard: HashMap::new(),
            reflected_plugboard: HashMap::new(),
            rotors,
            reflector,
        }
    }

    pub fn encode(&self, i: i32) -> i32 {
        // Bugger me, this is broken. Lets println everywhere.
        //println!("Input: {} ({})", i32_to_char(i), i);
        // First we encode through the plugboard. If the plugboard doesn't contain the letter, we just return the letter.
        let i = *self.plugboard.get(&i).unwrap_or(&i);
        //println!("After Plugboard: {} ({})", i32_to_char(i), i);
        // Then we encode through the rotors.
        // Println! will occur in the rotor encode rather than here
        let i = self.rotors.iter().fold(i, |i, rotor| rotor.encode(i));
        // Then we encode through the reflector.
        let i = *self.reflector.get(&i).unwrap_or(&i);
        //println!("After Reflector: {} ({})", i32_to_char(i), i);
        // Then we encode through the rotors in reverse.
        let i = self.rotors.iter().rev().fold(i, |i, rotor| rotor.reflected_encode(i));
        // Then we encode through the plugboard again.
        let i = *self.reflected_plugboard.get(&i).unwrap_or(&i);
        //println!("After Reflected Plugboard: {} ({})\n", i32_to_char(i), i);
        // Return the result.
        i
    }

    pub fn step(&mut self) -> &mut Self {
        // Stepping a rotor needs to happen when the rotor is at a specific position.
        // What that position is depends on the rotor.
        // For instance, the I rotor steps at R, the II rotor steps at F, and the III rotor steps at W.
        // This can make things...curious.
        for rotor in self.rotors.iter_mut() {
            rotor.step();
            if !rotor.should_step_next() {
                break;
            }
        }
        // For debugging purposes, we will print the current rotor positions.
        //println!("Current rotor positions: {:?}", self.rotors.iter().map(|rotor| i32_to_char(rotor.rot + 1)).collect::<Vec<char>>());
        self
    }

    pub fn add_plug(&mut self, key: i32, value: i32) -> &mut Self {
        self.plugboard.insert(key, value);
        self.reflected_plugboard.insert(value, key);
        self
    }

    pub fn add_rotor(&mut self, rotor: Rotor) -> &mut Self {
        self.rotors.push(rotor);
        self
    }

    pub fn set_reflector(&mut self, reflector: HashMap<i32, i32>) -> &mut Self {
        self.reflector = reflector;
        self
    }

    pub fn step_and_encode(&mut self, i: i32) -> i32 {
        // Due to the design of the enigma, the step happens before the encoding.
        self.step().encode(i)
    }

    pub fn encode_num_string(&mut self, num_str: &Vec<i32>) -> Vec<i32> {
        num_str.iter().map(|i| self.step_and_encode(*i)).collect()
    }

    pub fn reset(&mut self) {
        for rotor in self.rotors.iter_mut() {
            rotor.reset();
        }
    }
}