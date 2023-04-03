use crate::myio::myinput;
use crate::commands::{run,get_path};
use crate::figlet::fig_header;
use crate::password::get_hash;
use std::fs;

pub fn main() { 
    fig_header("Welcome!");
    println!("Use `/` commands to interact with the program, start with '/help' if you need\n");

    // detect if documents is missing main.txt
    // create main.txt file with contents:
    // Title: Main board 

    let file_name = get_hash("main");
    let main_path = get_path(format!("mutable/{}.txt", file_name));
    if !main_path.exists() {
        println!("Main file is missing, creating it...");
        fs::write(&main_path, "Title: Main board\n").expect("Failed to create main file");
        println!("Main board is created.\n");
    }

    let file_name = get_hash("contents");
    let contents_path = get_path(format!("mutable/{}.txt", file_name));
    if !contents_path.exists() {
        println!("Table of Contents file is missing, creating it...");
        fs::write(&contents_path, "contents (can't mod)\nmain\n").expect("Failed to create main file");
        println!("Table of Contents is created.\n");
    }

    loop {

        let input = myinput("");

        if input == "/quit" {
            break;
        }                        // break if input is quit command

        let mut item:char = ' ';
        if input.len() > 0 {
            item = input                               
                .chars()
                .nth(0)
                .unwrap();
        }
        let item = item;

        if (item != '/') | (input.len() <= 0) {                 // check if command
            print!("\x1B[2A\x1B[0G\n/select main {}", &input);
            run(["/select","main",&input].to_vec());            // write to document
        }else {
            let command_list: Vec<&str> = input             // run command
                .split(" ")
                .collect();

            run(command_list);
        }           

    } 
}