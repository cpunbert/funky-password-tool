use crate::password;
use std::fs::{read_to_string, write};
pub fn open_csv(path: &str) -> Result<Vec<password::PasswordEntry>, ()> {
    let Ok(data) = read_to_string(path) else {
        return Err(());
    };
    let lines: Vec<String> = data.lines().map(String::from).collect();
    let mut list_of_entries = Vec::<password::PasswordEntry>::with_capacity(lines.len());
    for line in lines {
        let mut x: Vec<&str> = line.split(';').collect();
        let entry = password::PasswordEntry::new(
            String::from(x.remove(0)),
            String::from(x.remove(0)),
            String::from(x.remove(0)),
        );
        list_of_entries.push(entry);
    }
    Ok(list_of_entries)
}

pub fn save_to_csv(path: &str, data: Vec<password::PasswordEntry>) -> Result<(), ()> {
    let mut result = String::new();
    for entry in data {
        result.push_str(&entry.name);
        result.push(';');
        result.push_str(&entry.login);
        result.push(';');
        result.push_str(&entry.password);
        result.push('\n');
    }
    match write(path, result) {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{e}");
            return Err(());
        }
    }
}
