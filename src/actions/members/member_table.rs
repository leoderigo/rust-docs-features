use std::fs;
use serde_json;
use serde::{Deserialize, Serialize};

const MEMBER_TABLE_PATH: &str = "database/member.json";

#[derive(Deserialize, Serialize)]
pub struct Member {
    pub nickname: String,
    pub email: String,
    pub team: Option<String>
}

pub fn get_members() -> Vec<Member> {
    let result = fs::read_to_string(MEMBER_TABLE_PATH);

    match result {
        Ok(result) => {
            let members: Vec<Member> = match serde_json::from_str(&result) {
                Ok(result) => result,
                _ => {
                    println!("Não foi possível converter os dados");
                    vec![]
                }
            };

            return members;
        },
        Err(err) => {
            println!("Não foi possível ler os dados");
            println!("===");
            println!("{}", err);
            println!("===");
            return vec![];
        }
    }
}

pub fn insert_member(nickname: String, email: String, team: Option<String>) {
    let mut members = get_members();
    let new_member = Member { email: email, nickname: nickname, team: team };

    let found = members.iter().enumerate().find(|item| item.1.email == new_member.email);
    match found {
        Some((index, _)) => {
            members[index] = new_member;
        },
        None => members.push(new_member)
    };

    match serde_json::to_string(&members) {
        Ok(text) => {
            match fs::write(MEMBER_TABLE_PATH, text) {
                Err(err) => println!("Não foi possível escrever no json: {}", err),
                _ => ()
            };
        },
        Err(err) => println!("Falha ao salvar: {}", err)
    };
}

pub fn delete_member(email: String) {
    let mut members = get_members();
    let mut found_index: Option<usize> = None;
    let mut found_nickname: Option<String> = None;
    for (index, member) in members.iter().enumerate() {
        if member.email != email { continue };

        found_index = Some(index);
        found_nickname = Some(member.nickname.clone());
        break;
    }
    
    if let None = found_index {
        println!("Não há membros com esse email");
        return;
    }

    members.swap_remove(found_index.unwrap());
    match serde_json::to_string(&members) {
        Ok(serialized) => {
            match fs::write(MEMBER_TABLE_PATH, serialized) {
                Ok(_) => println!("{} removido da lista de membros", found_nickname.unwrap()),
                Err(err) => println!("Não foi possível salvar a operação: {}", err)
            }
        },
        Err(err) => {
            println!("Não foi possível serializar o novo valor: {}", err)
        }
    }
}
