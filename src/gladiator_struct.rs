use crate::attacks;

pub struct Gladiator {
    name: String,                 //name of the gladiator
    speed: u8,                    //determines which gladiator goes first in battle
    stamina: u8,                  //determines how much HP a gladiator has
    strength: u8,                 //determines how much damage a gladiator deals
    moves: Vec<attacks::Attack>,  //vector with the moves a gladiator knows
    health_points: u32,           //health points, if 0 => game over for gladiator
    training_points: u8,          //training points, used for learning new moves
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

    ///get health points
    pub fn get_hp(&self) -> u32 {
        self.health_points
    }

    ///set health points
    pub fn set_hp(&mut self, new_tp: u32) {
        self.health_points = new_tp;
    }

    ///get training points
    pub fn get_tp(&self) -> u8 {
        self.training_points
    }

    /// set training points
    pub fn set_tp(&mut self, new_tp: u8) {
        self.training_points = new_tp;
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