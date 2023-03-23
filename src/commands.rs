use crate::figlet::fig_header;
use crate::myio::myinput;
use crate::password::{generate_master_password, update_password_file};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

use std::env;
use std::path::{Path, PathBuf};

/// Returns a path to a file or directory relative to the project directory.
///
/// The function takes an argument `path` which is a reference to a path, and returns a `PathBuf` representing the same path but relative to the project directory. The project directory is determined by looking at the directory containing the current executable, and if that directory contains `target\debug`, then the project directory is two levels above it. Otherwise, the project directory is the same as the directory containing the current executable.
///
/// # Arguments
///
/// * `path` - An object that can be converted to a `Path` reference using the `AsRef` trait.
///
/// # Returns
///
/// A `PathBuf` representing the given `path` relative to the project directory.
///
/// # Panics
///
/// The function panics if it fails to get the current executable path or the executable directory.
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use crate::get_path;
///
/// let data_dir = get_path(Path::new("data"));
/// println!("The path to the data directory is {:?}", data_dir);
/// ```
/// 
/// # Remarks
/// 
/// This will be able to tell if the exe is in the main directory or in the /debug/target directory so the exe can be run regardless of location
pub fn get_path(path: impl AsRef<Path>) -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");

    let project_dir = {
        if exe_dir.to_string_lossy().contains(r"target\debug") {
            let mut project_dir = exe_dir.to_path_buf();
            project_dir.pop(); // remove the `debug` directory
            project_dir.pop(); // remove the `target` directory
            project_dir
        }else {
            exe_dir.to_path_buf()
        }
    };

    project_dir.join(path)
}

/// Gets the file path from the given command attributes or prompts the user for it.
///
/// The function takes a reference to a vector of string slices called `given_attributes` and a string `command_name`, and returns a `PathBuf` representing the file path. If the first element of `given_attributes` is an empty string, the function prompts the user for the intended file name. Otherwise, it uses the first element of `given_attributes` as the file name to construct the file path.
///
/// # Arguments
///
/// * `given_attributes` - A reference to a vector of string slices containing the command attributes.
/// * `command_name` - A string representing the name of the command being executed.
///
/// # Returns
///
/// A `PathBuf` representing the file path.
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use crate::get_path_from_attributes;
///
/// let attributes = vec!["filename.txt"];
/// let command_name = "open";
/// let file_path = get_path_from_attributes(&attributes, command_name);
/// println!("The file path is {:?}", file_path);
/// ```
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

    /// This function selects a file and allows the user to write to it.
    /// 
    /// # Arguments
    /// 
    /// * `attributes` - A vector of strings containing the attributes of the file to select.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let attributes = vec!["MyFile.txt"];
    /// select(attributes);
    /// ```
    /// 
    /// # Remarks
    /// 
    /// The function prints the title of the selected file, then reads the contents of it and prints them in the terminal. 
    /// The user can then write messages to the file, which the function adds to it's contents. 
    /// 
    /// The user can quit the file selection by typing "/quit" twice. The first time will write the quit message to the file,
    /// the second time will exit the selection.
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

    /// This function prints help.txt to the terminal.
    ///
    /// # Parameters
    ///
    /// - `attributes`: A `Vec` of `&str` representing the attributes.
    ///
    /// # Examples
    ///
    /// ```
    /// help();
    /// ```
    /// prints:
    /// ```
    ///Opening document...
    ///Document opened.
    ///_   _          _             ____                               _
    ///| | | |   ___  | |  _ __     | __ )    ___     __ _   _ __    __| |
    ///| |_| |  / _ \ | | | '_ \    |  _ \   / _ \   / _` | | '__|  / _` |
    ///|  _  | |  __/ | | | |_) |   | |_) | | (_) | | (_| | | |    | (_| |
    ///|_| |_|  \___| |_| | .__/    |____/   \___/   \__,_| |_|     \__,_|
    ///                |_|
    ///    
    ///        /select (/sel) <file name> <message>    - opens <file_name> and writes <message> in file
    ///        /new           <file name>              - creates file of <file>
    ///        /files  (/dir)                          - lists all files in directory
    ///        /delete (/del) <file name>              - deletes file from directory
    ///        /quit                                   - quits document or terminal
    ///        /clear                                  - clears terminal (not document)
    ///        /newpassword (/pass)                    - generates new master-pass / create new password
    /// ```
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

    /// Creates a new file with a given name in the document directory.
    ///
    /// # Arguments
    ///
    /// * `attributes` - A vector of strings with the name of the file to be created.
    ///
    /// # Examples
    ///
    /// ```
    /// let file_name = vec!["my_file.txt"];
    /// new(file_name);
    /// ```
    /// 
    /// Will create a new file called "my_file.txt" in the document directory.
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

    /// `files` is a function that prints the names of all files in the "documents" directory.
    /// 
    /// # Parameters
    /// 
    /// `attributes` - a `Vec` of `&str` containing the attributes of the files to be printed.
    /// 
    /// # Example
    /// 
    /// ```
    /// // given the file conatins document.txt and main.txt
    /// files();
    /// ```
    /// prints:
    /// ``` 
    ///     ____    _                        _
    ///    |  _ \  (_)  _ __    ___    ___  | |_    ___    _ __   _   _ 
    ///    | | | | | | | '__|  / _ \  / __| | __|  / _ \  | '__| | | | |
    ///    | |_| | | | | |    |  __/ | (__  | |_  | (_) | | |    | |_| |
    ///    |____/  |_| |_|     \___|  \___|  \__|  \___/  |_|     \__, |
    ///                                                            |___/ 
    ///
    ///                document.txt
    ///                main.txt
    /// ```
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

    /// Deletes a file from a given path.
    ///
    /// # Parameters
    ///
    /// * `attributes` - A vector of strings representing the path of the file to delete.
    ///
    /// # Examples
    ///
    /// ```
    /// let attributes = vec!["folder_name", "file.txt"];
    /// delete(attributes);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the file path doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the file couldn't be removed.
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

    /// Prompts the user for a new password and updates the password file.
    ///
    /// If the user chooses to generate a new master password, the master password file is
    /// updated and the new master password is printed to the console. Otherwise,
    /// the user is prompted for a new password, and the password file is updated with
    /// the new password.
    ///
    /// # Examples
    ///
    /// ```
    /// let attributes: Vec<&str> = vec![];
    /// new_password(attributes);
    /// ```
    fn new_password(_attributes: Vec<&str>) {
        let gen = myinput("Do you want to generate new master password? (y/n)\n");
        if gen == "y" {
            println!("...\npassword saved successfully.");
            println!("Here is your Master password {}\n", generate_master_password());
            return;
        }
        let new_pass = myinput("What will your new password be?\n");
        update_password_file(&new_pass, false)
            .expect("Error updating password file");
        println!("...\npassword saved successfully.\n");
    }   

    /// Clears the terminal screen.
    ///
    /// This function clears the terminal screen by sending ANSI escape codes to the
    /// console.
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
