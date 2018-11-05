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
    First(u8),
    Second(u8),
    Third(u8),
    FirstSecond(u8, u8),
    FirstThird(u8, u8),
    SecondThird(u8, u8),
    Loaded(u8, u8, u8),
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
        match event {
            event::Event::Out | event::Event::Flyout | event::Event::Groundout | event::Event::Strikeout => {
                self.outs += 1;
            }
            _ => {
                let (runs_scored, next_base_state) = self.batter_bases(event);
                self.bases = next_base_state;
                self.add_runs(runs_scored);
            }
        }

        self.next_batter();
    }

    fn batter_bases(&mut self, event: event::Event) -> (u8, BaseState) {
        let mut runs_scored = 0;
        let mut next_base_state = BaseState::Empty;

        let batter = self.active_batter();
        match (&self.bases, event) {
            (BaseState::Empty, event::Event::Walk) => {
                next_base_state = BaseState::First(batter);
            }
            (BaseState::Empty, event::Event::Single) => {
                next_base_state = BaseState::First(batter);
            }
            (BaseState::Empty, event::Event::Double) => {
                next_base_state = BaseState::Second(batter);
            }
            (BaseState::Empty, event::Event::Triple) => {
                next_base_state = BaseState::Third(batter);
            }
            (BaseState::Empty, event::Event::HomeRun) => {
                next_base_state = BaseState::Empty;
                runs_scored += 1;
            }
            (BaseState::Empty, _) => {}
            (BaseState::First(runner1), event::Event::Walk) => {
                next_base_state = BaseState::FirstSecond(batter, *runner1);
            }
            (BaseState::First(runner1), event::Event::Single) => {
                next_base_state = BaseState::FirstSecond(batter, *runner1);
            }
            (BaseState::First(runner1), event::Event::Double) => {
                next_base_state = BaseState::SecondThird(batter, *runner1);
            }
            (BaseState::First(_), event::Event::Triple) => {
                next_base_state = BaseState::Third(batter);
                runs_scored += 1;
            }
            (BaseState::First(_), event::Event::HomeRun) => {
                next_base_state = BaseState::Empty;
                runs_scored += 2;
            }
            (BaseState::First(_), _) => {}
            (BaseState::Second(runner2), event::Event::Walk) => {
                next_base_state = BaseState::FirstSecond(batter, *runner2);
            }
            (BaseState::Second(runner2), event::Event::Single) => {
                next_base_state = BaseState::FirstThird(batter, *runner2);
            }
            (BaseState::Second(runner2), event::Event::Double) => {
                next_base_state = BaseState::Second(batter);
                runs_scored += 1;
            }
            (BaseState::Second(_), event::Event::Triple) => {
                next_base_state = BaseState::Third(batter);
                runs_scored += 1;
            }
            (BaseState::Second(_), event::Event::HomeRun) => {
                next_base_state = BaseState::Empty;
                runs_scored += 2;
            }
            (BaseState::Second(_), _) => {}
            (BaseState::Third(runner3), event::Event::Walk) => {
                next_base_state = BaseState::FirstThird(batter, *runner3);
            }
            (BaseState::Third(runner3), event::Event::Single) => {
                next_base_state = BaseState::First(batter);
                runs_scored += 1;
            }
            (BaseState::Third(runner3), event::Event::Double) => {
                next_base_state = BaseState::Second(batter);
                runs_scored += 1;
            }
            (BaseState::Third(_), event::Event::Triple) => {
                next_base_state = BaseState::Third(batter);
                runs_scored += 1;
            }
            (BaseState::Third(_), event::Event::HomeRun) => {
                next_base_state = BaseState::Empty;
                runs_scored += 2;
            }
            (BaseState::Third(_), _) => {}
            (BaseState::FirstSecond(runner1, runner2), event::Event::Walk) => {
                next_base_state = BaseState::Loaded(batter, *runner1, *runner2);
            }
            (BaseState::FirstSecond(runner1, runner2), event::Event::Single) => {
                next_base_state = BaseState::Loaded(batter, *runner1, *runner2);
            }
            (BaseState::FirstSecond(runner1, runner2), event::Event::Double) => {
                next_base_state = BaseState::SecondThird(batter, *runner1);
                runs_scored += 1;
            }
            (BaseState::FirstSecond(_, _), event::Event::Triple) => {
                next_base_state = BaseState::Third(batter);
                runs_scored += 2;
            }
            (BaseState::FirstSecond(_, _), event::Event::HomeRun) => {
                next_base_state = BaseState::Empty;
                runs_scored += 3;
            }
            (BaseState::FirstSecond(_, _), _) => {}
            (BaseState::FirstThird(runner1, runner3), event::Event::Walk) => {
                next_base_state = BaseState::Loaded(batter, *runner1, *runner3);
            }
            (BaseState::FirstThird(runner1, _), event::Event::Single) => {
                next_base_state = BaseState::FirstSecond(batter, *runner1);
                runs_scored += 1
            }
            (BaseState::FirstThird(runner1, _), event::Event::Double) => {
                next_base_state = BaseState::SecondThird(batter, *runner1);
                runs_scored += 1;
            }
            (BaseState::FirstThird(_, _), event::Event::Triple) => {
                next_base_state = BaseState::Third(batter);
                runs_scored += 2;
            }
            (BaseState::FirstThird(_, _), event::Event::HomeRun) => {
                next_base_state = BaseState::Empty;
                runs_scored += 3;
            }
            (BaseState::FirstThird(_, _), _) => {}
            (BaseState::SecondThird(runner2, runner3), event::Event::Walk) => {
                next_base_state = BaseState::Loaded(batter, *runner2, *runner3);
            }
            (BaseState::SecondThird(runner2, _), event::Event::Single) => {
                next_base_state = BaseState::FirstThird(batter, *runner2);
                runs_scored += 1;
            }
            (BaseState::SecondThird(_, _), event::Event::Double) => {
                next_base_state = BaseState::Second(batter);
                runs_scored += 2;
            }
            (BaseState::SecondThird(_, _), event::Event::Triple) => {
                next_base_state = BaseState::Third(batter);
                runs_scored += 2;
            }
            (BaseState::SecondThird(_, _), event::Event::HomeRun) => {
                next_base_state = BaseState::Empty;
                runs_scored += 3;
            }
            (BaseState::SecondThird(_, _), _) => {}
            (BaseState::Loaded(runner1, runner2, _), event::Event::Walk) => {
                next_base_state = BaseState::Loaded(batter, *runner1, *runner2);
                runs_scored += 1;
            }
            (BaseState::Loaded(runner1, runner2, _), event::Event::Single) => {
                next_base_state = BaseState::Loaded(batter, *runner1, *runner2);
                runs_scored += 1;
            }
            (BaseState::Loaded(runner1, _, _), event::Event::Double) => {
                next_base_state = BaseState::SecondThird(batter, *runner1);
                runs_scored += 2;
            }
            (BaseState::Loaded(_, _, _), event::Event::Triple) => {
                next_base_state = BaseState::Third(batter);
                runs_scored += 3;
            }
            (BaseState::Loaded(_, _, _), event::Event::HomeRun) => {
                next_base_state = BaseState::Empty;
                runs_scored += 4;
            }
            (BaseState::Loaded(_, _, _), _) => {}
        }

        return (runs_scored, next_base_state);
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
        let mut batter = self.active_batter();
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

    fn active_batter(&self) -> u8 {
        match self.active_team {
            ActiveTeam::Away => { self.away_batter }
            ActiveTeam::Home => { self.home_batter }
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
