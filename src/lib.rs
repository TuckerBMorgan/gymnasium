mod frozen_lake;
mod snake;
mod cartpole;
mod enviroment;

pub mod prelude {
    pub use crate::frozen_lake::*;
    pub use crate::snake::*;
    pub use crate::cartpole::*;
    pub use crate::enviroment::*;

    pub fn make(name: &String) -> Option<Box<dyn Enviroment>> {
        if name == "Cartpole" {
            return Some(Box::new(Cartpole::new()));
        }
        else if name == "FrozenLake" {
            return Some(Box::new(FrozenLake::new()));
        }

        return None;
    }
}
