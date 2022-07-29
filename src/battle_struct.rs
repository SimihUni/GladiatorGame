use crate::gladiator_struct::Gladiator;

pub struct Battle {
    players_turn: bool, // is it the players turn
    turn_count: u32,    // what turn is it currently
    player: Gladiator,
    enemy: Gladiator,
}

impl Battle {
    pub fn new(player: &Gladiator, enemy: &Gladiator) -> Battle {
        Battle {
            players_turn: true,
            turn_count: 1,
            player: player.clone(),
            enemy: enemy.clone(),
        }
    }

    ///increments turn_count with 1
    pub fn increment_turns(&mut self){
        self.turn_count += 1;
    }

    ///check if it is the players turn
    pub fn is_player_turn(&self) -> bool {
        self.players_turn
    }

    //set if it is the players turn
    pub fn set_player_turn(&mut self, new_value: bool) {
        self.players_turn = new_value;
    }
}