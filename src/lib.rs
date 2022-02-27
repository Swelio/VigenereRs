//! Vigenere engine to encrypt, decrypt, crack

use std::fs;
use std::io::{self, BufReader, Read};
use std::iter::{Cycle, Iterator};
use std::path::Path;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Copy, Clone)]
enum ProcessingMode {
    Encryption,
    Decryption,
}

#[derive(Clone)]
enum KeyIterator<'a> {
    StringKey(std::str::Bytes<'a>),
    FileKey(KeyFileIterator),
}

impl<'a> Iterator for KeyIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            KeyIterator::StringKey(it) => it.next(),
            KeyIterator::FileKey(it) => it.next(),
        }
    }
}

pub struct KeyFileIterator {
    key_path: String,
    key_reader: io::Bytes<BufReader<fs::File>>,
}

pub struct VigenereBuilder;
pub struct VigenereWantsMode<'key> {
    key_iterator: Cycle<KeyIterator<'key>>,
}
pub struct VigenereWantsText<'key> {
    key_iterator: Cycle<KeyIterator<'key>>,
    mode: ProcessingMode,
}
pub struct VigenereCipher<'txt, 'key> {
    key_iterator: Cycle<KeyIterator<'key>>,
    text_iterator: Box<dyn Iterator<Item = u8> + 'txt>,
    mode: ProcessingMode,
}

impl KeyFileIterator {
    fn new(path: &str) -> Self {
        let key_file = fs::File::open(path.to_string()).unwrap();
        let key_reader = BufReader::new(key_file.try_clone().unwrap()).bytes();

        Self {
            key_path: path.to_string(),
            key_reader,
        }
    }
}

impl Clone for KeyFileIterator {
    fn clone(&self) -> Self {
        KeyFileIterator::new(&self.key_path)
    }

    fn clone_from(&mut self, source: &Self) {
        let cloned_iterator = KeyFileIterator::new(&source.key_path);
        self.key_path = cloned_iterator.key_path.clone();
        self.key_reader = cloned_iterator.key_reader;
    }
}

impl Iterator for KeyFileIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.key_reader
            .next()
            .map(|result| result.expect("an error occurred while reading key file"))
    }
}

impl VigenereBuilder {
    pub fn build() -> Self {
        Self
    }

    pub fn with_key_string(self, key: &str) -> VigenereWantsMode {
        VigenereWantsMode {
            key_iterator: KeyIterator::StringKey(key.bytes()).cycle(),
        }
    }

    pub fn with_key_file(self, path: &str) -> VigenereWantsMode {
        VigenereWantsMode {
            key_iterator: KeyIterator::FileKey(KeyFileIterator::new(path)).cycle(),
        }
    }
}

impl<'key> VigenereWantsMode<'key> {
    pub fn encrypt(&self) -> VigenereWantsText<'key> {
        VigenereWantsText {
            key_iterator: self.key_iterator.clone(),
            mode: ProcessingMode::Encryption,
        }
    }

    pub fn decrypt(&self) -> VigenereWantsText<'key> {
        VigenereWantsText {
            key_iterator: self.key_iterator.clone(),
            mode: ProcessingMode::Decryption,
        }
    }
}

impl<'txt, 'key> VigenereWantsText<'key> {
    pub fn with_text_string(self, processed_text: &'txt str) -> VigenereCipher<'txt, 'key> {
        VigenereCipher::<'txt, 'key> {
            key_iterator: self.key_iterator,
            text_iterator: Box::new(processed_text.bytes()),
            mode: self.mode,
        }
    }

    pub fn with_text_file<P>(self, processed_path: P) -> VigenereCipher<'txt, 'key>
    where
        P: AsRef<Path>,
    {
        let processed_file = fs::File::open(processed_path).unwrap();
        let file_reader = BufReader::new(processed_file);

        VigenereCipher {
            key_iterator: self.key_iterator,
            text_iterator: Box::new(file_reader.bytes().map(|byte| byte.unwrap())),
            mode: self.mode,
        }
    }
}

impl<'txt, 'key> Iterator for VigenereCipher<'txt, 'key> {
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
