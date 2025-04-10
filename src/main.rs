mod command;
mod crypto;
mod parser;
mod password;

static DATA_PATH: &str = "data/passwords.csv";

fn main() -> Result<(), ()> {
    //TODO: get ridf of unwrap nad handle error for save to csv
    let input: Vec<String> = std::env::args().skip(1).collect();
    let Ok(password_list) = parser::open_csv(DATA_PATH) else {
        return Err(());
    };
    let command = command::parse(input);
    let password_list = command.execute(password_list);

    match parser::save_to_csv(DATA_PATH, password_list) {
        Ok(()) => Ok(()),
        Err(()) => Err(()),
    }
}
