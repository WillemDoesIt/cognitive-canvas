use aes::Aes128;
use block_modes::{BlockMode, Ecb};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::str;

type Aes128Ecb = Ecb<Aes128, Pkcs7>;

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::commands::get_path;

/// Encrypts all `.txt` files within a specified directory using AES-128 in ECB mode.
///
/// # Parameters
///
/// * `path_str` - Directory path as a string
/// * `key` - Encryption key as a hex string
///
/// # Return Value
///
/// An `std::io::Result<()>` that represents the success or failure of the encryption operation.
///
/// # Example
///
/// ```rust
/// use crypto_lib::encrypt_files;
///
/// let key = "2b7e151628aed2a6abf7158809cf4f3c";
/// let path_str = "/home/user/sample_dir";
///
/// match encrypt_files(path_str, key) {
///     Ok(_) => println!("Encryption successful!"),
///     Err(e) => println!("Encryption failed: {}", e),
/// }
/// ```
pub fn encrypt_files(path_str: &str, key: &str) -> std::io::Result<()> {
    let iv = hex!("");

    let key_bytes = hex::decode(key).expect("Failed to decode key");

    let path_buf = PathBuf::from(path_str);
    let file_path = get_path(path_buf);

    // Get all .txt files within the specified directory
    let path = Path::new(&file_path);
    let txt_files: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_file() && path.extension().unwrap() == "txt" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Encrypt each file
    for file in txt_files {
        let mut input_file = File::open(&file)?;
        let mut input_data = Vec::new();
        input_file.read_to_end(&mut input_data)?;

        let cipher = Aes128Ecb::new_from_slices(&key_bytes, &iv).unwrap();

        let mut buffer = vec![0u8; input_data.len() * 2]; // use a buffer with a fixed size
        buffer[..input_data.len()].copy_from_slice(&input_data);

        let ciphertext = cipher.encrypt(&mut buffer, input_data.len()).unwrap();

        let mut output_file = File::create(file)?;
        output_file.write_all(&ciphertext)?;
    }

    Ok(())
}

/// Decrypts all `.txt` files within a specified directory using AES-128 in ECB mode.
///
/// # Parameters
///
/// * `path_str` - Directory path as a string
/// * `key` - Decryption key as a hex string
///
/// # Return Value
///
/// An `std::io::Result<()>` that represents the success or failure of the decryption operation.
///
/// # Example
///
/// ```rust
/// use crypto_lib::decrypt_files;
///
/// let key = "2b7e151628aed2a6abf7158809cf4f3c";
/// let path_str = "/home/user/sample_dir";
///
/// match decrypt_files(path_str, key) {
///     Ok(_) => println!("Decryption successful!"),
///     Err(e) => println!("Decryption failed: {}", e),
/// }
/// ```
pub fn decrypt_files(path_str: &str, key: &str) -> std::io::Result<()> {
    let iv = hex!("");

    let key_bytes = hex::decode(key).expect("Failed to decode key");

    let path_buf = PathBuf::from(path_str);
    let file_path = get_path(path_buf);

    // Get all .txt files within the specified directory
    let path = Path::new(&file_path);
    let txt_files: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_file() && path.extension().unwrap() == "txt" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Decrypt each file
    for file in txt_files {
        let mut input_file = File::open(&file)?;
        let mut input_data = Vec::new();
        input_file.read_to_end(&mut input_data)?;

        let cipher = Aes128Ecb::new_from_slices(&key_bytes, &iv).unwrap();

        let mut buffer = input_data.clone();

        let decrypted_ciphertext = cipher.decrypt(&mut buffer).unwrap();

        let mut output_file = File::create(file)?;
        output_file.write_all(&decrypted_ciphertext)?;
    }

    Ok(())
}