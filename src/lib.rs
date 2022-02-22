//! Vigenere engine to encrypt, decrypt, crack

use std::iter::{Cycle, Iterator};

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Copy, Clone)]
enum ProcessingMode {
    Encryption,
    Decryption,
}

pub struct VigenereBuilder;
pub struct VigenereWantsMode<K> {
    key_iterator: Cycle<K>,
}
pub struct VigenereWantsText<K> {
    key_iterator: Cycle<K>,
    mode: ProcessingMode,
}
pub struct VigenereCipher<K, M> {
    key_iterator: Cycle<K>,
    text_iterator: M,
    mode: ProcessingMode,
}

impl VigenereBuilder {
    pub fn build() -> Self {
        Self
    }

    pub fn with_key_string(self, key: &str) -> VigenereWantsMode<std::str::Bytes> {
        VigenereWantsMode {
            key_iterator: key.bytes().cycle(),
        }
    }
}

impl<K> VigenereWantsMode<K>
where
    K: Clone + Iterator,
{
    pub fn encrypt(&self) -> VigenereWantsText<K> {
        VigenereWantsText {
            key_iterator: self.key_iterator.clone(),
            mode: ProcessingMode::Encryption,
        }
    }

    pub fn decrypt(&self) -> VigenereWantsText<K> {
        VigenereWantsText {
            key_iterator: self.key_iterator.clone(),
            mode: ProcessingMode::Decryption,
        }
    }
}

impl<K> VigenereWantsText<K> {
    pub fn with_text_string(self, processed_text: &str) -> VigenereCipher<K, std::str::Bytes>
    where
        K: Clone + Iterator,
    {
        VigenereCipher {
            key_iterator: self.key_iterator,
            text_iterator: processed_text.bytes(),
            mode: self.mode,
        }
    }
}

impl<K, M> Iterator for VigenereCipher<K, M>
where
    K: Clone + Iterator<Item = u8>,
    M: Clone + Iterator<Item = u8>,
{
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
                ProcessingMode::Encryption => indexed_char + key_char,
                ProcessingMode::Decryption => indexed_char - key_char,
            }
            .rem_euclid(ALPHABET.len() as isize) as usize;
            let processed_char = ALPHABET.as_bytes()[char_index] as char;

            if char.is_ascii_uppercase() {
                Some(processed_char.to_ascii_uppercase())
            } else {
                Some(processed_char.to_ascii_lowercase())
            }
        } else {
            Some(char as char)
        }
    }
}
