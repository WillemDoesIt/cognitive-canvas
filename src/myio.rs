use std::io;

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