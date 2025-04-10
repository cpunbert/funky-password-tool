pub mod command;
pub mod crypto;
pub mod parser;
pub mod password;

static DATA_PATH: &str = "data/passwords.csv";

fn main() -> std::io::Result<()> {
    //TODO: get ridf of unwrap nad handle error for save to csv
    let input: Vec<String> = std::env::args().skip(1).collect();
    let password_list = parser::open_csv(DATA_PATH).unwrap();
    let command = command::parse(input);
    let password_list = command.execute(password_list);

    parser::save_to_csv(DATA_PATH, password_list);

    Ok(())
}
