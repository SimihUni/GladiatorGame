extern crate gladiator;

use gladiator::gladiator_struct::{self, Gladiator};
use gladiator::attacks;
use gladiator::tio;
use std::io;
use std::io::Write;
use std::str::FromStr;


fn main() {
    println!("Gladiators");
    start_menu();
}

///menu for starting the game
fn start_menu() {
    loop {
        print!("\n\t1: Start game\n\t2: Exit\n    Input: ");
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        println!("{:?}",&input);
        let choice = u8::from_str(input).unwrap();
        match choice {
            1 => main_menu(),
            2 => std::process::exit(0),
            _ => continue,
        }
    }
}



///menu for choosing starting stats
fn choose_character() -> gladiator_struct::Gladiator {
    let stats:(u8,u8,u8);
    //menu for starting stats
    loop {
        print!("\n\t1: SPD=10,STM=5,STR=5\n\t2: SPD=5,STM=10,STR=5\n\t3: SPD=5,STM=5,STR=10\n    Input: ");
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        println!("{:?}",&input);
        let choice = u8::from_str(input).unwrap();
        match choice {
            1 => {stats = (10,5,5); break;},
            2 => {stats = (5,10,5); break;},
            3 => {stats = (5,5,10); break;},
            _ => continue,
        }
    }
    //prompt for character name
    io::stdout().flush().unwrap();
    println!("Choose a name for your gladiator: ");
    let input = match tio::get_input() {
        Ok(i) => i,
        Err(e) => panic!("Error in io. {}",e),
    };
    let input = String::from(input.trim());
    println!("name of gladiator = {:?}",&input);

    gladiator_struct::Gladiator::new(input,stats.0,stats.1,stats.2)
}

fn main_menu() {
    let mut player = choose_character();
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
        let choice = u8::from_str(input).unwrap();
        match choice {
            1 => {
                if tutorial {
                    print!("\n{}: I can't fight now.",player.get_name());
                }
                else{
                    todo!();
                }
            },
            2 => training_menu(&mut player, &mut tutorial),
            3 => todo!(),
            4 => std::process::exit(0),
            _ => continue,
        }
    }
}

fn training_menu(player: &mut Gladiator, tutorial: &mut bool) {
    loop{
        print!("\n\tTraining points: {}",player.get_tp());
        if *tutorial {
            print!("\n{}: The least I should do is train to hit and to dodge.",player.get_name());
            print!("\n\tTrain moves");
            print!("\n\t0: Go back");
            print!("\n\t1: Stab");
            if player.is_move_known(attacks::Attack::Stab) {
                print!(" - Known");
            }
            else{
                print!(" - Cost 1 TP");
            }
            print!("\n\t2: Dodge");
            if player.is_move_known(attacks::Attack::Dodge) {
                print!(" - Known");
            }
            else{
                print!(" - Cost 1 TP");
            }
            print!("\n Choose: ");
        }
        else{
            print!("\n\tTrain moves");
            print!("\n\t0: Go back");
            print!("\n\t1: Tackle");
            if player.is_move_known(attacks::Attack::Tackle) {
                print!(" - Known");
            }
            else{
                print!(" - Cost 1 TP");
            }
            print!("\n\t2: Stun");
            if player.is_move_known(attacks::Attack::Stun) {
                print!(" - Known");
            }
            else{
                print!(" - Cost 1 TP");
            }
            print!("\n\t3: Smash");
            if player.is_move_known(attacks::Attack::Smash) {
                print!(" - Known");
            }
            else{
                print!(" - Cost 2 TP");
            }
            print!("\n Choose: ");
        }
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        println!("{:?}",&input);
        let choice = u8::from_str(input).unwrap();
        match choice {
            0 => break,
            1 => {
                if *tutorial {
                    if player.is_move_known(attacks::Attack::Stab) {
                        println!("\n{} already knows how to stab people.",player.get_name())
                    }
                    else {
                        player.add_move(attacks::Attack::Stab);
                        println!("\n{} learned how to stab people.",player.get_name());
                    }
                }
                else {
                    if player.is_move_known(attacks::Attack::Tackle) {
                        println!("\n{} already knows how to tackle people.",player.get_name())
                    }
                    else {
                        if player.get_tp() < 1 {
                            println!("\n{} doesn't have enough training points, go battle some more than come back.",player.get_name());
                            continue;
                        }
                        player.add_move(attacks::Attack::Tackle);
                        println!("\n{} learned how to tackle people.",player.get_name());
                    }
                }
            },
            2 => {
                if *tutorial {
                    if player.is_move_known(attacks::Attack::Dodge) {
                        println!("\n{} already knows how to dodge attacks.",player.get_name())
                    }
                    else {
                        player.add_move(attacks::Attack::Dodge);
                        println!("\n{} learned how to dodge attacks.",player.get_name());
                    }
                }
                else {
                    if player.is_move_known(attacks::Attack::Stun) {
                        println!("\n{} already knows how to stun people.",player.get_name())
                    }
                    else {
                        if player.get_tp() < 1 {
                            println!("\n{} doesn't have enough training points, go battle some more than come back.",player.get_name());
                            continue;
                        }
                        player.add_move(attacks::Attack::Stun);
                        println!("\n{} learned how to stun people.",player.get_name());
                    }
                }
            },
            3 => {
                if player.is_move_known(attacks::Attack::Smash) {
                    println!("\n{} already knows how to smash people.",player.get_name())
                }
                else {
                    if player.get_tp() < 2 {
                        println!("\n{} doesn't have enough training points, go battle some more than come back.",player.get_name());
                    }
                    else
                    {
                        player.add_move(attacks::Attack::Smash);
                        println!("\n{} learned how to smash people.",player.get_name());
                    }
                }
            },
            _ => continue,
        }
    }
    
}