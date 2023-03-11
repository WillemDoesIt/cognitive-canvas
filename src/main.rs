mod terminal;
mod commands;
mod figlet;
mod myio;
mod mycrypto;
mod password;
use crate::password::{update_password_file, check_password, generate_master_password, new_key};
use crate::mycrypto::{decrypt_files, encrypt_files};
use crate::myio::myinput;
use crate::commands::get_path;
use std::fs;

fn main () {

    let password_file_path = get_path("Immutable/Shapassword.txt");

    println!("{}", password_file_path.display());

    let user_inputed_password = myinput("Enter password:\n");

    let metadata = fs::metadata(password_file_path).expect("Failed to get file metadata");
    let is_empty = metadata.len() == 0;

    if is_empty {
        update_password_file(user_inputed_password.as_str(), false)
            .unwrap_or_else(|e| eprintln!("Error updating password: {}", e));
        println!("...\npassword saved successfully.");
        println!("Here is your Master password {}", generate_master_password());
    }else {
        let pass:bool = check_password(user_inputed_password.as_str());
        match pass {
            true => println!("\x1B[2A\x1B[0GCorrect password."),
            false => {
                println!("Wrong password.");
                return;
            },
        }
    }

    let key = new_key("my password");
    match decrypt_files("documents", &key.as_str()) {
        Ok(_) => println!("Decrypt files successfully."),
        Err(err) => println!("error: {}", err)
    }

    terminal::main();

    match encrypt_files(get_path("documents").to_str().unwrap(), &key.as_str()){
        Ok(_) => println!("Encrypted files successfully."),
        Err(err) => println!("error: {}", err)
    }
}