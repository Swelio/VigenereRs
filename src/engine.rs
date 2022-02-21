//! Vigenere engine to encrypt, decrypt, crack

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

enum TranformationMode {
    Encryption,
    Decryption,
}

fn transformer(transformed_text: &str, key: &str, mode: TranformationMode) -> String {
    let mut result = String::with_capacity(transformed_text.len());

    let mut key_iterator = key.chars().map(|c| c.to_ascii_uppercase() as isize).cycle();
    let text_iterator = transformed_text.chars();

    for char in text_iterator {
        if ALPHABET.contains(char.to_ascii_uppercase()) {
            let key_char = key_iterator.next().unwrap();
            let operated_char = char.to_ascii_uppercase() as isize;

            let transformed_char_index = match mode {
                TranformationMode::Encryption => operated_char + key_char,
                TranformationMode::Decryption => operated_char - key_char,
            }
            .rem_euclid(ALPHABET.len() as isize) as usize;
            let mut transformed_char = ALPHABET.as_bytes()[transformed_char_index] as char;

            if char.is_ascii_uppercase() {
                transformed_char = transformed_char.to_ascii_uppercase();
            } else {
                transformed_char = transformed_char.to_ascii_lowercase();
            }

            result.push(transformed_char);
        } else {
            result.push(char);
        }
    }

    result
}

pub fn encrypt(clear_text: &str, key: &str) -> String {
    transformer(clear_text, key, TranformationMode::Encryption)
}

pub fn decrypt(cipher_text: &str, key: &str) -> String {
    transformer(cipher_text, key, TranformationMode::Decryption)
}

#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt};

    #[test]
    fn test_encryption() {
        let clear_text = "Hello, Friend";
        let key = "MrRobot";
        let cipher_text = encrypt(clear_text, key);

        assert_eq!(cipher_text, "Tvczp, Tkuver");
    }

    #[test]
    fn test_decryption() {
        let cipher_text = "Tvczp, Tkuver";
        let key = "MrRobot";
        let clear_text = decrypt(cipher_text, key);

        assert_eq!(clear_text, "Hello, Friend");
    }

    /// Test full cipher pipeline: encryption then decryption
    #[test]
    fn test_full_cipher() {
        let original_text = "Hello, Friend";
        let key = "MrRobot";

        let encrypted_text = encrypt(original_text, key);
        let decrypted_text = decrypt(&encrypted_text, key);

        assert_eq!(original_text, decrypted_text);
    }
}
