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

    pub fn result(&self) {
        if !self.finished {
            return;
        }
        if self.away_score > self.home_score {
            println!("Away team won, {}-{}, in {} innings",
                     self.away_score, self.home_score, self.inning - 1);
        }
        else if self.home_score > self.away_score {
            println!("Home team won, {}-{}, in {} innings",
                     self.home_score, self.away_score, self.inning - 1);
        }
        else {
            println!("Tie game!");
        }
    }

    pub fn finish_game(&mut self) {
        while !self.finished {
            self.step_plate_appearance();
            self.check_game_over();
        }
    }

    pub fn step_plate_appearance(&mut self) {
        let event = event::Event::random_event();
        let mut runs_scored = 0;
        match event {
            event::Event::Out | event::Event::Flyout | event::Event::Groundout | event::Event::Strikeout => {
                self.outs += 1;
            }
            event::Event::Walk => {
                runs_scored = self.batter_bases(1);
            }
            event::Event::Single => {
                runs_scored = self.batter_bases(1);
            }
            event::Event::Double => {
                runs_scored = self.batter_bases(2);
            }
            event::Event::Triple => {
                runs_scored = self.batter_bases(3);
            }
            event::Event::HomeRun => {
                runs_scored = self.batter_bases(4);
            }
        }

        self.add_runs(runs_scored);

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

    fn batter_bases(&mut self, bases: u8) -> u8 {
        let mut runs_scored = 0;

        match self.bases {
            BaseState::Empty => {
                match bases {
                    1 => { self.bases = BaseState::First }
                    2 => { self.bases = BaseState::Second }
                    3 => { self.bases = BaseState::Third }
                    4 => { runs_scored += 1 }
                    _ => {},
                }
            }
            BaseState::First => {
                match bases {
                    1 => { self.bases = BaseState::FirstSecond }
                    2 => { self.bases = BaseState::FirstThird }
                    3 => { self.bases = BaseState::Third; runs_scored += 1 }
                    4 => { self.bases = BaseState::Empty; runs_scored += 2 }
                    _ => {},
                }
            }
            BaseState::Second => {
                match bases {
                    1 => { self.bases = BaseState::FirstThird }
                    2 => { runs_scored += 1 }
                    3 => { self.bases = BaseState::Third; runs_scored += 1 }
                    4 => { self.bases = BaseState::Empty; runs_scored += 2 }
                    _ => {},
                }
            }
            BaseState::Third => {
                match bases {
                    1 => { self.bases = BaseState::First; runs_scored += 1 }
                    2 => { self.bases = BaseState::Second; runs_scored += 1 }
                    3 => { self.bases = BaseState::Third; runs_scored += 1 }
                    4 => { self.bases = BaseState::Empty; runs_scored += 2 }
                    _ => {},
                }
            }
            BaseState::FirstSecond => {
                match bases {
                    1 => { self.bases = BaseState::Loaded }
                    2 => { self.bases = BaseState::SecondThird; runs_scored += 1 }
                    3 => { self.bases = BaseState::Third; runs_scored += 2 }
                    4 => { self.bases = BaseState::Empty; runs_scored += 3 }
                    _ => {},
                }
            }
            BaseState::FirstThird => {
                match bases {
                    1 => { self.bases = BaseState::FirstSecond; runs_scored += 1 }
                    2 => { self.bases = BaseState::SecondThird; runs_scored += 1 }
                    3 => { self.bases = BaseState::Third; runs_scored += 2 }
                    4 => { self.bases = BaseState::Empty; runs_scored += 3 }
                    _ => {},
                }
            }
            BaseState::SecondThird => {
                match bases {
                    1 => { self.bases = BaseState::FirstThird; runs_scored += 1 }
                    2 => { self.bases = BaseState::Second; runs_scored += 2 }
                    3 => { self.bases = BaseState::Third; runs_scored += 2 }
                    4 => { self.bases = BaseState::Empty; runs_scored += 3 }
                    _ => {},
                }
            }
            BaseState::Loaded => {
                match bases {
                    1 => { self.bases = BaseState::Empty; runs_scored += 1 }
                    2 => { self.bases = BaseState::Empty; runs_scored += 2 }
                    3 => { self.bases = BaseState::Empty; runs_scored += 3 }
                    4 => { self.bases = BaseState::Empty; runs_scored += 4 }
                    _ => {},
                }
            }
        }

        return runs_scored;
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
