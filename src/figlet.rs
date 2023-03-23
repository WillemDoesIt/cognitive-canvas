/*
let standard_font = FIGfont::standard().unwrap();
let figure = standard_font.convert("FIGlet");
assert!(figure.is_some());

let small_font = FIGfont::from_file("resources/small.flf").unwrap();
let figure = small_font.convert("FIGlet");
assert!(figure.is_some());
*/

use figlet_rs::FIGfont;

/// Prints a message in the standard FIGlet font.
///
/// # Parameters
///
/// * `msg` - A string slice containing the message to be printed.
///
/// # Examples
///
/// ```
/// fig_header("Hello, World!");
/// ```
///
/// Output:
///
/// ```
/// _   _      _ _        __        __         _     _
///| | | | ___| | | ___   \ \      / /__  _ __| | __| |
///| |_| |/ _ \ | |/ _ \   \ \ /\ / / _ \| '__| |/ _` |
///|  _  |  __/ | | (_) |   \ V  V / (_) | |  | | (_| |
///|_| |_|\___|_|_|\___/     \_/\_/ \___/|_|  |_|\__,_|
///
/// ```
///
/// # Panics
///
/// Panics if the message could not be converted to FIGlet font.
pub fn fig_header(msg: &str) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(msg);
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}