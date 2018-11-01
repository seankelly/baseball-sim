use std::default::Default;

use event;
use team;

#[derive(Default)]
pub struct Game {
    away_team: team::Team,
    away_score: u8,
    home_team: team::Team,
    home_score: u8,
    active_team: ActiveTeam,
    active_batter: CurrentBatter,
    inning: u8,
    outs: u8,
    bases: BaseState,
    state: GameStatus,
    finished: bool
}

enum BaseState {
    Empty,
    First,
    Second,
    Third,
    FirstSecond,
    FirstThird,
    SecondThird,
    Loaded,
}

enum GameStatus {
    Pending,
    Started,
    Finished,
}

enum ActiveTeam {
    Away,
    Home,
}

// The u8 is the index into the lineup.
enum CurrentBatter {
    Away(u8),
    Home(u8),
}


impl Default for BaseState {
    fn default() -> Self {
        BaseState::Empty
    }
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus::Pending
    }
}

impl Default for ActiveTeam {
    fn default() -> Self {
        ActiveTeam::Away
    }
}

impl Default for CurrentBatter {
    fn default() -> Self {
        CurrentBatter::Away(0)
    }
}


impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn finish_game(&mut self) {
        while !self.finished {
            self.step_plate_appearance();
            self.check_game_over();
        }
    }

    pub fn step_plate_appearance(&mut self) {
        let event = event::Event::random_event();
        match event {
            event::Event::Out | event::Event::Flyout | event::Event::Groundout | event::Event::Strikeout => {
                self.outs += 1;
            }
            event::Event::Walk => {
            }
            event::Event::Single => {
            }
            event::Event::Double => {
            }
            event::Event::Triple => {
            }
            event::Event::HomeRun => {
                self.add_runs(1);
            }
        }

        if self.outs == 3 {
            match self.active_team {
                ActiveTeam::Away => {
                    self.active_team = ActiveTeam::Home;
                }
                ActiveTeam::Home => {
                    self.inning = self.inning + 1;
                    self.active_team = ActiveTeam::Away;
                }
            }
            self.outs = 0;
        }
    }

    fn add_runs(&mut self, runs: u8) {
        match self.active_team {
            ActiveTeam::Away => {
                println!("away team scored");
                self.away_score += runs;
            }
            ActiveTeam::Home => {
                println!("home team scored");
                self.home_score += runs;
            }
        }
    }

    fn check_game_over(&mut self) -> bool {
        if self.finished {
            return true;
        }
        else if self.inning < 10 {
            return false;
        }
        else if self.home_score != self.away_score {
            self.finished = true;
            return true;
        }
        return false;
    }
}
