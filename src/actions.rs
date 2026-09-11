mod members;


pub mod system {
    use crate::public;
    use crate::actions::members::{self, MembersAction};

    pub enum SystemActions {
        Exit,
        Invalid,
        CheckMembers,
    }

    pub fn get_system_action(action: u8) -> SystemActions {
        match action {
            2 => SystemActions::CheckMembers,
            0 => SystemActions::Exit,
            _ => SystemActions::Invalid
        }
    }

    pub fn check_system() {
        loop {
            println!("Bem vindo LittleFish");
            println!("");
            println!("1- Verificar campeonato");
            println!("2- Verificar membros");
            println!("0- Sair");
            println!("");

            let action: u8 = public::get_u8_from_input();

            let action: SystemActions = get_system_action(action);

            match action {
                SystemActions::CheckMembers => check_members(),
                SystemActions::Exit => break,
                SystemActions::Invalid => continue
            };
        }
    }

    pub fn check_members() {
        loop {
            println!("1- Listar membros");
            println!("2- Adicionar membro");
            println!("3- Remover membro");
            println!("0- Voltar");
            println!("");
        
            let mut should_go_back = false;
            
            let action: u8 = public::get_u8_from_input();
            println!("");
            let action = members::get_members_action(action);

            match action {
                MembersAction::List => members::list_members(),
                MembersAction::Add => members::add_member(),
                MembersAction::GoBack => should_go_back = true,
                _ => ()
            };

            if should_go_back {
                break;
            }
        }
    }
}
