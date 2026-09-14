use serde::{Deserialize, Serialize};
use crate::public;
use crate::{public::Year};
use super::matches;
use super::tournament_participation;

const PATH_TABLE: &str = "database/tournaments.json";

pub fn get_all_tournaments() -> Vec<Tournament> {
    get_full_data()
}

pub fn get_table_scores(tournament_name: &String) -> Result<Vec<Score>, &str> {
    let tournaments: Vec<Tournament> = get_full_data();
    let mut found_tournament = None;
    for tournament in tournaments {
        if tournament.name != *tournament_name { continue };
        found_tournament = Some(tournament);
        break;
    }

    if found_tournament.is_none() {
        return Err("Tournament not found");
    }
    let tournament = found_tournament.unwrap();

    let teams_in_tournament = tournament_participation::get_teams_in_tournament(&tournament.name);
    let mut scores: Vec<Score> = Vec::with_capacity(teams_in_tournament.len());

    for team in teams_in_tournament {
        scores.push(matches::get_score_from_tournament(team.name(), &tournament.name));
    };

    Ok(scores)
}

fn get_full_data() -> Vec<Tournament> {
    public::get_serialized_full_table::<Tournament>(PATH_TABLE)
}

#[derive(Deserialize, Serialize)]
pub struct Tournament {
    name: String,
    status: TournamentStatus
}

impl Tournament {
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all="lowercase")]
enum TournamentStatus {
    Completed(Year),
    OnGoing,
    NotStarted
}

pub struct Score {
    team: String,
    wins: u32,
    participations: u32
}

impl Score {
    pub fn team(&self) -> &String {
        &self.team
    }

    pub fn participations(&self) -> u32 {
        self.participations
    }

    pub fn wins(&self) -> u32 {
        self.wins
    }

    pub fn new(team: String, participations: u32, wins: u32) -> Self {
        Self { team, participations, wins }
    }
}
