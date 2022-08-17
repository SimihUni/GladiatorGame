
use crate::attacks::Attack;
use crate::tio;
use std::str::FromStr;

fn print_move(mv:Attack, order: u8) {
    println!("\t{}: {}",order,mv);
}

pub fn battle_choose_move(move_list: Vec<Attack>) -> Attack {
    loop {
        println!("\n\tChoose what move to use.\n");
        let mut order = 1;
        for i in move_list.as_slice() {
            print_move(*i, order);
            order += 1;
        }
        println!("\nChoose:");
        //input segment
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        //debug
        //println!("{:?}",&input);
        let mut choice = u8::from_str(input).unwrap_or_default();
        //end of input segment
        if choice == 0 {
            continue;
        }
        if choice <= move_list.len() as u8 {
            choice -= 1;
            return move_list[choice as usize];
        }
        
    }
}