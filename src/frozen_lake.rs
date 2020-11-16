use ndarray::prelude::*;
use std::collections::HashMap;
use rand::prelude::*;
use crate::enviroment::*;

pub struct FrozenLake {
    pub transition_diagram: HashMap<usize, [usize;4]>,
    pub what_is_state: Vec<char>,
    pub state_type_to_reward: HashMap<char, usize>,
    pub current_state: usize
}

impl FrozenLake {
    pub fn new() -> FrozenLake {
        let transition_diagram: HashMap<usize, [usize;4]> = [
        //First Row
        (0,  [0, 1, 4, 0]),
        (1,  [1, 2, 5, 0]),
        (2,  [2, 3, 6, 1]),
        (3,  [3, 3, 7, 2]),
        //Second Row
        (4,  [0, 5, 8, 4]),
        (5,  [1, 6, 9, 4]),
        (6,  [2, 7, 10, 5]),
        (7,  [3, 7, 11, 6]),
        //Third Row
        (8,  [4, 9,  12, 8]),
        (9,  [5, 10, 13, 8]),
        (10, [6, 11, 14, 9]),
        (11, [7, 11, 15, 10]),
        //Forth Row
        (12,  [8,  13,  12, 12]),
        (13,  [9,  14,  13, 8]),
        (14,  [10, 15,  14, 9]),
        (15,  [15, 15,  15, 15]),
        ].iter().cloned().collect();

        let what_is_state = vec!['S', 'F', 'F', 'F',
                             'F', 'H', 'F', 'H',
                             'F', 'F', 'F', 'H',
                             'H', 'F', 'F', 'G'];

        let state_type_to_reward : HashMap<char, usize> = [('S', 0), ('F', 0), ('H', 0), ('G', 1)].iter().cloned().collect();

        FrozenLake {transition_diagram, what_is_state, state_type_to_reward, current_state: 0}
    }
}

impl Enviroment for FrozenLake {
    fn reset(&mut self) -> Array2<f32> {
        self.current_state = 0;
        return Array2::from_shape_vec((1, 1), vec![0.0]).unwrap();
    }

    //Returns, (next_state, reward, is)
    fn step(&mut self, action: usize) -> StepReturn {
        if action > 3 {
            panic!("Bad input {} ", action);
        }

        let mut rng = rand::thread_rng();
        let num: usize = rng.gen_range(0, 100);

        //Thirty percent chance to not go where we wanted
        let picked_action;
        if num < 25 {
            let mut rng = rand::thread_rng();
            let num: usize = rng.gen_range(0, 4);
            picked_action = num;
        }
        else {
            picked_action = action;
        }

        self.current_state = self.transition_diagram[&self.current_state][picked_action];
        let is_done = self.current_state == 15 
                    || self.current_state == 5
                    || self.current_state == 7
                    || self.current_state == 11
                    || self.current_state == 12;
        return (Array2::from_shape_vec((1, 1), vec![self.current_state as f32]).unwrap(),
                self.state_type_to_reward[&self.what_is_state[self.current_state]] as f32, 
                is_done);
    }

    fn opservation_space(&self) -> Vec<usize> {
        return vec![1, 1];
    }

    fn action_space(&self) -> Vec<usize> {
        return vec![1, 4];
    }
}