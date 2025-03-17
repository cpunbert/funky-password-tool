use rand::Rng; /*should be replaced in the future */
static PASSWORD_CHAR_SET: &str =
    "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ0123456789~`!@#$%^&*()-_+={}[]|;:<>,./?";
fn main() {
    let pas1 = new_entry(
        String::from("amazon"),
        String::from("maupa@gmail.com"),
        PasswordStrength::SpecialCharacters,
        32,
    );
    println!("{a}", a = pas1.password);
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
fn new_entry(name: String, login: String, str: PasswordStrength, len: usize) -> PasswordEntry {
    return PasswordEntry {
        name: name,
        login: login,
        password: generate_password(str, len),
    };
}

enum PasswordStrength {
    Lowercase,
    Uppercase,
    Numbers,
    SpecialCharacters,
}
