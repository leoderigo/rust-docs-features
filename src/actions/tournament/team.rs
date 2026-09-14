mod team_table;

pub use team_table::Team;

use crate::public;

fn team_exists(name: String) -> bool {
    team_table::team_exists(name)
}

pub fn get_team(name: &String) -> Option<Team> {
    team_table::get_team(name)
}

pub fn get_team_actions(action: u8) -> TeamAction {
    match action {
        1 => TeamAction::CreateTeam,
        2 => TeamAction::DeleteTeam,
        3 => TeamAction::AddToTeam,
        4 => TeamAction::RemoveFromTeam,
        0 => TeamAction::GoBack,
        _ => TeamAction::Invalid,
    }
}

fn add_member() {

}

fn create_team() {

}

fn delete_team() {

}

fn remove_member() {
    
}

pub fn check_teams() {
    loop {
        println!("1- Criar nova equipe");
        println!("2- Apagar equipe");
        println!("3- Adicionar membro à equipe");
        println!("4- Apagar membro da equipe");
        println!("");

        let action = public::get_u8_from_input();
        let action = get_team_actions(action);

        match action {
            TeamAction::AddToTeam => add_member(),
            TeamAction::CreateTeam => create_team(),
            TeamAction::DeleteTeam => delete_team(),
            TeamAction::RemoveFromTeam => remove_member(),
            TeamAction::GoBack => break,
            TeamAction::Invalid => continue
        }
    }
}

pub enum TeamAction {
    CreateTeam,
    DeleteTeam,
    AddToTeam,
    RemoveFromTeam,
    GoBack,
    Invalid,
}
