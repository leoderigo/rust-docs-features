use serde::{Deserialize, Serialize};
use crate::public;

const PATH_TABLE: &str = "database/team.json";

pub fn team_exists(name: String) -> bool {
    let teams: Vec<Team> = get_full_data();

    for team in teams {
        if team.name != name { continue }
        return true
    }

    false
}

pub fn get_team(name: &String) -> Option<Team> {
    for team in get_full_data() {
        if team.name != *name { continue };
        return Some(team);
    }

    None
}

fn get_full_data() -> Vec<Team> {
    public::get_serialized_full_table::<Team>(PATH_TABLE)
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    name: String
}

impl Team {
    pub fn name(&self) -> &String {
        &self.name
    }
}

