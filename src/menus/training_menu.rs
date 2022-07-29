use crate::gladiator_struct;
use crate::tio;
use crate::attacks;
use std::str::FromStr;

pub fn training_menu(player: &mut gladiator_struct::Gladiator, tutorial: &mut bool) {
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