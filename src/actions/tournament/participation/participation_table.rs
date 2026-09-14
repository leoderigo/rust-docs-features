use crate::public;
use serde::{Deserialize, Serialize};

const PATH_TABLE: &str = "database/participation.json";

pub fn get_participation_by_match(match_uid: String) -> Vec<Participation> {
    let mut participations = public::get_serialized_full_table::<Participation>(PATH_TABLE);

    participations.retain(|x| x.match_uid == match_uid);
    participations
}

pub fn get_participation_by_team(team_name: &String) -> Vec<Participation> {
    let mut participations = public::get_serialized_full_table::<Participation>(PATH_TABLE);

    participations.retain(|x| x.team_name == *team_name);
    participations
}

pub fn get_win_participation_by_team(team_name: &String) -> Vec<Participation> {
    let mut participations = public::get_serialized_full_table::<Participation>(PATH_TABLE);

    participations.retain(|x| x.team_name == *team_name && x.place == 1);
    participations
}

#[derive(Deserialize, Serialize)]
pub struct Participation {
    match_uid: String,
    kills: u32,
    place: u8,
    team_name: String
}

impl Participation {
    pub fn is_a_win(&self) -> bool {
        self.place == 1
    }

    pub fn game(&self) -> &String {
        &self.match_uid
    }
}
