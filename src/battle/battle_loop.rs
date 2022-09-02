use std::str::FromStr;
use crate::gladiator_struct::Gladiator;
use crate::battle::battle_struct::Battle;
use crate::battle::battle_choose_move::battle_choose_move;
use crate::tio::clear_screen;
use crate::tio::press_to_continue;
use super::enemies::get_enemy;
use super::enemies::get_enemy_move;

///main function for logic of a battle
pub fn battle_loop(battle_difficulty: &mut u16, player: &mut Gladiator, random_enemy: &mut bool) {
    //idea: could add intro depending on what battle it is.

    //Gets random enemy, with a range of enemies depending on difficulty
    let enemy = get_enemy(*battle_difficulty,*random_enemy);

    //creating battle structure that controls the data that is relevent to the current battle.
    let mut battle_info = Battle::new(&player,&enemy);

    //idea: add turn limit, to balance infinite move uses --> Done
    let turn_limit: u32 = *battle_difficulty as u32 + 5;
    
    //loop
    loop {
        clear_screen();
        //idea: add items, which means adding a currency, and menu for choosing items and so on
        //idea: add move uses to balance strong moves --> Obsolete because turn limit
        //idea: add battles against two enemies (not likely to happen, reason: need for a battle loop rework)

        //screen with battle stats
        println!("{}",battle_info);
        println!("\t{} turns remaining.",turn_limit-battle_info.get_turn_count());

        //choose attack
        if battle_info.is_player_turn() {
            //potencially add choice of attack or item here
            //choose an attack and deal damage
            battle_info.deal_damage_to_enemy(battle_choose_move(player.get_move_list()));
        }
        else {
            //enemy's turn
            //deal damage
            //choose enemy's attack
            battle_info.deal_damage_to_player(get_enemy_move(&enemy));
        }

        //check battle state
        match battle_info.check_if_anyone_is_dead() {
            Some(b) => {
                if b {
                    //idea: if battle is successful give player tp -> Done ( and money if shop is implemented)
                    println!("Player won.");
                    //adding 2 TP to player
                    player.set_tp(player.get_tp() + 2);
                    *battle_difficulty += 1;
                    //check if player has beaten the end game boss
                    if enemy.get_name() == String::from_str("Boss Man").unwrap() {
                        //turn on random enemies
                        *random_enemy = true;
                        println!("\n\tCongratulations you defeated the final boss.\n\n\tNow you can continue to fight random gladiators named Razvigor, but this was the end of the game\n");
                        press_to_continue();
                    }
                    break
                }
                else {
                    println!("Enemy won.");
                    //lower difficulty
                    if *battle_difficulty > 1 {
                        *battle_difficulty -= 1;
                    }
                    break
                }
            },
            None => {
                if turn_limit == battle_info.get_turn_count() {
                    println!("Draw. Both gladiators survived the battle.");
                    //adding 1 TP to player
                    player.set_tp(player.get_tp() + 1);
                    //increase battle difficulty
                    *battle_difficulty += 1;
                    break
                }
            },
        };

        //update turn variables
        battle_info.increment_turns();
        //check if a gladiator is stunned and if there is a need to change whose turn it is
        if (battle_info.is_player_turn() && battle_info.is_enemy_stunned()) || (!battle_info.is_player_turn() && battle_info.is_player_stunned()) {
            //display who got stunned
            battle_info.unstun_gladiators();
            press_to_continue();
            continue;
        }

        battle_info.flip_turn();
        press_to_continue();
        //repeat
    }
}