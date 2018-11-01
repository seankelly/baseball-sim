
pub struct Player {
    name: String,
}

struct Batter {
    name: String,
}

struct Pitcher {
    name: String,
}


impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_owned(),
        }
    }
}
