
pub struct Player {
    name: String,
    number: u8
}

struct Batter {
    name: String,
}

struct Pitcher {
    name: String,
}


impl Player {
    pub fn new(name: &str, number: u8) -> Self {
        Player {
            name: name.to_owned(),
            number: number,
        }
    }
}
