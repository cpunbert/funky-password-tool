use std::fs::read_to_string;

use rand::Rng;
static PASSWORD_CHAR_SET: &str =
    "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ0123456789~`!@#$%^&*()-_+={}[]|:<>,./?";
fn main() -> std::io::Result<()> {
    let mut input: Vec<String> = std::env::args().skip(1).collect();
    let mut password_list = Vec::<PasswordEntry>::new();
    let one = PasswordEntry::new(
        String::from("google"),
        String::from("adam.nowak@gmail.com"),
        String::from("password"),
    );
    password_list.push(one);
    let asd = parse_input(input);
    asd.execute(password_list);

    let entries = open_csv("passwords.csv").unwrap();
    for entry in entries {
        entry.display();
    }

    Ok(())
}
fn open_csv(path: &str) -> Result<Vec<PasswordEntry>, ()> {
    let Ok(data) = read_to_string(path) else {
        return Err(());
    };
    let mut lines: Vec<String> = data.lines().map(String::from).collect();
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

fn parse_input(mut input: Vec<String>) -> Command {
    match input.remove(0).to_lowercase().as_str() {
        "show" => Command::Show(),
        "new" => Command::New(input),
        "edit" => Command::Edit(input),
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

fn new(mut input: Vec<String>, mut list: Vec<PasswordEntry>) -> Vec<PasswordEntry> {
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
    list
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
        }
    }
}
