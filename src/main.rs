use rand::Rng; /*should be replaced in the future */
static PASSWORD_CHAR_SET: &str =
    "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ0123456789~`!@#$%^&*()-_+={}[]|;:<>,./?";
fn main() {
    let mut input: Vec<String> = std::env::args().skip(1).collect();
    let mut password_list = Vec::<PasswordEntry>::new();
    let one = PasswordEntry::new(
        String::from("google"),
        String::from("adam.nowak@gmail.com"),
        PasswordStrength::SpecialCharacters,
        40,
    );
    password_list.push(one);
    let asd = parse_input(input);
    asd.execute(password_list);
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
    fn new(name: String, login: String, pas_str: PasswordStrength, pas_len: usize) -> Self {
        PasswordEntry {
            name: name,
            login: login,
            password: generate_password(pas_str, pas_len),
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
