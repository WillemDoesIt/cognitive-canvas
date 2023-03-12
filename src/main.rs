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

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use ctrlc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");


    let key = new_key("my password"); // need this outside of the loop scope because its used after to encrypt again
    while running.load(Ordering::SeqCst) {
        // Your program's main loop here...
        let password_file_path = get_path("Immutable/Shapassword.txt");

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

        match decrypt_files("documents", &key.as_str()) {
            Ok(_) => println!("Decrypt files successfully."),
            Err(err) => println!("error: {}", err)
        }

        terminal::main();

        thread::sleep(Duration::from_millis(100));
        break;
    }

    // Run your cleanup code here.
    match encrypt_files(get_path("documents").to_str().unwrap(), &key.as_str()){
        Ok(_) => println!("Encrypted files successfully."),
        Err(err) => println!("error: {}", err)
    }
}