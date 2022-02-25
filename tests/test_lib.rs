use std::fs;
use std::io::Read;

use vigenere::VigenereBuilder;

#[test]
fn test_string_encryption() {
    let clear_text = "Hello, Friend";
    let key = "MrRobot";
    let cipher = VigenereBuilder::build()
        .with_key_string(key)
        .encrypt()
        .with_text_string(clear_text);
    let cipher_text: String = cipher.collect();

    assert_eq!(cipher_text, "Tvczp, Tkuver");
}

#[test]
fn test_string_decryption() {
    let cipher_text = "Tvczp, Tkuver";
    let key = "MrRobot";
    let cipher = VigenereBuilder::build()
        .with_key_string(key)
        .decrypt()
        .with_text_string(cipher_text);
    let clear_text: String = cipher.collect();

    assert_eq!(clear_text, "Hello, Friend");
}

/// Test full cipher pipeline: encryption then decryption
#[test]
fn test_string_full_cipher() {
    let original_text = "Hello, Friend";
    let key = "MrRobot";
    let cipher = VigenereBuilder::build().with_key_string(key);

    let encrypted_text: String = cipher.encrypt().with_text_string(original_text).collect();
    let decrypted_text: String = cipher.decrypt().with_text_string(&encrypted_text).collect();

    assert_eq!(original_text, decrypted_text);
}

/// Test full cipher pipeline on files: encryption then decryption
#[test]
fn test_file_full_cipher() {
    let original_text_path = "tests/data/text.txt";
    let key = "MrRobot";
    let cipher = VigenereBuilder::build().with_key_string(key);
    let mut original_text = String::new();
    fs::File::open(original_text_path)
        .unwrap()
        .read_to_string(&mut original_text)
        .unwrap();

    let encrypted_text: String = cipher
        .encrypt()
        .with_text_file(original_text_path)
        .collect();
    let decrypted_text: String = cipher.decrypt().with_text_string(&encrypted_text).collect();

    assert_eq!(encrypted_text, "Tvczp, Tkuver");
    assert_eq!(original_text, decrypted_text);
}
