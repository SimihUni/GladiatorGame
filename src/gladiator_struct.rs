use crate::attacks;

pub struct Gladiator {
    ///name of the gladiator
    name: String,
    ///determines which gladiator goes first in battle
    speed: u8,
    ///determines how much HP a gladiator has
    stamina: u8,
    ///determines how much damage a gladiator deals
    strength: u8,
    ///vector with the moves a gladiator knows
    moves: Vec<attacks::Attack>,
    ///health points, if 0 => game over for gladiator
    health_points: u32,
    ///training points, used for learning new moves
    training_points: u8,
}

impl Gladiator {
    pub fn new(name: String, speed: u8, stamina: u8, strength: u8) -> Gladiator {
        let vec: Vec<attacks::Attack> = Vec::new();
        Gladiator {
            name,
            speed,
            stamina,
            strength,
            moves: vec,
            health_points: stamina as u32 * 5,
            training_points: 0,
        }
    }

    ///clone a gladiator, with 0 TP
    pub fn clone(&self) -> Gladiator {
        Gladiator {
            name: self.name.clone(),
            speed: self.speed,
            stamina: self.stamina,
            strength: self.strength,
            moves: self.moves.clone(),
            health_points: self.stamina as u32 * 5,
            training_points: 0,
        }
    }

    ///add a new move to the move verctor
    pub fn add_move(&mut self, new_move: attacks::Attack) {
        self.moves.push(new_move);
    }

    ///gets the gladiators name
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    ///get tuple with speed,stamina,strength stats
    pub fn get_stats(&self) -> (u8, u8, u8) {
        (self.speed,self.stamina,self.strength)
    }

    ///get strength
    pub fn get_str(&self) -> u8 {
        self.strength
    }

    ///increase streangth with 1
    pub fn increase_str(&mut self) {
        self.strength += 1;
    }

    ///get stamina
    pub fn get_stm(&self) -> u8 {
        self.stamina
    }

    ///increase stamina with 1
    pub fn increase_stm(&mut self) {
        self.stamina += 1;
    }

    ///get speed
    pub fn get_spd(&self) -> u8 {
        self.speed
    }

    ///increase speed with 1
    pub fn increase_spd(&mut self) {
        self.speed += 1;
    }

    ///get health points
    pub fn get_hp(&self) -> u32 {
        self.health_points
    }

    ///set health points
    pub fn set_hp(&mut self, new_hp: u32) {
        self.health_points = new_hp;
    }

    ///get training points
    pub fn get_tp(&self) -> u8 {
        self.training_points
    }

    ///set training points
    pub fn set_tp(&mut self, new_tp: u8) {
        self.training_points = new_tp;
    }

    ///get move list in a vector
    pub fn get_move_list(&self) -> Vec<attacks::Attack> {
        self.moves.clone()
    }

    /// checks if a move is already in the vector moves
    pub fn is_move_known(&self, _move: &attacks::Attack) -> bool {
        let v_iter = self.moves.iter();
        let mut result: bool = false;
        for i in v_iter {
            if *i == *_move {
                result = true;
            }
        }
        result
    }

}