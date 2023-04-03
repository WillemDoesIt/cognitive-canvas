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

use std::process::Command;

/// Enables or disables the ConPTY feature on Windows systems using the Windows Registry or PowerShell.
///
/// # Arguments
///
/// * `enabled` - A boolean value indicating whether the ConPTY feature should be enabled (`true`) or disabled (`false`).
///
/// * `print_status` - A boolean value indicating whether to print the status of the ConPTY feature after attempting to enable or disable it. If `false`, the function returns immediately without printing anything.
///
/// # Example
///
/// ```
/// set_conpty(true, true);
/// ```
///
/// # Notes
///
/// This function sets the `VirtualTerminalLevel` registry key to `1` to enable the ConPTY feature, and removes the `VirtualTerminalLevel` key to disable the feature. It uses PowerShell to execute the appropriate script, and prints a message indicating whether the operation was successful or not.
///
/// # Errors
///
/// If the PowerShell script fails to execute for any reason, an error message will be printed to the console.
fn set_conpty(enabled: bool, print_status: bool) {
    let script = if enabled {
        r#"
            reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1 /f
        "#
    } else {
        r#"
            Remove-ItemProperty -Path HKCU:\Console -Name VirtualTerminalLevel -ErrorAction SilentlyContinue
        "#
    };
    
    if !print_status {return}
    let action = if enabled { "enabled" } else { "disabled" };
    let output = Command::new("powershell")
                     .arg("-Command")
                     .arg(script)
                     .output()
                     .expect("failed to execute powershell script");

    if output.status.success() {
        println!("conpty feature {} successfully", action);
    } else {
        println!("failed to {} conpty feature: {}", action, output.status);
    }
}

fn main () {
    // Enable conpty feature
    // this is necessary to let lines be removed with println!
    set_conpty(true,false);

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

    let key = new_key("my password");
    match decrypt_files("mutable", &key.as_str()) {
        Ok(_) => println!("Decrypt files successfully."),
        Err(err) => println!("error: {}", err)
    }

    terminal::main();

    match encrypt_files(get_path("mutable").to_str().unwrap(), &key.as_str()){
        Ok(_) => println!("Encrypted files successfully."),
        Err(err) => println!("error: {}", err)
    }

    // Disable conpty feature
    set_conpty(false,false);
}