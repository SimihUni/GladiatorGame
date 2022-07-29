use crate::tio;
use std::str::FromStr;
use crate::menus::main_menu;

///menu for starting the game
pub fn start_menu() {
    loop {
        print!("\n\t1: Start game\n\t2: Exit\n    Input: ");
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        println!("{:?}",&input);
        let choice = u8::from_str(input).unwrap_or_default();
        match choice {
            1 => main_menu::main_menu(),
            2 => std::process::exit(0),
            _ => continue,
        }
    }
}