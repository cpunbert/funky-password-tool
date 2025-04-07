use std::fs::{read_to_string, write};

use rand::Rng;
static DATA_PATH: &str = "data/passwords.csv";
static PASSWORD_CHAR_SET: &str =
    "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ0123456789~`!@#$%^&*()-_+={}[]|:<>,./?";
fn main() -> std::io::Result<()> {
    let input: Vec<String> = std::env::args().skip(1).collect();
    let password_list = open_csv(DATA_PATH).unwrap();
    let asd = parse_input(input);
    let password_list = asd.execute(password_list);

    save_to_csv(DATA_PATH, password_list);

    Ok(())
}
fn open_csv(path: &str) -> Result<Vec<PasswordEntry>, ()> {
    let Ok(data) = read_to_string(path) else {
        return Err(());
    };
    let lines: Vec<String> = data.lines().map(String::from).collect();
    let mut list_of_entries = Vec::<PasswordEntry>::with_capacity(lines.len());
    for line in lines {
        let mut x: Vec<&str> = line.split(';').collect();
        let entry = PasswordEntry::new(
            String::from(x.remove(0)),
            String::from(x.remove(0)),
            String::from(x.remove(0)),
        );
        list_of_entries.push(entry);
    }
    Ok(list_of_entries)
}
fn save_to_csv(path: &str, data: Vec<PasswordEntry>) {
    let mut result = String::new();
    for entry in data {
        let x = entry.name;
        let y = entry.login;
        let z = entry.password;
        result.push_str(&x);
        result.push(';');
        result.push_str(&y);
        result.push(';');
        result.push_str(&z);
        result.push('\n');
    }
    write(path, result);
}

fn parse_input(mut input: Vec<String>) -> Command {
    match input.remove(0).to_lowercase().as_str() {
        "show" => Command::Show(),
        "new" => Command::New(input),
        "edit" => Command::Edit(input),
        "help" => Command::Help(),
        _ => {
            println!("Invalid command");
            Command::None()
        }
    }
}

fn generate_password(password_strength: PasswordStrength, password_length: usize) -> String {
    let mut rng = rand::rng();
    let charset = match password_strength {
        PasswordStrength::Lowercase => &PASSWORD_CHAR_SET[0..26],
        PasswordStrength::Uppercase => &PASSWORD_CHAR_SET[0..52],
        PasswordStrength::Numbers => &PASSWORD_CHAR_SET[0..62],
        PasswordStrength::SpecialCharacters => &PASSWORD_CHAR_SET[0..90],
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
    fn new(name: String, login: String, password: String) -> Self {
        PasswordEntry {
            name,
            login,
            password,
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
    Show(),
    New(Vec<String>),
    Edit(Vec<String>),
    None(),
    Help(),
}

impl Command {
    fn execute(self, mut list: Vec<PasswordEntry>) -> Vec<PasswordEntry> {
        match self {
            Self::Show() => {
                for entry in &list {
                    entry.display();
                }
                return list;
            }
            Self::New(mut input) => {
                let name = input.remove(0);
                let login = input.remove(0);
                let pas_str = match input.remove(0).as_str() {
                    "1" => PasswordStrength::Lowercase,
                    "2" => PasswordStrength::Uppercase,
                    "3" => PasswordStrength::Numbers,
                    "4" => PasswordStrength::SpecialCharacters,
                    _ => {
                        println!("Invalid password strength");
                        return list;
                    }
                };
                let pas_len = match input.remove(0).parse::<usize>() {
                    Ok(len) => len,
                    Err(_) => {
                        println!("Invalid password length");
                        return list;
                    }
                };
                let result = PasswordEntry {
                    name,
                    login,
                    password: generate_password(pas_str, pas_len),
                };
                list.push(result);
                println!("Entry added succesfully");
                list
            }
            //FOR LATER: implement edit command
            Self::Edit(input) => list,
            Self::None() => list,
            Self::Help() => {
                println!("Help message");
                list
            }
        }
    }
}
