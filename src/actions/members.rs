use std::fs;
use std::io::{Write, stdin, stdout};
use serde_json;
use serde::{Deserialize, Serialize};

const MEMBER_TABLE_PATH: &str = "database/member.json";

fn get_members() -> Vec<Member> {
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

fn insert_member(nickname: String, email: String) {
    let mut members = get_members();
    let new_member = Member { email: email, nickname: nickname };

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

#[derive(Deserialize, Serialize)]
struct Member {
    nickname: String,
    email: String,
}

pub fn list_members() {
    let member_list = get_members();

    println!("==============");
    for (index, member) in member_list.iter().enumerate() {
        println!("{}- {} ({})", index + 1, member.nickname, member.email);
    }
    println!("==============");
}

pub fn add_member() {
    let mut new_member: Member = Member { nickname: String::new(), email: String::new() };
    loop {
        let mut nickname: String = String::new();
        print!("Qual o nickname do membro? ");
        match stdout().flush() {
            Err(_) => continue,
            _ => ()
        };
        match stdin().read_line(&mut nickname) {
            Err(_) => continue,
            _ => ()
        };
        nickname = String::from(nickname.trim());
        if "0".as_bytes() == nickname.as_bytes() {
            return;
        }
        new_member.nickname = nickname;
        break;
    }
    loop {
        let mut email: String = String::new();
        print!("Qual o email do membro? ");
        match stdout().flush() {
            Err(_) => continue,
            _ => ()
        };
        match stdin().read_line(&mut email) {
            Err(_) => continue,
            _ => ()
        };
        email = String::from(email.trim());
        if "0".as_bytes() == email.as_bytes() {
            return;
        }
        new_member.email = email;
        break;
    }

    insert_member(new_member.nickname, new_member.email);
}

pub enum MembersAction {
    List,
    Add,
    Delete,
    GoBack,
    Invalid,
}


pub fn get_members_action(action: u8) -> MembersAction {
    match action {
        1 => MembersAction::List,
        2 => MembersAction::Add,
        3 => MembersAction::Delete,
        0 => MembersAction::GoBack,
        _ => MembersAction::Invalid,
    }
}
