/*
let standard_font = FIGfont::standard().unwrap();
let figure = standard_font.convert("FIGlet");
assert!(figure.is_some());

let small_font = FIGfont::from_file("resources/small.flf").unwrap();
let figure = small_font.convert("FIGlet");
assert!(figure.is_some());
*/

use figlet_rs::FIGfont;

pub fn fig_header(msg: &str) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(msg);
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}