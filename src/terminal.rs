use crate::myio::myinput;
use crate::commands::run;
use crate::password::get_key;
use crate::mycrypto::encrypt_files;

pub fn main() { loop {

    let input = myinput("");

    if input == "/quit" {
        match encrypt_files("documents", &get_key()){
            Ok(_) => println!("encrypted files successfully"),
            Err(err) => println!("error: {}", err)
        }
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
        print!("\x1B[2A\x1B[0G");
        run(["/select","main.txt",&input].to_vec());            // write to document
    }else {
        let command_list: Vec<&str> = input             // run command
            .split(" ")
            .collect();

        run(command_list);
    }           

} }