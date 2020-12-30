use ndarray::prelude::*;

pub type StepReturn = (Array2<f32>, f32, bool);

pub trait Enviroment {
    fn reset(&mut self) -> Array2<f32>;
    fn step(&mut self, action: usize) -> StepReturn;
    fn opservation_space(&self) -> Vec<usize>;
    fn action_space(&self) -> Vec<usize>;
    fn render(&mut self);
}