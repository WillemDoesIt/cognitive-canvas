mod terminal;
mod commands;
mod figlet;
mod myio;
mod mycrypto;
mod password;
use crate::figlet::fig_header;
use crate::password::{update_password_file, check_password, generate_master_password, get_key};
use crate::mycrypto::decrypt_files;
use crate::myio::myinput;
use std::fs;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();

    println!("{:?} {:?}",exe_path, exe_dir);

    let user_inputed_password = myinput("Enter test password:\n");

    let file_path = r"Immutable\SHApassword.txt";
    let metadata = fs::metadata(file_path).expect("Failed to get file metadata");
    let is_empty = metadata.len() == 0;

    if is_empty {
        update_password_file(user_inputed_password.as_str(), false)
            .unwrap_or_else(|e| eprintln!("Error updating password: {}", e));
        println!("...\npassword saved successfully.");
        println!("Here is your Mastern password {}", generate_master_password());
    }else {
        let pass:bool = check_password(user_inputed_password.as_str());
        match pass {
            true => println!("Correct password."),
            false => {
                println!("Wrong password.");
                return;
            },
        }
    }

    match decrypt_files("documents", &get_key()) {
        Ok(_) => println!("decrypt files successfully"),
        Err(err) => println!("error: {}", err)
    }
    fig_header("Welcome!");
    println!("Use `/` commands to interact with the program, start with '/help' if you need\n");
    terminal::main()
}