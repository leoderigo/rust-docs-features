use serde::{Deserialize, Serialize};
use crate::public;
use crate::actions::tournament::participation;
use super::super::participation::Participation;
use super::super::Score;

const PATH_TABLE: &str = "database/matches.json";

pub fn get_matches_by_tournament(tournament_name: String) -> Vec<(Match, Vec<Participation>)> {
    let mut matches = get_full_data();

    let mut result: Vec<(Match, Vec<Participation>)> = Vec::with_capacity(matches.len());

    matches.retain(|x| x.tournament == tournament_name);

    for game in matches {
        let game_uid = game.uid.clone();
        result.push((game, participation::get_participation_by_match(game_uid)));
    }

    result
}

pub fn get_score_from_tournament(team_name: &String, tournament_name: &String) -> Score {
    let mut matches = get_full_data();
    matches.retain(|x| x.status == MatchStatus::Completed && x.tournament == *tournament_name);
    let mut wins: u32 = 0;
    let mut games: u32 = 0;
    for game in matches {
        let mut participations = participation::get_participation_by_team(team_name);
        participations.retain(|x| *x.game() == game.uid);
        for participation in participations {
            games += 1;
            if participation.is_a_win() { wins += 1 }
        };
    };

    Score::new(team_name.clone(), games, wins)
}

fn get_full_data() -> Vec<Match> {
    public::get_serialized_full_table::<Match>(PATH_TABLE)
}

#[derive(Deserialize, Serialize)]
pub struct Match {
    uid: String,
    status: MatchStatus,
    victoryreason: VictoryReason,
    tournament: String
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all="lowercase")]
enum MatchStatus {
    Completed,
    OnGoing,
    Waiting
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum VictoryReason {
    Battle,
    WO
}
