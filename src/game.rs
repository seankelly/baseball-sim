use std::default::Default;

use event;
use team;

#[derive(Default)]
pub struct Game {
    away_team: team::Team,
    away_score: u8,
    away_batter: u8,
    home_team: team::Team,
    home_score: u8,
    home_batter: u8,
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
        let mut game = Game::default();
        game.away_batter = 0;
        game.home_batter = 0;
        return game;
    }

    pub fn result(&self) {
        if !self.finished {
            return;
        }
        if self.away_score > self.home_score {
            println!("Away team won, {}-{}, in {} innings",
                     self.away_score, self.home_score, self.inning);
        }
        else if self.home_score > self.away_score {
            println!("Home team won, {}-{}, in {} innings",
                     self.home_score, self.away_score, self.inning);
        }
        else {
            println!("Tie game!");
        }
    }

    pub fn finish_game(&mut self) {
        while !self.finished {
            self.step_plate_appearance();
            self.check_game_over();
            self.check_next_inning();
        }
    }

    pub fn step_plate_appearance(&mut self) {
        let event = event::Event::random_event();
        let mut runs_scored = 0;
        match event {
            event::Event::Out | event::Event::Flyout | event::Event::Groundout | event::Event::Strikeout => {
                self.outs += 1;
            }
            _ => {
                runs_scored = self.batter_bases(event);
            }
        }

        self.add_runs(runs_scored);
        self.next_batter();
    }

    fn batter_bases(&mut self, event: event::Event) -> u8 {
        let mut runs_scored = 0;

        match (&self.bases, event) {
            (BaseState::Empty, event::Event::Walk) => {
                self.bases = BaseState::First;
            }
            (BaseState::Empty, event::Event::Single) => {
                self.bases = BaseState::First;
            }
            (BaseState::Empty, event::Event::Double) => {
                self.bases = BaseState::Second;
            }
            (BaseState::Empty, event::Event::Triple) => {
                self.bases = BaseState::Third;
            }
            (BaseState::Empty, event::Event::HomeRun) => {
                self.bases = BaseState::Empty;
                runs_scored += 1;
            }
            (BaseState::Empty, _) => {}
            (BaseState::First, event::Event::Walk) => {
                self.bases = BaseState::FirstSecond;
            }
            (BaseState::First, event::Event::Single) => {
                self.bases = BaseState::FirstSecond;
            }
            (BaseState::First, event::Event::Double) => {
                self.bases = BaseState::SecondThird;
            }
            (BaseState::First, event::Event::Triple) => {
                self.bases = BaseState::Third;
                runs_scored += 1;
            }
            (BaseState::First, event::Event::HomeRun) => {
                self.bases = BaseState::Empty;
                runs_scored += 2;
            }
            (BaseState::First, _) => {}
            (BaseState::Second, event::Event::Walk) => {
                self.bases = BaseState::FirstSecond;
            }
            (BaseState::Second, event::Event::Single) => {
                self.bases = BaseState::FirstThird;
            }
            (BaseState::Second, event::Event::Double) => {
                self.bases = BaseState::Second;
                runs_scored += 1;
            }
            (BaseState::Second, event::Event::Triple) => {
                self.bases = BaseState::Third;
                runs_scored += 1;
            }
            (BaseState::Second, event::Event::HomeRun) => {
                self.bases = BaseState::Empty;
                runs_scored += 2;
            }
            (BaseState::Second, _) => {}
            (BaseState::Third, event::Event::Walk) => {
                self.bases = BaseState::FirstThird;
            }
            (BaseState::Third, event::Event::Single) => {
                self.bases = BaseState::First;
                runs_scored += 1;
            }
            (BaseState::Third, event::Event::Double) => {
                self.bases = BaseState::Second;
                runs_scored += 1;
            }
            (BaseState::Third, event::Event::Triple) => {
                self.bases = BaseState::Third;
                runs_scored += 1;
            }
            (BaseState::Third, event::Event::HomeRun) => {
                self.bases = BaseState::Empty;
                runs_scored += 2;
            }
            (BaseState::Third, _) => {}
            (BaseState::FirstSecond, event::Event::Walk) => {
                self.bases = BaseState::Loaded;
            }
            (BaseState::FirstSecond, event::Event::Single) => {
                self.bases = BaseState::Loaded;
            }
            (BaseState::FirstSecond, event::Event::Double) => {
                self.bases = BaseState::SecondThird;
                runs_scored += 1;
            }
            (BaseState::FirstSecond, event::Event::Triple) => {
                self.bases = BaseState::Third;
                runs_scored += 2;
            }
            (BaseState::FirstSecond, event::Event::HomeRun) => {
                self.bases = BaseState::Empty;
                runs_scored += 3;
            }
            (BaseState::FirstSecond, _) => {}
            (BaseState::FirstThird, event::Event::Walk) => {
                self.bases = BaseState::Loaded;
            }
            (BaseState::FirstThird, event::Event::Single) => {
                self.bases = BaseState::FirstSecond;
                runs_scored += 1
            }
            (BaseState::FirstThird, event::Event::Double) => {
                self.bases = BaseState::SecondThird;
                runs_scored += 1;
            }
            (BaseState::FirstThird, event::Event::Triple) => {
                self.bases = BaseState::Third;
                runs_scored += 2;
            }
            (BaseState::FirstThird, event::Event::HomeRun) => {
                self.bases = BaseState::Empty;
                runs_scored += 3;
            }
            (BaseState::FirstThird, _) => {}
            (BaseState::SecondThird, event::Event::Walk) => {
                self.bases = BaseState::Loaded;
            }
            (BaseState::SecondThird, event::Event::Single) => {
                self.bases = BaseState::FirstThird;
                runs_scored += 1;
            }
            (BaseState::SecondThird, event::Event::Double) => {
                self.bases = BaseState::Second;
                runs_scored += 2;
            }
            (BaseState::SecondThird, event::Event::Triple) => {
                self.bases = BaseState::Third;
                runs_scored += 2;
            }
            (BaseState::SecondThird, event::Event::HomeRun) => {
                self.bases = BaseState::Empty;
                runs_scored += 3;
            }
            (BaseState::SecondThird, _) => {}
            (BaseState::Loaded, event::Event::Walk) => {
                self.bases = BaseState::Loaded;
                runs_scored += 1;
            }
            (BaseState::Loaded, event::Event::Single) => {
                self.bases = BaseState::Loaded;
                runs_scored += 1;
            }
            (BaseState::Loaded, event::Event::Double) => {
                self.bases = BaseState::SecondThird;
                runs_scored += 2;
            }
            (BaseState::Loaded, event::Event::Triple) => {
                self.bases = BaseState::Third;
                runs_scored += 3;
            }
            (BaseState::Loaded, event::Event::HomeRun) => {
                self.bases = BaseState::Empty;
                runs_scored += 4;
            }
            (BaseState::Loaded, _) => {}
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

    fn next_batter(&mut self) {
        let mut batter = match self.active_team {
            ActiveTeam::Away => { self.away_batter }
            ActiveTeam::Home => { self.home_batter }
        };
        batter += 1;
        if batter > 8 {
            batter = 0;
        }
        match self.active_team {
            ActiveTeam::Away => {
                self.away_batter = batter;
            }
            ActiveTeam::Home => {
                self.home_batter = batter;
            }
        }
    }

    fn check_game_over(&mut self) -> bool {
        if self.finished {
            return true;
        }
        else if self.inning < 9 {
            return false;
        }
        else {
            match self.active_team {
                ActiveTeam::Away => {}
                ActiveTeam::Home => {
                    // Home team can win immediately in the bottom of the ninth or later.
                    if self.home_score > self.away_score {
                        self.finished = true;
                        return true;
                    }
                    // Away team can only win after the inning is completely over.
                    else if self.away_score > self.home_score && self.outs == 3 {
                        self.finished = true;
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn check_next_inning(&mut self) {
        if self.finished {
            return;
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
            self.bases = BaseState::Empty;
            self.outs = 0;
        }
    }
}
