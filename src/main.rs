use std::{fs::OpenOptions, io::{Read, Write}};

use crate::ara_letters::clear_message;

mod ara_letters;

fn main() {
    let letters = ara_letters::all_letters();
    // println!("{:#?}", letters);
    
    // let mut buf = String::new();
    // std::io::stdin().read_line(&mut buf).expect("Can't read from stdin..");
    let mut file_income =
        OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open("./file_income.txt")
            .unwrap();

    let mut file_contect = String::with_capacity(2048);
    file_income.read_to_string(&mut file_contect).unwrap();

    let mut file_outcome =
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .read(true)
            .open("./file_outcome.txt")
            .unwrap();

    if let Some(m) = clear_message(&letters[1], file_contect) { 
        file_outcome.write_all(m.as_bytes()).unwrap();
        file_outcome.flush().unwrap();
    }
    // println!("{:#?}", cleared);
}
