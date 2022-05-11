// https://github.com/TheAlgorithms/Rust/blob/master/src/ciphers/vigenere.rs

//! Vigenère Cipher
//!
//! # Algorithm
//!
//! Rotate each ascii character by the offset of the corresponding key character.
//! When we reach the last key character, we start over from the first one.
//! This implementation does not rotate unicode characters.

/// Vigenère cipher to rotate plain_text text by key and return an owned String.
pub fn vigenere(plain_text: &str, key: &str) -> String {
  // Remove all unicode and non-ascii characters from key
  let key: String = key.chars().filter(|&c| c.is_ascii_alphabetic()).collect();
  key.to_ascii_lowercase();

  let key_len = key.len();
  if key_len == 0 {
      return String::from(plain_text);
  }

  let mut index = 0;

  plain_text
      .chars()
      .map(|c| {
          if c.is_ascii_alphabetic() {
              let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
              let shift = key.as_bytes()[index % key_len] - b'a';
              index = index + 1;
              // modulo the distance to keep character range
              (first + (c as u8 + shift - first) % 26) as char
          } else {
              c
          }
      })
      .collect()
}


fn main() {
  println!("vigenere(attack at dawn, key)={}", vigenere("attack at dawn", "key"));
}
