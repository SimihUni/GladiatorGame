use crate::gladiator_struct::Gladiator;



pub fn get_enemy(battle_difficulty: u16) -> Gladiator {
    Gladiator::new("Grog".to_string(),4,3,4)
}