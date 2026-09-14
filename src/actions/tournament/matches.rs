mod matches_table;

use matches_table::{Match};
use super::participation::Participation;
use super::Score;

pub fn find_matches_by_tournament(tournament_name: String) -> Vec<(Match, Vec<Participation>)> {
    matches_table::get_matches_by_tournament(tournament_name)
}

pub fn get_score_from_tournament(team_name: &String, tournament_name: &String) -> Score {
    matches_table::get_score_from_tournament(team_name, tournament_name)
}
