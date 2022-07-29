use crate::menus::choose_character;
use crate::menus::training_menu;
use crate::tio;
use std::str::FromStr;

pub fn main_menu() {
    let mut player = choose_character::choose_character();
    let mut tutorial: bool = true;
    player.set_tp(2);
    loop {
        if tutorial {
            print!("\n{}: I don't know any moves. So I need to train.",player.get_name());
            println!("\n\t1: Battle (not avaliable)");
        }
        else{
            print!("\n{}: Can't wait to meet my next foe.",player.get_name());
            println!("\n\t1: Battle");
        }
        println!("\n\t2: Train");
        println!("\n\t3: Shop /todo/");
        println!("\n\t4: Exit");
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        println!("{:?}",&input);
        let choice = u8::from_str(input).unwrap_or_default();
        match choice {
            1 => {
                if tutorial {
                    print!("\n{}: I can't fight now.",player.get_name());
                }
                else{
                    todo!();
                }
            },
            2 => training_menu::training_menu(&mut player, &mut tutorial),
            3 => todo!(),
            4 => std::process::exit(0),
            _ => continue,
        }
    }
}