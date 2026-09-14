mod tournament_table;
mod team;
mod matches;
mod participation;
mod tournament_participation;

pub use tournament_table::Score;

use crate::public;
    
pub enum TournamentAction {
    CheckTable,
    CheckTeams,
    CreateTournament,
    GoBack,
    Invalid
}

pub fn get_tournament_actions(action: u8) -> TournamentAction {
    match action {
        0 => TournamentAction::GoBack,
        1 => TournamentAction::CheckTable,
        2 => TournamentAction::CheckTeams,
        3 => TournamentAction::CreateTournament,
        _ => TournamentAction::Invalid,
    }
}

pub fn check_teams() {
    team::check_teams();
}

pub fn show_table() {
    println!("Qual campeonato deseja visualizar a tabela?");
    loop {
        let tournaments = tournament_table::get_all_tournaments();
        for (index, tournament) in tournaments.iter().enumerate() {
            println!("{}- {}", index + 1, tournament.name());
        };
        println!("0- Voltar");
        println!("");

        let action = public::get_u8_from_input();
        if action == 0 { break };

        let index = usize::from(action - 1);

        if index >= tournaments.len() {
            println!("Opção inválida. Escolha uma opção");
            continue;
        }

        let tournament = tournaments.get(index).unwrap();

        let scores = tournament_table::get_table_scores(tournament.name()).expect("Expect the tournament to have a score");
        println!("===================");
        println!("Nome da equipe---------------- Vitórias / Jogos");
        for score in scores {
            println!("{}- {} / {}", get_score_name_with_space(score.team()), score.wins(), score.participations());
        }
        println!("===================");
        break;
    }
}

fn get_score_name_with_space(team_name: &String) -> String {
    const MAX_CHAR: u8 = 29;
    let team_name_length = team_name.chars().count();
    if team_name_length == usize::from(MAX_CHAR) {
        return String::from(team_name);
    } else if team_name_length > usize::from(MAX_CHAR) {
        let mut new_string = String::with_capacity(usize::from(MAX_CHAR));
        let mut index:u8 = 0;
        for char in team_name.chars() {
            new_string.insert(usize::from(index), char);
            if index == MAX_CHAR - 3 { break };
            index += 1;
        }
        return String::from(new_string + "...");
    } else {
        let mut remaing_chars = usize::from(MAX_CHAR) - team_name_length;
        let mut new_string = String::from(team_name);
        loop {
            new_string.insert_str(team_name_length, "-");
            remaing_chars -= 1;
            if remaing_chars == 0 { break };
        }
        return new_string;
    }
}
