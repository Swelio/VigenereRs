//! Vigenere engine to encrypt, decrypt, crack

use std::iter::Cycle;
use std::str::Bytes;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Copy, Clone)]
enum TranformationMode {
    Encryption,
    Decryption,
}

pub struct VigenereCipher<'key, 'text> {
    key_iterator: Cycle<Bytes<'key>>,
    text_iterator: Bytes<'text>,
    mode: TranformationMode,
}

pub struct VigenereWantsKey;
pub struct VigenereWantsMode<'key> {
    key_iterator: Cycle<Bytes<'key>>,
}
pub struct VigenereWantsText<'key> {
    key_iterator: Cycle<Bytes<'key>>,
    mode: TranformationMode,
}

impl<'key, 'text> VigenereCipher<'key, 'text> {
    pub fn build() -> VigenereWantsKey {
        VigenereWantsKey
    }
}

impl<'key, 'text> Iterator for VigenereCipher<'key, 'text> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let char = match self.text_iterator.next() {
            None => return None,
            Some(char) => char,
        };

        if ALPHABET.contains(char.to_ascii_uppercase() as char) {
            let key_char = self.key_iterator.next().unwrap().to_ascii_uppercase() as isize;
            let indexed_char = char.to_ascii_uppercase() as isize;

            let char_index = match self.mode {
                TranformationMode::Encryption => indexed_char + key_char,
                TranformationMode::Decryption => indexed_char - key_char,
            }
            .rem_euclid(ALPHABET.len() as isize) as usize;
            let transformed_char = ALPHABET.as_bytes()[char_index] as char;

            if char.is_ascii_uppercase() {
                Some(transformed_char.to_ascii_uppercase())
            } else {
                Some(transformed_char.to_ascii_lowercase())
            }
        } else {
            Some(char as char)
        }
    }
}

impl VigenereWantsKey {
    pub fn with_key_string(self, key: &str) -> VigenereWantsMode {
        VigenereWantsMode {
            key_iterator: key.bytes().cycle(),
        }
    }
}

impl<'key> VigenereWantsMode<'key> {
    pub fn encrypt(&self) -> VigenereWantsText<'key> {
        VigenereWantsText {
            key_iterator: self.key_iterator.clone(),
            mode: TranformationMode::Encryption,
        }
    }

    pub fn decrypt(&self) -> VigenereWantsText<'key> {
        VigenereWantsText {
            key_iterator: self.key_iterator.clone(),
            mode: TranformationMode::Decryption,
        }
    }
}

impl<'key, 'text> VigenereWantsText<'key> {
    pub fn with_text_string(self, transformed_text: &'text str) -> VigenereCipher<'key, 'text> {
        VigenereCipher {
            key_iterator: self.key_iterator,
            text_iterator: transformed_text.bytes(),
            mode: self.mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VigenereCipher;

    #[test]
    fn test_encryption() {
        let clear_text = "Hello, Friend";
        let key = "MrRobot";
        let cipher = VigenereCipher::build()
            .with_key_string(key)
            .encrypt()
            .with_text_string(clear_text);
        let cipher_text: String = cipher.collect();

        assert_eq!(cipher_text, "Tvczp, Tkuver");
    }

    #[test]
    fn test_decryption() {
        let cipher_text = "Tvczp, Tkuver";
        let key = "MrRobot";
        let cipher = VigenereCipher::build()
            .with_key_string(key)
            .decrypt()
            .with_text_string(cipher_text);
        let clear_text: String = cipher.collect();

        assert_eq!(clear_text, "Hello, Friend");
    }

    /// Test full cipher pipeline: encryption then decryption
    #[test]
    fn test_full_cipher() {
        let original_text = "Hello, Friend";
        let key = "MrRobot";
        let cipher = VigenereCipher::build().with_key_string(key);

        let encrypted_text: String = cipher.encrypt().with_text_string(original_text).collect();
        let decrypted_text: String = cipher.decrypt().with_text_string(&encrypted_text).collect();

        assert_eq!(original_text, decrypted_text);
    }
}
