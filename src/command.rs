use crate::password;
pub fn parse(mut input: Vec<String>) -> Command {
    match input.remove(0).to_lowercase().as_str() {
        "show" => Command::Show(),
        "new" => Command::New(input),
        "edit" => Command::Edit(input),
        "help" => Command::Help(),
        _ => Command::None(),
    }
}

pub enum Command {
    Show(),
    New(Vec<String>),
    Edit(Vec<String>),
    None(),
    Help(),
}

impl Command {
    pub fn execute(self, mut list: Vec<password::PasswordEntry>) -> Vec<password::PasswordEntry> {
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
                    "1" => password::PasswordStrength::Lowercase,
                    "2" => password::PasswordStrength::Uppercase,
                    "3" => password::PasswordStrength::Numbers,
                    "4" => password::PasswordStrength::SpecialCharacters,
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
                let result = password::PasswordEntry {
                    name,
                    login,
                    password: password::generate_password(pas_str, pas_len),
                };
                list.push(result);
                println!("Entry added succesfully");
                list
            }
            //FOR LATER: implement edit command
            Self::Edit(input) => list,
            Self::None() => {
                println!("Invalid command. Use help to view valid commands.");
                list
            }
            Self::Help() => {
                println!("Help message");
                list
            }
        }
    }
}
