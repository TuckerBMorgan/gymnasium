use ndarray::prelude::*;
use rand::prelude::*;
use crate::enviroment::*;

pub struct Acrobot {
    high: Array2<f32>,
    low: Array2<f32>,
    state: Array2<f32>
}

impl Enviroment for Acrobot {
    pub fn reset(&mut self) -> Array2<f32> {
        let mut rng = thread_rng();
        let side = Uniform::new(-0.1, 0.1);
        return Array2::from_shape_vec((1, 4), vec![rng.sample(side), rng.sample(side), rng.sample(side), rng.sample(side)]).unwrap();
    }

    pub fn step(&mut self, action: usize) -> StepReturn {
        
    }
}