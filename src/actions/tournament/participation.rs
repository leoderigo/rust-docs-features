mod participation_table;

pub use participation_table::Participation;

pub fn get_participation_by_match(match_uid: String) -> Vec<Participation> {
    participation_table::get_participation_by_match(match_uid)
}

pub fn get_participation_by_team(team_name: &String) -> Vec<Participation> {
    participation_table::get_participation_by_team(&team_name)
}

pub fn get_win_participation_by_team(team_name: &String) -> Vec<Participation> {
    participation_table::get_win_participation_by_team(&team_name)
}
