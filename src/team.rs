use std::default::Default;
use std::vec::Vec;

use rand::{thread_rng, Rng};

use player;

pub struct Team {
    lineup: Vec<player::Player>,
}


impl Default for Team {
    fn default() -> Self {
        Team::league_average()
    }
}

impl Team {
    pub fn league_average() -> Self {
        let mut players = Vec::new();
        for idx in 1..10 {
            players.push(player::Player::new(&format!("Player {}", idx), idx));
        }

        let mut rng = thread_rng();
        rng.shuffle(&mut players);

        let team = Team {
            lineup: players,
        };
        return team;
    }
}
