use std::io::{Write, stdin, stdout};

mod member_table;
use member_table::Member;

pub fn list_members() {
    let member_list = member_table::get_members();

    println!("==============");
    for (index, member) in member_list.iter().enumerate() {
        println!("{}- {} ({})", index + 1, member.nickname, member.email);
    }
    println!("==============");
}

pub fn add_member() {
    let mut new_member: Member = Member { nickname: String::new(), email: String::new(), team: None };
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
    loop {
        let mut team: String = String::new();
        print!("Qual a equipe do membro? ");
        match stdout().flush() {
            Err(_) => continue,
            _ => ()
        };
        match stdin().read_line(&mut team) {
            Err(_) => continue,
            _ => ()
        };
        team = String::from(team.trim());
        if "0".as_bytes() == team.as_bytes() {
            return;
        }
        
        new_member.team = if team.len() == 0 { None } else { Some(team) };
        break;
    }

    member_table::insert_member(new_member.nickname, new_member.email, new_member.team);
}

pub fn remove_member() {
    loop {
        print!("Para remover o membro, digite o email: ");
        if let Err(_) = stdout().flush() {
            println!("Erro no flush");
            break;
        }

        let mut email: String = String::new();
        if let Err(_) = stdin().read_line(&mut email) {
            println!("Erro ao ler email");
        };

        email = String::from(email.trim());

        member_table::delete_member(email);
        
        break;
    }
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
