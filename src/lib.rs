mod actions;
mod public;

use crate::actions::system::{self};

pub fn start() {
    println!("Bem vindo LittleFish");
    println!("");
    system::check_system();
}
