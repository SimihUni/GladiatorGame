use crate::gladiator_struct;
use crate::tio;
use crate::attacks;
use std::str::FromStr;

///check and print menu option for learning
fn print_training_option(player: &gladiator_struct::Gladiator, cost: u8, mv:attacks::Attack, order_in_menu: u8) {
    print!("\n\t{}: {}",order_in_menu,mv);
    if player.is_move_known(&mv) {
        print!(" - Known");
    }
    else{
        print!(" - Cost {} TP",cost);
    }
}

///logic for trying to learn a move in training menu
fn try_to_learn_a_move(player: &mut gladiator_struct::Gladiator,cost: u8, mv:attacks::Attack) {
    if player.is_move_known(&mv) {
        println!("\n{} already knows how to {} people.",player.get_name(),&mv)
    }
    else {
        if player.get_tp() < cost {
            println!("\n{} doesn't have enough training points, go battle some more than come back.",player.get_name());
            return;
        }
        player.set_tp(player.get_tp() - cost);
        player.add_move(mv);
        println!("\n{} learned how to {} people.",player.get_name(),mv);
    }
}

pub fn training_menu(player: &mut gladiator_struct::Gladiator, tutorial: &mut bool) {
    let cost_to_increase_stat = 3;
    loop{
        tio::clear_screen();
        if player.get_move_list().len() > 1 {
            *tutorial = false;
        }
        print!("\n\tTraining points: {}",player.get_tp());
        if *tutorial {
            print!("\n{}: The least I should do is train to hit and to dodge.",player.get_name());
            print!("\n\tTrain moves");
            print!("\n\t0: Go back");
            print_training_option(&player,1,attacks::Attack::Stab,1);
            print_training_option(&player,1,attacks::Attack::Dodge,2);
            println!("\n   Choose: ");
        }
        else{
            print!("\n\tTrain moves");
            print!("\n\t0: Go back");
            print_training_option(&player,1,attacks::Attack::Tackle,1);
            print_training_option(&player,1,attacks::Attack::Stun,2);
            print_training_option(&player,2,attacks::Attack::Smash,3);
            print!("\n\t4: Increase strength");
            if player.get_str() == 255 {
                print!(" - Max");
            }
            else {
                print!(" - Currently: {} - Cost {}",player.get_str(),cost_to_increase_stat);
            }
            print!("\n\t5: Increase stamina");
            if player.get_stm() == 255 {
                print!(" - Max");
            }
            else {
                print!(" - Currently: {} - Cost {}",player.get_stm(),cost_to_increase_stat);
            }
            print!("\n\t6: Increase speed");
            if player.get_spd() == 255 {
                print!(" - Max");
            }
            else {
                print!(" - Currently: {} - Cost {}",player.get_spd(),cost_to_increase_stat);
            }
            println!("\n   Choose: ");
        }
        //input segment
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        //println!("{:?}",&input);    //for debug
        let choice = u8::from_str(input).unwrap_or_default();
        //end of input segment

        match choice {
            0 => break,
            1 => {
                if *tutorial {
                    if player.is_move_known(&attacks::Attack::Stab) {
                        println!("\n{} already knows how to stab people.",player.get_name())
                    }
                    else {
                        player.add_move(attacks::Attack::Stab);
                        player.set_tp(player.get_tp() - 1);
                        println!("\n{} learned how to stab people.",player.get_name());
                    }
                }
                else {
                    try_to_learn_a_move(player, 1, attacks::Attack::Tackle);
                }
            },
            2 => {
                if *tutorial {
                    if player.is_move_known(&attacks::Attack::Dodge) {
                        println!("\n{} already knows how to dodge attacks.",player.get_name())
                    }
                    else {
                        player.add_move(attacks::Attack::Dodge);
                        player.set_tp(player.get_tp() - 1);
                        println!("\n{} learned how to dodge attacks.",player.get_name());
                    }
                }
                else {
                    try_to_learn_a_move(player, 1, attacks::Attack::Stun);
                }
            },
            3 => {
                try_to_learn_a_move(player, 2, attacks::Attack::Smash);
            },
            4 => {
                if player.get_str() == 255 {
                    println!("\n{} is at peek strength.",player.get_name());
                    continue;
                }
                if player.get_tp() < cost_to_increase_stat {
                    println!("\n{} doesn't have enough training points, go battle some more than come back.",player.get_name());
                    return;
                }
                player.set_tp(player.get_tp() - cost_to_increase_stat);
                //increase strength
                player.increase_str();
                println!("\n{} increased his strength to {}.",player.get_name(),player.get_str());
            },
            5 => {
                if player.get_stm() == 255 {
                    println!("\n{} is at peek stamina.",player.get_name());
                    continue;
                }
                if player.get_tp() < cost_to_increase_stat {
                    println!("\n{} doesn't have enough training points, go battle some more than come back.",player.get_name());
                    return;
                }
                player.set_tp(player.get_tp() - cost_to_increase_stat);
                player.increase_stm();
                println!("\n{} increased his stamina to {}.",player.get_name(),player.get_stm());
            },
            6 => {
                if player.get_spd() == 255 {
                    println!("\n{} is at peek speed.",player.get_name());
                    continue;
                }
                if player.get_tp() < cost_to_increase_stat {
                    println!("\n{} doesn't have enough training points, go battle some more than come back.",player.get_name());
                    return;
                }
                player.set_tp(player.get_tp() - cost_to_increase_stat);
                player.increase_spd();
                println!("\n{} increased his speed to {}.",player.get_name(),player.get_spd());
            },
            _ => continue,
        }
    }
}