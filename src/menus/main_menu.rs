use crate::menus::choose_character;
use crate::menus::training_menu;
use crate::attacks;
use crate::tio;
use crate::battle::battle_loop::battle_loop;
use std::str::FromStr;

pub fn main_menu() {
    //creating a gladiator with a menu
    let mut player = choose_character::choose_character();
    //sets tutorial mode on
    let mut tutorial: bool = true;
    //sets the number of consecutive battles the gladiator has been in
    let mut battle_order:u16 = 0;
    //give the player 2 TP so they can learn the first two basic moves
    player.set_tp(2);
    //loop for the main menu
    loop {
        if tutorial && !(player.is_move_known(&attacks::Attack::Stab) && player.is_move_known(&attacks::Attack::Stab)) {
            //the player doesn't know the basic moves, so they are barred from battling
            print!("\n{}: I don't know any moves. So I need to train.",player.get_name());
            println!("\n\t1: Battle (not avaliable)");
        }
        else{
            //default message
            print!("\n{}: Can't wait to meet my next foe.",player.get_name());
            println!("\n\t1: Battle");
        }
        println!("\n\t2: Train");
        println!("\n\t3: Shop /todo/");
        println!("\n\t4: Exit");
        println!("\n    Input:");
        //input segment
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        println!("{:?}",&input);
        let choice = u8::from_str(input).unwrap_or_default();
        //end of input segment
        match choice {
            //battle 
            1 => {
                if tutorial && !(player.is_move_known(&attacks::Attack::Stab) && player.is_move_known(&attacks::Attack::Stab)) {
                    //message for trying to battle without enough moves in moves vector
                    print!("\n{}: I can't fight now.",player.get_name());
                }
                else{
                    battle_loop(&battle_order,&mut player);
                }
            },
            //training
            2 => training_menu::training_menu(&mut player, &mut tutorial),
            //shop
            3 => todo!(),
            //exit
            4 => std::process::exit(0),
            _ => continue,
        }
    }
}