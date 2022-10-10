use std::fs::OpenOptions;
use std::io::{self};
use std::io::Write;

struct Address {
    name: String,
    number: String
}

fn main() {
    let mut name = String::new();
    let mut number = String::new();

    println!("Name: ");
    io::stdin().read_line(&mut name).expect("Failed to read Name");

    println!("Number: ");
    io::stdin().read_line(&mut number).expect("Failed to read Number");

    let new_entry: Address = Address {
        name: name,
        number: number
    };

    let mut file = OpenOptions::new().write(true).append(true).open("phonebook.csv").unwrap();

    let to_write = format!("{},{}", new_entry.name.replace(|c: char| c == '\n', ""), new_entry.number);
    let result = file.write_all(to_write.as_bytes());

    match result {
        Ok(_) => println!("Data written successfully"),
        Err(e) => println!("Error: {}", e),
    }
}

