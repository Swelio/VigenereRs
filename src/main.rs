//! Vigenere toy program to encrypt, decrypt, and crack classic Vigenere cipher (english only).

#![deny(clippy::cargo, clippy::all)]

mod engine;

use clap::{ArgGroup, Parser};

/// Vigenere toy program to encrypt, decrypt, and crack classic Vigenere cipher (english only).
#[derive(Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help=true)]
#[clap(group(ArgGroup::new("modes").required(true)))]
#[clap(group(ArgGroup::new("keys").required(true)))]
#[clap(group(ArgGroup::new("texts").required(true)))]
struct Cli {
    /// Encrypt mode
    #[clap(
        short = 'E',
        long = "encrypt",
        group = "modes",
        takes_value = false,
        conflicts_with = "decrypt-mode"
    )]
    encrypt_mode: bool,

    /// Decrypt mode
    #[clap(short = 'D', long = "decrypt", group = "modes", takes_value = false)]
    decrypt_mode: bool,

    /// Key as string
    #[clap(short = 'k', long, group = "keys", conflicts_with = "key-file")]
    key: Option<String>,

    /// Path to key file
    #[clap(short = 'K', long, group = "keys")]
    key_file: Option<String>,

    /// Text to process, as string
    #[clap(short = 't', long, group = "texts", conflicts_with = "text-file")]
    text: Option<String>,

    /// Path of text to process
    #[clap(short = 'T', long, group = "texts")]
    text_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
}
