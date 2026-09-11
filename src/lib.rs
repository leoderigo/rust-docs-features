mod actions;
mod public;

use crate::actions::system::{self};

pub fn start() {
    system::check_system();
}
