// https://github.com/TheAlgorithms/Rust/blob/master/src/ciphers/caesar.rs

//! Caesar Cipher
//! Based on cipher_crypt::caesar
//!
//! # Algorithm
//!
//! Rotate each ascii character by shift. The most basic example is ROT 13, which rotates 'a' to
//! 'n'. This implementation does not rotate unicode characters.

/// Caesar cipher to rotate cipher text by shift and return an owned String.
pub fn caesar(cipher: &str, shift: u8) -> String {
  cipher
      .chars()
      .map(|c| {
          if c.is_ascii_alphabetic() {
              let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
              // modulo the distance to keep character range
              (first + (c as u8 + shift - first) % 26) as char
          } else {
              c
          }
      })
      .collect()
}

fn main() {
    println!("caesar(attack at dawn, 5)={}", caesar("attack at dawn", 5));
}
