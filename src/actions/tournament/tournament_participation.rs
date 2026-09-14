mod tournament_participation_table;
use tournament_participation_table::{TournamentParticipation};
use super::team::{self, Team};

pub fn get_by_tournament(tournament_name: &String) -> Vec<TournamentParticipation> {
    tournament_participation_table::get_by_tournament(tournament_name)
}

pub fn get_teams_in_tournament(tournament_name: &String) -> Vec<Team> {
    let participations = tournament_participation_table::get_by_tournament(tournament_name);
    let mut teams: Vec<Team> = Vec::with_capacity(participations.len());
    for participation in participations {
        if let Some(team) = team::get_team(participation.team_name()) {
            teams.push(team);
            continue;
        }
        panic!("Could not find team {} with a participation in the tournament ({})", participation.team_name(), tournament_name);
    }

    teams
}
