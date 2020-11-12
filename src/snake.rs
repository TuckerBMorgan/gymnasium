use std::collections::*;
use rand::prelude::*;
enum BoardState {
    Snake,
    Food,
    Empty
}

pub struct Snake {
    board: HashMap<usize, BoardState>,
    number_of_tiles: usize,
    board_size: usize
}

impl Snake {

    pub fn new(board_size: usize) -> Snake {
        let number_of_tiles = board_size * board_size;
        let mut board = HashMap::<usize, BoardState>::new();
        let mut rng = thread_rng();

        for i in 0..number_of_tiles {
            board.insert(i, BoardState::Empty);
        }

        //Man does this feel hacky
        let snake_index = rng.gen_range(0, number_of_tiles);
        let mut food_index = snake_index;
        while food_index == snake_index {
            food_index = rng.gen_range(0, number_of_tiles);
        }

        board.insert(snake_index, BoardState::Snake);
        board.insert(snake_index, BoardState::Food);
        //generate two unqiue indexes for the food and snake start

        Snake {
            board,
            number_of_tiles,
            board_size
        }
    }

    fn setup_snake(&mut self) {
        for i in 0..self.number_of_tiles {
            self.board.insert(i, BoardState::Empty);
        }
    }

    /// This assumes that you already have a snake tile set
    fn place_food(&mut self, snake_index: usize) {
        let mut new_food_pos = snake_index;
        let mut rng = thread_rng();
        while new_food_pos == snake_index {
            new_food_pos = rng.gen_range(0, self.number_of_tiles);
        }
        self.board.insert(new_food_pos, BoardState::Food);
    }

    fn place_food_and_snake(&mut self) {
        let mut rng = thread_rng();
        let snake_index = rng.gen_range(0, self.number_of_tiles);
        self.place_food(snake_index);
        self.board.insert(snake_index, BoardState::Snake);
    }

    fn oned_into_twod_index(index: usize, board_size: usize) -> (usize, usize) {
        return (index / board_size, index % board_size);
    }

    // Returns a cardinal direction from the perspective of the snake as to where the food is
    // Will roll a random die when the food is a perfect half way between two direction, NE, SE, SW, NW
    // Else it will return which direction is the shortest distance
    // Returning 0 - 3, starting with 0 for up and going in a clockwise direction
    // 0 is up, 1 if right, 2 is down, 3 is left
    fn where_is_food(&self) -> usize {
        let mut index_of_food = 0;
        let mut index_of_snake = 0;
        for (k, v) in self.board.iter() {
            match v {
                BoardState::Snake => {
                    index_of_snake = *k;
                },
                BoardState::Food => {
                    index_of_food = *k;
                },
                BoardState::Empty => {
                    continue;
                }
            }
        }

        let snake_pos = Snake::oned_into_twod_index(index_of_snake, self.board_size);
        let food_pos = Snake::oned_into_twod_index(index_of_food, self.board_size);

        // If they are in the same row
        if snake_pos.0 == food_pos.0 {
            //Food is above the snake
            if food_pos.1 > snake_pos.1 {
                return 1;
            }
            //Food is below the snake
            else {
                return 3;
            }

        }
        else {
            //Food is above the snake
            if food_pos.0 > snake_pos.0 {
                return 0;
            }
            //Food is below the snake
            else {
                return 2;
            }
        }
    }

    pub fn where_snake(&self) -> usize {
        let mut index_of_snake = 0;
            for (k, v) in self.board.iter() {
                match v {
                    BoardState::Snake => {
                        index_of_snake = *k;
                    },
                    BoardState::Food => {
                        continue;
                    },
                    BoardState::Empty => {
                        continue;
                    }
                }
            }
            return index_of_snake;
    }

    pub fn reset(&mut self) -> usize {
        self.setup_snake();
        self.place_food_and_snake();
        return 0;
    }

    pub fn twod_to_oned(indexes: (isize, isize), board_size: usize) -> usize {
        return (indexes.0 as usize * board_size) + indexes.1 as usize;
    }

    pub fn process_action(&mut self, action: usize) -> f32 {
        let snake_index = self.where_snake();
        let as_two_d = Snake::oned_into_twod_index(snake_index, self.number_of_tiles);

        //Convert the usize into a isize so I can go into negative numbers
        //Then I can check to see if the desired position would violate any bounds
        //and not update the position if that is true
        let mut as_two_d = (as_two_d.0 as isize, as_two_d.1 as isize);
        if action == 0 {
            //Move up
            as_two_d.0 += 1;
        }
        else if action == 1 {
            //Move right
            as_two_d.1 += 1;
        }
        else if action == 2 {
            //Move down
            as_two_d.0 -= 1;
        }
        else if action == 3 {
            //move left
            as_two_d.1 -= 1;
        }

        if as_two_d.0 < 0 || as_two_d.1 < 0 {
            //Bad new index don't move
            return 0.0f32;
        }
        else {
            let index = Snake::twod_to_oned(as_two_d, self.board_size);
            if index > self.number_of_tiles {
                return 0.0f32
            }
            else {
                //If we eat the food
                match self.board[&index] {
                    BoardState::Food => {
                        self.board.insert(snake_index, BoardState::Empty);
                        self.board.insert(index, BoardState::Snake);
                        self.place_food(index);
                        return 1.0f32;
                    },
                    _ => {}
                }
                self.board.insert(snake_index, BoardState::Empty);
                self.board.insert(index, BoardState::Snake);
                return 0.0f32;
            }
        }


    }

    //Returns, (next_state, reward, is)
    pub fn step(&mut self, action: usize) -> Result<(usize, f32, bool), &'static str> {
        if action >= 4 {
            return Err("Whoops bad action value");
        }
        let reward = self.process_action(action);
        let state = self.where_is_food();
        return Ok((state, reward, false));
    }
}