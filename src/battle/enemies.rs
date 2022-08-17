use crate::gladiator_struct::Gladiator;
use crate::attacks::Attack;
extern crate rand;
use rand::Rng;

pub fn get_enemy(battle_difficulty: u16) -> Gladiator {
    let mut roster: Vec<Gladiator> = Vec::new();

    roster.push(Gladiator::new("Grog".to_string(),4,3,4));
    roster.last_mut().unwrap().add_move(Attack::Stab);

    roster.push(Gladiator::new("Drax".to_string(),5,5,5));
    roster.last_mut().unwrap().add_move(Attack::Stab);
    roster.last_mut().unwrap().add_move(Attack::Tackle);

    roster.push(Gladiator::new("Speedy Gonzalez".to_string(),20,5,3));
    roster.last_mut().unwrap().add_move(Attack::Stab);
    roster.last_mut().unwrap().add_move(Attack::Dodge);

    roster.push(Gladiator::new("Stronk Boi".to_string(),4,8,15));
    roster.last_mut().unwrap().add_move(Attack::Stab);
    roster.last_mut().unwrap().add_move(Attack::Tackle);
    roster.last_mut().unwrap().add_move(Attack::Smash);
    roster.last_mut().unwrap().add_move(Attack::Stun);

    roster.push(Gladiator::new("Mr. Muffins".to_string(),10,10,10));
    roster.last_mut().unwrap().add_move(Attack::Stab);
    roster.last_mut().unwrap().add_move(Attack::Tackle);
    roster.last_mut().unwrap().add_move(Attack::Stun);

    roster.push(Gladiator::new("Boss Man".to_string(),20,20,20));
    roster.last_mut().unwrap().add_move(Attack::Stab);
    roster.last_mut().unwrap().add_move(Attack::Tackle);
    roster.last_mut().unwrap().add_move(Attack::Smash);
    roster.last_mut().unwrap().add_move(Attack::Stun);
    roster.last_mut().unwrap().add_move(Attack::Dodge);

    let random_number = match battle_difficulty {
        1..=5 => rand::thread_rng().gen_range(0..2),
        6..=10 => rand::thread_rng().gen_range(2..5),
        _ => 5,
    };
    roster[random_number].clone()
}

pub fn get_enemy_move(enemy:&Gladiator) -> Attack {
    let move_list:Vec<Attack> = enemy.get_move_list();
    move_list[rand::thread_rng().gen_range(0..move_list.len()) as usize]
}
