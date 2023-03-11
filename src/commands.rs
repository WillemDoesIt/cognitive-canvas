use crate::figlet::fig_header;
use crate::myio::myinput;
use crate::password::{generate_master_password, update_password_file};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

use std::env;
use std::path::{Path, PathBuf};

pub fn get_path(path: impl AsRef<Path>) -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");

       let project_dir = exe_dir.to_path_buf();
//add mut ^ there
//  project_dir.pop(); // remove the `debug` or `release` directory
//  project_dir.pop(); // remove the `target` directory

    project_dir.join(path)
}

fn get_path_from_attributes(given_attributes: &Vec<&str>, command_name:&str) -> PathBuf {
    // if attributes is empty
    let file_path;
    if given_attributes[0] == "" {
        let input = myinput("What is the file name\n");

        
        println!("\x1b[2K\x1b[2A\x1b[2K\x1b[2A");
        println!("{command_name} {input}\n");

        file_path = get_path(format!("{}/{}", "documents", input));
    }else {
        // get <file name> from attributes
        println!("");
        file_path = get_path(format!("{}/{}", "documents", given_attributes[0].to_string()));  
    };

    return file_path.to_owned();
}

pub fn run(input: Vec<&str>) { 

    fn select (attributes : Vec<&str>) {
        let file_path = get_path_from_attributes(&attributes, "/select");
            
        println!("Opening document...");
        let contents = fs::read_to_string(&*file_path)
            .expect("Error reading file");
        println!("Document opened.");

        let title: String = contents
            .lines()
            .next()
            .unwrap()
            .chars()
            .skip(7)
            .collect();

        let contents: String = contents
            .lines()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\n");

        
        fig_header(&title);
        println!(
           "{contents}\n"
        );

        let mut temp_message = {
            if attributes.len() > 1 {
                attributes[1]
            }else {
                ""
            }
        };
        loop {
            // get next input
            let message:String = {
                if &temp_message.len() > &0 {temp_message.to_owned()} else {
                    myinput("")
                }
            };

            // quit if told
            if message == "/quit" {
                println!("\nExiting document...");
                // encrypt selected file
                // thing
                println!("Existed document.");
                println!("write quit one more time to exit terminal\n");
                break;
            }

            // add time and date to message
            use chrono::{Local, DateTime};
            let now: DateTime<Local> = Local::now();
            let formatted_date_time = format!("{} ", now.format("%Y-%m-%d %H:%M:%S"));
            let final_message = formatted_date_time + &message;
            
            // add message to selected file
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(&*file_path)
                .expect("Could not open file");

            file.write_all(&format!("{}\n", final_message).as_bytes())
                .expect("Could not write to file");

            // rewrite in terminal
            if temp_message == "" {
                print!("\x1b[2A");
                println!("{final_message}\n\x1b[K");
            } else {
                print!("\x1b[1A");
                println!("{final_message}\n\x1b[K");
                temp_message = ""
            }
        }
    }

    fn help(_attributes : Vec<&str>) {
        // print help.txt to terminal

        println!("Opening document...");
        let contents = fs::read_to_string(get_path("Immutable/help.txt"))
            .expect("Error reading file");
        println!("Document opened.");

        let title: String = contents
            .lines()
            .next()
            .unwrap()
            .chars()
            .skip(7)
            .collect();

        let contents: String = contents
            .lines()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\n");

        fig_header(&title);
        println!(
           "{contents}\n"
        );

    }

    fn new (attributes : Vec<&str>) {
        let file_path = get_path_from_attributes(&attributes, "/new");

        // create a new file of name <file name> in document directory
        println!("Creating file...");
        if Path::new(&file_path).exists() {
            println!("File already exists.\n");
            return;
        }

        let mut file = File::create(&*file_path)
            .expect("Error creating file"); 
        println!("File created.");

        let title:String = format!("Title: {} \n", myinput("What do you want the title to be?\n"));

        // write the title to file
        file.write_all(title.as_bytes())
           .expect("Error writing title to file");
        
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        select(file_name
           .split(" ")
           .collect()
        );
    }

    fn files (_attributes : Vec<&str>) {

        // print all files in the documents directory
        let dir = "documents";

        // Get a list of all the files in the directory
        println!("Reading files...");
        let files = fs::read_dir(dir).unwrap();
        println!("Files read.");

        fig_header("Directory");

        // Iterate over the list of files and print their names
        for file in files {
            let file = file.unwrap();
            let file_type = file.file_type().unwrap();
            
            if file_type.is_file() {
                let file_name = file.file_name().into_string().unwrap();
                println!("            {}", file_name);
            }
        }
        println!("");
    }

    fn delete (attributes: Vec<&str>) {
        let file_path = get_path_from_attributes(&attributes, "/delete");

        println!("Deleting...");
        if !Path::new(&file_path).exists() {
            println!("File path does not exist.\n");
            return;
        }

        fs::remove_file(file_path)
            .expect("Couldn't remove file");
        println!("Deleted file.\n");
    }

    fn new_password(_attributes: Vec<&str>) {
        //TODO: do you want to generate new master password?
        let gen = myinput("Do you want to generate new master password? (y/n)\n");
        if gen == "y" {
            println!("...\npassword saved successfully.");
            println!("Here is your Master password {}\n", generate_master_password());
            return;
        }
        //TODO: what will your new password be
        let new_pass = myinput("What will your new password be?\n");
        update_password_file(&new_pass, false)
            .expect("Error updating password file");
        println!("...\npassword saved successfully.\n");
    }   

    fn clear (_attributes: Vec<&str>) {
        print!("{}[2J", 27 as char);
        print!("{}[H", 27 as char);
        std::io::stdout().flush().unwrap();
    }

    let command = input[0];

    let mut attributes: Vec<&str> = input;
    attributes.remove(0);

    if attributes.is_empty() {
        attributes.push("");
    }

    let commands =  {
        let mut h: HashMap<String, fn(Vec<&str>)> = HashMap::new();

        h.insert(String::from("/select"), select);
        h.insert(String::from("/sel"), select);
        
        h.insert(String::from("/files"), files);
        h.insert(String::from("/dir"), files);

        h.insert(String::from("/delete"), delete);
        h.insert(String::from("/del"), delete);

        h.insert(String::from("/clear"), clear);
        h.insert(String::from("/help"), help);
        h.insert(String::from("/new"), new);

        h.insert(String::from("/newpassword"), new_password);
        h.insert(String::from("/pass"), new_password);

        h // returns h
    };

    if commands.contains_key(command) {
        commands[command](attributes);
    }else if commands.contains_key(&format!("/{}", command)) {
        commands[&format!("/{}", command)](attributes);
    }else {
        println!("\ninvalid command, use `/help` to list commands.\n")
    }
}
