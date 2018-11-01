use rand::prelude::*;
use rand::{thread_rng, Rng};

pub enum Event {
    Out,
    Groundout,
    Flyout,
    Strikeout,
    Walk,
    Single,
    Double,
    Triple,
    HomeRun,
}


impl Event {
    pub fn random_event() -> Self {
        let mut rng = thread_rng();
        let e: u8 = rng.gen_range(0, 9);

        match e {
            0 => Event::Out,
            1 => Event::Groundout,
            2 => Event::Flyout,
            3 => Event::Strikeout,
            4 => Event::Walk,
            5 => Event::Single,
            6 => Event::Double,
            7 => Event::Triple,
            8 => Event::HomeRun,
            _ => Event::Out,
        }
    }
}
