use std::cmp::Ordering;
// There are faster hashing functions other than Rust's built-ins
// Both fnv and fx could be good alternatives, but this should be good enough
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Outcome<'a> {
    WINLOSS((&'a str, &'a str)), // tuple of winner, loser
    DRAW((&'a str, &'a str)),
}

// Refactor-NOTE
// Instead of handling Strings for team names, we could use a hashbag for space-savings.
// Scores could also be made up of more detailed data, such as vectors of tuples of (playername, minute scored).

pub struct Game {
    home_name: String,
    home_score: u8,
    away_name: String,
    away_score: u8,
}

impl Game {
    // Refactor-TODO: implement FromStr Trait instead
    pub fn from_str(raw: &str) -> Result<Game, String> {
        // NOTE: assuming "{home name} {home score}, {away name} {away score}" format.
        // If the input format cannot be guaranteed, this will be the place to adjust.
        let v: Vec<&str> = raw.split(", ").collect();
        if v.len() != 2 {
            return Err(format!("No game data found in line {}", raw));
        }
        let h: Vec<&str> = v[0].rsplitn(2, ' ').collect();
        let a: Vec<&str> = v[1].rsplitn(2, ' ').collect();
        Ok(Game {
            home_name: h[1].to_string(),
            home_score: h[0].parse().unwrap(),
            away_name: a[1].to_string(),
            away_score: a[0].parse().unwrap(),
        })
    }

    pub fn outcome(&self) -> Outcome {
        match self.home_score.cmp(&self.away_score) {
            Ordering::Greater => Outcome::WINLOSS((&self.home_name, &self.away_name)),
            Ordering::Less => Outcome::WINLOSS((&self.away_name, &self.home_name)),
            Ordering::Equal => Outcome::DRAW((&self.home_name, &self.away_name)),
        }
    }
}

#[derive(Debug)]
pub struct Standings {
    teams_with_points: HashMap<String, u8>,
    tmp_teams_with_games: HashSet<String>, // temporary set to determine whether a new matchday has started
    // (we're expexting to have every team play once during a matchday)
    win_points: u8,   // points the winner gets
    draw_points: u8,  // points for a draw for both teams,
    print_top: usize, // prints the top-ranking n teams
    matchday: usize,  // current matchday
}

impl Default for Standings {
    fn default() -> Self {
        Standings {
            teams_with_points: Default::default(),
            tmp_teams_with_games: Default::default(),
            win_points: 3,
            draw_points: 1,
            print_top: 3,
            matchday: 1,
        }
    }
}

impl Standings {
    pub fn new(win_points: u8, draw_points: u8, print_top: usize) -> Standings {
        Standings {
            win_points,
            draw_points,
            print_top,
            ..Default::default()
        }
    }

    pub fn print_rankings(&self) {
        if !self.teams_with_points.is_empty() {
            let mut v: Vec<(&String, &u8)> = self.teams_with_points.iter().collect();
            v.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
            println!("Matchday {}", self.matchday);
            for item in v.iter().take(self.print_top) {
                println!("{}, {} pt{}", item.0, item.1, pluralize(*item.1));
            }
        }
    }

    pub fn ingest(&mut self, game: Game) {
        // check if a new matchday has started
        if self.tmp_teams_with_games.contains(&game.home_name)
            || self.tmp_teams_with_games.contains(&game.away_name)
        {
            // it's a new day!
            self.print_rankings();
            println!(); // separator between matchdays, but not at the end of program
            self.tmp_teams_with_games.clear();
            self.matchday += 1;
        }

        match game.outcome() {
            Outcome::WINLOSS((winner, loser)) => {
                self.add_points_to_team(winner, self.win_points);
                self.add_points_to_team(loser, 0); // important if printing of rankings cannot be filled by teams who have earned wins
            }
            Outcome::DRAW((home, away)) => {
                self.add_points_to_team(home, self.draw_points);
                self.add_points_to_team(away, self.draw_points);
            }
        }

        // add both teams to seen teams for current matchday
        self.tmp_teams_with_games.insert(game.home_name);
        self.tmp_teams_with_games.insert(game.away_name);
    }

