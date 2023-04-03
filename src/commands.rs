use crate::figlet::fig_header;
use crate::myio::myinput;
use crate::password::{generate_master_password, update_password_file, get_hash};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use std::env;
use std::path::{Path, PathBuf};

/// Reads the names of the unhashed files from the mutable directory, the names
/// are stored in contents.txt which are encrypted when program is not in use.
///
/// The file path to contents.txt is obtained using the `get_path` function, 
/// which returns the path of the file relative to the project root directory.
///
/// # Panics
///
/// This function will panic if the file cannot be opened or read.
///
/// # Examples
///
/// ```
/// // list is empty
/// let mut new_list = get_contents();
///
/// new_list.push("this".to_owned());
///
/// assert_eq!(new_list, ["this".to_owned()]);
/// ```
fn get_contents() -> Vec<String> {
    let file_name = get_path("mutable/d1b2a59fbea7e20077af9f91b27e95e865061b270be03ff539ab3b73587882e8.txt");
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut list = Vec::new();

    for line in reader.lines() {
        let item = line.unwrap();
        list.push(item);
    }

    list
}


/// lets you update the list of the names of the files in the mutable directory
///
/// The file path is obtained using the `get_path` function, which returns the
/// path of the file relative to the project root directory.
///
/// # Panics
///
/// This function will panic if the file cannot be opened or read.
/// 
/// # Arguments
/// 
/// * `list` - &Vec<String> to set contents.txt to.
///
/// # Examples
///
/// ```
/// let list = vec!["apple"]
///     .iter()
///     .map(|&s| s.to_string())
///     .collect();
/// 
/// set_contents(&list);
/// assert_eq!(list, get_contents());
///
/// // you can add to the list and update it too: 
/// new_list.push("this".to_owned());
///
/// set_contents(&new_list);
/// assert_eq!(new_list, get_contents());
///
/// ```
fn set_contents(list: &Vec<String>) {
    let file_name = get_path("mutable/d1b2a59fbea7e20077af9f91b27e95e865061b270be03ff539ab3b73587882e8.txt");
    let file = File::create(file_name).unwrap();

    file.set_len(0).unwrap(); // truncate the file to zero bytes
    let mut writer = BufWriter::new(file);

    for item in list {
        writer.write_all(item.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
    }
}

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
    // read the contents.json


    // if attributes is empty
    let file_path;
    if given_attributes[0] == "" {
        let input = myinput("What is the file name\n");

        
        println!("\x1b[2K\x1b[2A\x1b[2K\x1b[2A");
        println!("{command_name} {input}\n");

        file_path = get_path(format!("{}/{}", "mutable", input));
    }else {
        // get <file name> from attributes
        println!("");
        let file_name = get_hash(given_attributes[0]);
        file_path = get_path(format!("{}/{}.txt", "mutable", file_name)); 
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
        if attributes[0] == "contents" {
            println!("Cannot select contents file -> access directory from /files\n");
            return;
        }      

        let file_path = get_path_from_attributes(&attributes, "/select");
            
        println!("Opening document {}", format!(r#""{}""#, attributes[0]));
        
        let contents = match fs::read_to_string(&*file_path) {
            Ok(contents) => contents,
            Err(err) => {
                println!("How stupid do you feel for typing to open a file that doesn't fucking exist, I mean really, the ignorance you have to your own directory that you have been building with this app is fucking hilarious\ntry it again and don't fuck it up.\nalr?\nalr.\nOh and for good measure, here is your error you fucking half-wit: {}\nwow, oH My gOd, what a shocker, couldn't find the fucking file\nyou're a joke.\n", err); 
                return;
            }
        };
        
        println!("Document opened.");

        let title: String = attributes[0].to_owned();

        let contents: String = contents
            .lines()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\n");

        clear(vec![]);
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
        println!("Existed document.");
        println!("write quit one more time to exit terminal\n");
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
        let _file = File::create(&*file_path)
            .expect("Error creating file"); 
        println!("File Created.\n");

        let title:String = attributes[0].to_owned();
        
        // add title to contents.txt list
        let mut list = get_contents();
        list.push(title);
        set_contents(&list);

        select(vec![attributes[0]]);
    }

    /// `files` is a function that prints the names of all files in the "mutable" directory.
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

        fig_header("Directory");

        let file_names = get_contents();
        for file_name in file_names {
            println!("{}", file_name);
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
        if attributes[0] == "contents" {
            println!("Delteing contents file will result in lost access to all of your files will remain unencrypted and only will be recoverable if you have a copy of the contents file and know how to replace it.");
        }
        let del = myinput("Are you sure you want to delete? (y/n)\n");
        if !(del.to_lowercase() == "y") {
            println!("deltion cancelled\n");
            return;
        }

        let file_path = get_path_from_attributes(&attributes, "/delete");

        println!("Deleting...");
        if !Path::new(&file_path).exists() {
            println!("File path does not exist.\n");
            return;
        }

        fs::remove_file(file_path)
            .expect("Couldn't remove file");
        println!("Deleted file.\n");

        // remove from contents
        let mut list = get_contents();
        if let Some(index) = list.iter().position(|x| *x == attributes[0]) {
            list.remove(index);
        }
        set_contents(&list);

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