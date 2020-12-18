mod frozen_lake;
mod snake;
mod cartpole;
mod enviroment;
mod acrobot;
//mod mountain_car;

pub mod prelude {
    pub use crate::frozen_lake::*;
    pub use crate::snake::*;
    pub use crate::cartpole::*;
    pub use crate::enviroment::*;
    pub use crate::acrobot::*;

    pub fn make(name: &String) -> Option<Box<dyn Enviroment>> {
        if name == "Cartpole" {
            return Some(Box::new(Cartpole::new()));
        }
        else if name == "FrozenLake" {
            return Some(Box::new(FrozenLake::new()));
        }
        else if name == "Acrobot" { 
            return Some(Box::new(Acrobot::new()));
        }
        /*
        else if name == "MountainCar" {
            return Some(Box::new(MountainCar::new()));
        }
        */

        return None;
    }
}