    fn add_points_to_team(&mut self, name: &str, points: u8) {
        let p = self.teams_with_points.entry(name.to_string()).or_insert(0);
        *p += points;
    }
}

fn pluralize<'a>(n: u8) -> &'a str {
    match n {
        1 => "",
        _ => "s",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_from_str_works() {
        let line = "San Jose Earthquakes 3, Santa Cruz Slugs 3";
        let game = Game::from_str(line).unwrap();
        assert_eq!(game.home_name, "San Jose Earthquakes");
        assert_eq!(game.away_name, "Santa Cruz Slugs");
        assert_eq!(game.home_score, 3);
        assert_eq!(game.away_score, 3);
    }

    #[test]
    fn outcome_draw_works() {
        let line = "San Jose Earthquakes 3, Santa Cruz Slugs 3";
        let game = Game::from_str(line).unwrap();
        assert_eq!(
            game.outcome(),
            Outcome::DRAW(("San Jose Earthquakes", "Santa Cruz Slugs"))
        );
    }

    #[test]
    fn outcome_home_win_works() {
        let line = "Capitola Seahorses 1, Aptos FC 0";
        let game = Game::from_str(line).unwrap();
        assert_eq!(
            game.outcome(),
            Outcome::WINLOSS(("Capitola Seahorses", "Aptos FC"))
        );
    }

    #[test]
    fn outcome_away_win_works() {
        let line = "San Jose Earthquakes 1, Felton Lumberjacks 4";
        let game = Game::from_str(line).unwrap();
        assert_eq!(
            game.outcome(),
            Outcome::WINLOSS(("Felton Lumberjacks", "San Jose Earthquakes"))
        );
    }

    #[test]
    fn standings_ingest_works() {
        let mut standings = Standings::default();
        assert_eq!(standings.teams_with_points.len(), 0);
        standings.ingest(Game::from_str("San Jose Earthquakes 3, Santa Cruz Slugs 3").unwrap());
        assert_eq!(standings.matchday, 1);
        assert_eq!(standings.teams_with_points.len(), 2);
        standings.ingest(Game::from_str("Capitola Seahorses 1, Aptos FC 0").unwrap());
        standings.ingest(Game::from_str("Felton Lumberjacks 2, Monterey United 0").unwrap());
        standings.ingest(Game::from_str("Felton Lumberjacks 1, Aptos FC 2").unwrap());
        standings.ingest(Game::from_str("Santa Cruz Slugs 0, Capitola Seahorses 0").unwrap());
        standings.ingest(Game::from_str("Monterey United 4, San Jose Earthquakes 2").unwrap());
        standings.ingest(Game::from_str("Santa Cruz Slugs 2, Aptos FC 3").unwrap());
        standings.ingest(Game::from_str("San Jose Earthquakes 1, Felton Lumberjacks 4").unwrap());
        standings.ingest(Game::from_str("Monterey United 1, Capitola Seahorses 0").unwrap());
        standings.ingest(Game::from_str("Aptos FC 2, Monterey United 0").unwrap());
        standings.ingest(Game::from_str("Capitola Seahorses 5, San Jose Earthquakes 5").unwrap());
        standings.ingest(Game::from_str("Santa Cruz Slugs 1, Felton Lumberjacks 1").unwrap());
        assert_eq!(standings.matchday, 4);
        assert_eq!(standings.teams_with_points.len(), 6);
        assert_eq!(standings.teams_with_points.get("Aptos FC"), Some(&9));
        assert_eq!(
            standings.teams_with_points.get("Felton Lumberjacks"),
            Some(&7)
        );
        assert_eq!(standings.teams_with_points.get("Monterey United"), Some(&6));
        assert_eq!(standings.teams_with_points.get("FC St. Pauli"), None);
    }
}
