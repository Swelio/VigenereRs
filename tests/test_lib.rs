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
