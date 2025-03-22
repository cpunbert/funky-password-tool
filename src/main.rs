use std::{f32::consts::E, io::Error, num::ParseIntError};

use rand::Rng; /*should be replaced in the future */
static PASSWORD_CHAR_SET: &str =
    "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ0123456789~`!@#$%^&*()-_+={}[]|;:<>,./?";
fn main() {
    let mut input: Vec<String> = std::env::args().skip(1).collect();
    for argument in &input {
        println!("{argument}")
    }
    for argument in &input {
        println!("{argument}")
    }
    let one = PasswordEntry::new(
        String::from("google"),
        String::from("adam.nowak@gmail.com"),
        PasswordStrength::SpecialCharacters,
        40,
    );
    let two = PasswordEntry::new2(input);
    one.display();
    two.display();
}
/*fn parse_usize(str: String) -> Result<usize, ParseIntError> {
    let arg = str.parse::<usize>()?;
    Ok(arg)
}*/
// parse input with .expect() just to make it work
fn parse_input(mut input: Vec<String>) -> Result<Command, ()> {
    match input[0].to_lowercase().as_str() {
        "show" => Ok(Command::Display()),
        "new" => Ok(Command::New(input)),
        "edit" => Ok(Command::Edit(input)),
        _ => Err(()),
    }
}

fn generate_password(password_strength: PasswordStrength, password_length: usize) -> String {
    let mut rng = rand::rng();
    let charset = match password_strength {
        PasswordStrength::Lowercase => &PASSWORD_CHAR_SET[0..26],
        PasswordStrength::Uppercase => &PASSWORD_CHAR_SET[0..52],
        PasswordStrength::Numbers => &PASSWORD_CHAR_SET[0..62],
        PasswordStrength::SpecialCharacters => &PASSWORD_CHAR_SET[0..91],
    };
    let mut password = String::new();
    while (password.len() < password_length) {
        password.push(charset.as_bytes()[rng.random_range(0..charset.len())] as char);
    }
    return password;
}
/*
FPL new name login pass_len pass_str
?struct to generate password??
*/
struct PasswordEntry {
    name: String,
    login: String,
    password: String,
}

impl PasswordEntry {
    fn new(name: String, login: String, pas_str: PasswordStrength, pas_len: usize) -> Self {
        PasswordEntry {
            name: name,
            login: login,
            password: generate_password(pas_str, pas_len),
        }
    }
    fn new2(mut input: Vec<String>) -> Self {
        PasswordEntry {
            name: input.remove(0),
            login: input.remove(0),
            password: generate_password(PasswordStrength::SpecialCharacters, 40),
        }
    }
    fn display(&self) {
        println!(
            "\n name:     {}\n login:    {}\n password: {} ",
            self.name, self.login, self.password
        )
    }
}

enum PasswordStrength {
    Lowercase,
    Uppercase,
    Numbers,
    SpecialCharacters,
}

enum Command {
    Display(),
    New(Vec<String>),
    Edit(Vec<String>),
}
