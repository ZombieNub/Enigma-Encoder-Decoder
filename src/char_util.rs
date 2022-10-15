/*
This contains a number of utility functions for dealing with characters, specifically conversion between char and i32.
The general conversion is:
A -> 1
B -> 2
...
Z -> 26
This can't be done with a simple char to i32 conversion, because the ASCII values for A-Z do not start at 1.
So we will have to do some arithmetic to get the correct value.
 */

// Convert a char to an i32
pub fn char_to_i32(c: char) -> i32 {
    // Convert the char to an i32
    let mut i = c as i32;
    // Subtract 64 to get the correct value
    i -= 64;
    // Return the value
    i
}

// Convert an i32 to a char
pub fn i32_to_char(i: i32) -> char {
    // Convert the i32 to a char
    let mut c = i as u8;
    // Add 64 to get the correct value
    c += 64;
    // Return the value
    c as char
}

// If a number is below 1 or above 26, constrain it to the range 1-26 using modulo.
pub fn constrain_char(i: i32) -> i32 {
    // The modulus operator does not work due to the potential for negative numbers.
    // Thankfully, we do have rem_euclid, which does exactly what we want.
    (i - 1).rem_euclid(26) + 1
}

pub fn string_to_i32(s: &str) -> Vec<i32> {
    let mut v = Vec::new();
    for c in s.chars() {
        v.push(char_to_i32(c));
    }
    v
}

pub fn i32_to_string(v: &Vec<i32>) -> String {
    let mut s = String::new();
    for i in v {
        s.push(i32_to_char(*i));
    }
    s
}

// We need to make sure that all the characters in the string are uppercase letters (A-Z). No numbers, no punctuation, no spaces.
pub fn filter_string(s: &str) -> String {
    let mut s = s.to_uppercase();
    s.retain(|c| c.is_alphabetic());
    s
}