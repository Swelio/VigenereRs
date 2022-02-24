# VigenereRs

[![GitHub](https://img.shields.io/github/license/Swelio/VigenereRs)](LICENSE.md)
[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/Swelio/VigenereRs/MainBuildTest/main)](https://github.com/Swelio/VigenereRs/actions/workflows/main.yml)
[![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Swelio/VigenereRs/main)](https://github.com/Swelio/VigenereRs/commits/main)
![Maintenance](https://img.shields.io/maintenance/no/2022)

Toy program to encrypt, decrypt, and crack classic Vigenere cipher (english only).

Take note this cipher is case-insensitive.

## Usage

````shell
vigenere 0.1.0
Toy program to encrypt, decrypt, and crack classic Vigenere cipher (english only).

USAGE:
    vigenere.exe <--encrypt|--decrypt> <--key <KEY>|--key-file <KEY_FILE>> <--text <TEXT>|--text-file <TEXT_FILE>>

OPTIONS:
    -D, --decrypt                  Decrypt mode
    -E, --encrypt                  Encrypt mode
    -h, --help                     Print help information
    -k, --key <KEY>                Key as string
    -K, --key-file <KEY_FILE>      Path to key file
    -t, --text <TEXT>              Text to process, as string
    -T, --text-file <TEXT_FILE>    Path of text to process
    -V, --version                  Print version information
````

## Features

- [x] Encryption/decryption with key from command line
- [x] Encryption/decryption with text from command line
- [ ] Encryption/decryption with key from file
- [ ] Encryption/decryption with text from file
- [ ] Cracking of english text