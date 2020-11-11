trait Enviroment {
    fn step(&mut self, action: usize) -> usize;
    fn step(&mut self) -> usize;
}