use std::collections::*;

enum BoardState {
    snake,
    food,
    empty
}

pub struct Snake {
    board: HashMap<usize, BoardState>
}

impl Snake {

    pub fn new(board_size: usize) -> Snake {
        let number_of_tiles = board_size * board_size;
        let mut board = HashMap::<usize, BoardState>::new();
        
        for i in 0..number_of_tiles {
            board[i] = BoardState::empty;
        }

        //generate two unqiue indexes for the food and snake start

        Snake {
            board
        }
    }

    pub fn reset(&mut self) -> usize {
        
    }

    //Returns, (next_state, reward, is)
    pub fn step(&mut self, action: usize) -> Result<(usize, f32, bool), &'static str> {

    }
}