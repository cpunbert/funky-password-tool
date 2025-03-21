use std::io::Error;

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
//? ditch enum, instead match 3rd arg as stren,
//parse command line args one by one into variables???? and do stuff afterwards
fn pinp(mut input: Vec<String>) -> Result<Command, Error> {
    let command = input[0].to_lowercase().as_str();
    let arg0 = input.remove(1).parse::<usize>();
    Ok(Command::Display(vec![input[0]], (arg0, arg0)))
}
fn parse_input(mut input: Vec<String>) -> Result<Command, ()> {
    match input[0].to_lowercase().as_str() {
        "show" => Ok(Command::Display),
        "new" => Ok(Command::New(input)),
        "edit" => Ok(Command::Edit(input)),
        _ => Err(()),
    }
}
//TODO: FIX function above

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
    Display(Vec<String>, (usize, usize)),
    New(Vec<String>),
    Edit(Vec<String>),
}
