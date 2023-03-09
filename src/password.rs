use std::fs::File;
use std::fs;
use std::io::Write;
use sha2::{Sha256, Digest};
use rand::Rng;

pub fn update_password_file(password: &str, is_master: bool) -> Result<(), std::io::Error> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hashed_bytes = hasher.finalize();
    let mut file = {
        if is_master {
            File::create(r"Immutable\SHAmasterpassword.txt")?
        } else {
            File::create(r"Immutable\SHApassword.txt")?
        }
    };
    file.write_all(format!("{:x}", hashed_bytes).as_bytes())?;
    Ok(())
}

pub fn check_password(password: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hashed_bytes = hasher.finalize();

    let sha_password = fs::read_to_string(&r"Immutable\SHApassword.txt")
        .expect("Error reading file");

    let sha_masterpassword = fs::read_to_string(&r"Immutable\SHAmasterpassword.txt")
        .expect("Error reading file");

    sha_password.as_bytes() == format!("{:x}", hashed_bytes).as_bytes() || sha_masterpassword.as_bytes() == format!("{:x}", hashed_bytes).as_bytes()
}

pub fn get_key() -> String {
    let sha_password = fs::read_to_string(&r"Immutable\SHApassword.txt")
        .expect("Error reading file");
    let len = sha_password.len();
    sha_password[..len/2].to_owned()
}

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