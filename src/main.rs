//! Vigenere toy program to encrypt, decrypt, and crack classic Vigenere cipher (english only).

#![deny(clippy::cargo, clippy::all)]

mod lib;

use clap::{ArgGroup, Parser};
use lib::VigenereBuilder;

/// Vigenere toy program to encrypt, decrypt, and crack classic Vigenere cipher (english only).
#[derive(Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help=true)]
#[clap(group(ArgGroup::new("modes").required(true)))]
#[clap(group(ArgGroup::new("keys").required(true)))]
#[clap(group(ArgGroup::new("texts").required(true)))]
struct Cli {
    /// Encrypt provided text
    #[clap(
        short = 'E',
        long = "encrypt",
        group = "modes",
        takes_value = false,
        conflicts_with = "decrypt-mode",
        help_heading = Some("Modes")
    )]
    encrypt_mode: bool,

    /// Decrypt provided text
    #[clap(short = 'D', long = "decrypt", group = "modes", takes_value = false, help_heading = Some("Modes"))]
    decrypt_mode: bool,

    /// Key as string
    #[clap(short = 'k', long, group = "keys", conflicts_with = "key-file", help_heading = Some("Key options"))]
    key: Option<String>,

    /// Path to key file
    #[clap(short = 'K', long, group = "keys", help_heading = Some("Key options"))]
    key_file: Option<String>,

    /// Text to process, as string
    #[clap(short = 't', long, group = "texts", conflicts_with = "text-file", help_heading = Some("Text options"))]
    text: Option<String>,

    /// Path of text to process
    #[clap(short = 'T', long, group = "texts", help_heading = Some("Text options"))]
    text_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let cipher = VigenereBuilder::build();

    let cipher = match &cli.key {
        Some(key_file) => cipher.with_key_string(key_file),
        None => match &cli.key_file {
            Some(path) => cipher.with_key_file(path),
            None => panic!("no key provided neither from args nor from file"),
        },
    };

    let cipher = if cli.encrypt_mode {
        cipher.encrypt()
    } else if cli.decrypt_mode {
        cipher.decrypt()
    } else {
        panic!("got unknown mode, exit")
    };

    let cipher = match &cli.text {
        Some(text) => cipher.with_text_string(text),
        None => match &cli.text_file {
            Some(path) => cipher.with_text_file(path),
            None => panic!("no text provided neither from args nor from file"),
        },
    };

    for char in cipher {
        print!("{}", char);
    }
}
