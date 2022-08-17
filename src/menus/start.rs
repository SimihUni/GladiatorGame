use crate::tio::{self, clear_screen};
use std::str::FromStr;
use crate::menus::main_menu;

///menu for starting the game
pub fn start_menu() {
    loop {
        clear_screen();
        print!("\n\t1: Start game\n\t2: Exit\n    Input: \n");
        //input segment
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        //debug
        //println!("{:?}",&input);
        let choice = u8::from_str(input).unwrap_or_default();
        //end of input segment
        match choice {
            //main menu
            1 => main_menu::main_menu(),
            //exit
            2 => std::process::exit(0),
            _ => continue,
        }
    }
}