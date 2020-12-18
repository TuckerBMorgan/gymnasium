use ndarray::prelude::*;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use crate::enviroment::*;


pub struct MountainCar {
    min_position: f32,
    max_position: f32,
    max_speed: f32,
    goal_position: f32,
    goal_velocity: f32,
    force: f32
}

impl MountainCar {
    pub fn new() -> MountainCar {
        MountainCar {
            min_position: -1.2,
            max_position: 0.6,
            max_speed: 0.07,
            goal_position: 0.5,
            goal_velocity: 0.0,
            force: 0.001,
            gravity: 0.0025
        }
    }
}

impl Enviroment for MountainCar {
    pub fn reset() -> Array2<f32> {

    }

    pub fn step(action: usize) -> StepReturn {
        
    }
}