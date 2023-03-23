use std::io;

/// This function takes a `&str` as an argument and returns a `String` as output. It prints out the string passed in as an argument, and then reads a single line of user input, trims it, and parses it into a `String` before returning it.
///
/// # Examples
///
/// ```
/// let name = myinput("What's your name? ");
/// ```
/// 
/// # Errors
/// 
/// This function may panic if there is an error reading or parsing the user input.
pub fn myinput(msg: &str) -> String {
    print!("{}", msg);
    let mut input = String::with_capacity(5);            // get input
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    let input:String = input
        .trim()
        .parse()
        .expect("Error parsing number");

    return input;
}