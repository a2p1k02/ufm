extern crate dirs;

use std::env;
use std::io::Write;

mod utils;

fn get_user_input() {
    env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
    utils::show_hints();
    loop {
        print!("{}> ", env::current_dir().unwrap().display());
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut input = input.trim().splitn(2, ' ');
        let command = input.next().unwrap();
        let args = input.peekable().peek().map_or("", |x| *x);

        if command != "q" {
            utils::check_command(command, args);
        } else {
            break;
        }
    }
}

fn main() {
    get_user_input();
}
