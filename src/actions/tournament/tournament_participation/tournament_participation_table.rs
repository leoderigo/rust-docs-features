use serde::{Deserialize, Serialize};
use crate::public;

const PATH_TABLE: &str = "database/tournament_participation.json";

pub fn get_by_tournament(tournament_name: &String) -> Vec<TournamentParticipation> {
    let mut full_data= public::get_serialized_full_table::<TournamentParticipation>(PATH_TABLE);
    full_data.retain(|x| x.tournament_name == *tournament_name);

    full_data
}

#[derive(Deserialize, Serialize)]
pub struct TournamentParticipation {
    team_name: String,
    tournament_name: String
}

impl TournamentParticipation {
    pub fn team_name(&self) -> &String {
        &self.team_name
    }

    pub fn tournament_name(&self) -> &String {
        &self.tournament_name
    }
}
