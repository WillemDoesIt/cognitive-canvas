use std::fs::File;
use std::fs;
use std::io::Write;
use sha2::{Sha256, Digest};
use rand::Rng;
use crate::commands::get_path;

pub fn get_hash(input_str: &str) -> String {
    // Create a Sha256 object
    let mut hasher = Sha256::new();

    // Hash the input string
    hasher.update(input_str.as_bytes());

    // Get the hash result as a byte array and return it
    let hashed_bytes = hasher.finalize();

    let hash = format!("{:x}", hashed_bytes);
    hash
}



/// Updates the SHA256 password file with the provided `password`.
/// 
/// # Arguments
/// 
/// * `password` - The password string to be hashed and written to the file.
/// * `is_master` - Boolean value indicating whether the file is a master password file.
/// 
/// # Returns
/// 
/// A `Result<(), std::io::Error>` which will be `Ok(())` on success or `Err(e)` on failure, where `e` is an [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html).
/// 
/// # Examples
/// 
/// ```
/// use std::io;
/// use crypto_hash::Sha256;
/// 
/// let result = update_password_file("mypassword", true);
/// assert!(result.is_ok());
/// ```
pub fn update_password_file(password: &str, is_master: bool) -> Result<(), std::io::Error> {
//  let mut hasher = Sha256::new();
//  hasher.update(password.as_bytes());
//  let hashed_bytes = hasher.finalize();
//  println!("The type of my_var is: {}", std::any::type_name::<typeof(hashed_bytes)>());
    let mut file = {
        if is_master {
            File::create(get_path("Immutable/SHAmasterpassword.txt"))?
        } else {
            File::create(get_path("Immutable/SHApassword.txt"))?
        }
    };
//  file.write_all(format!("{:x}", hashed_bytes).as_bytes())?;
    file.write_all(get_hash(password).as_bytes())?;
    Ok(())
}

/// Checks whether the given password matches the stored passwords.
///
/// # Arguments
///
/// * `password` - The password to check.
///
/// # Returns
///
/// `true` if the password matches the stored passwords, `false` otherwise.
///
/// # Examples
///
/// ```
/// assert!(check_password("some_password"));
/// ```
pub fn check_password(password: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hashed_bytes = hasher.finalize();

    let sha_password = fs::read_to_string(get_path("Immutable/SHApassword.txt"))
        .expect("Error reading file");

    let sha_masterpassword = fs::read_to_string(get_path("Immutable/SHAmasterpassword.txt"))
        .expect("Error reading file");

    sha_password.as_bytes() == format!("{:x}", hashed_bytes).as_bytes() || sha_masterpassword.as_bytes() == format!("{:x}", hashed_bytes).as_bytes()
}

/// Generates an SHA256 cryptographic key from the provided password, concatenated with a salt value.
///
/// # Arguments
///
/// * `password` - A string slice containing the password to generate the key from
///
/// # Examples
///
/// ```
/// let key = new_key("my_password");
/// ```
///
/// # Returns
///
/// A `String` containing the generated SHA256 key.
pub fn new_key(password: &str) -> String {
    let salted_password = format!("{}{}", password, "my salt");
     
    let mut hasher = Sha256::new();
    hasher.update(salted_password.as_bytes());
    let two_big:String = format!("{:x}", hasher.finalize());

    let len = two_big.len();
    let key:String = two_big[..len/2].to_owned();

    key
}

/// Generates a new, random master password and stores it in the password file.
///
/// # Arguments
///
/// * None
///
/// # Returns
///
/// * `String` - The master password that was generated.
///
/// # Examples
///
/// ```
/// let master_password = generate_master_password();
/// println!("Generated master password: {}", master_password);
/// ```
pub fn generate_master_password() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const PASSWORD_LEN: usize = 32;

    let mut rng = rand::thread_rng();
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    update_password_file(password.as_str(), true)
        .unwrap_or_else(|e| eprintln!("Error updating password: {}", e));

    password
}