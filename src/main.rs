// Conway's Game of Life, implemented in Rust.

// weird edge case: trying to -1 from a usize of 0 causes a panic.
// Solution: Bound all possible coords between [1, BOARD_WIDTH/HEIGHT-1] inclusive. 

use std::{fmt, thread::sleep, time, write};


use rand::distributions::{Bernoulli, Distribution};

const BOARD_WIDTH:   usize = 125 + 1;
const BOARD_HEIGHT:  usize = 70 + 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cell {
    alive:      bool,
    x_coord:    usize,
    y_coord:    usize,
}

impl Cell {
    fn flip(&mut self) {
        self.alive = !self.alive;
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.alive { write!(f, "█") }
        else { write!(f, "░")}
    }

}

struct Board {
    board: [[Cell; BOARD_HEIGHT]; BOARD_WIDTH]
}

impl Board {
    fn new() -> Board {
        let mut board = Board {
            board: [[Cell {
                alive: false,
                x_coord: 0,
                y_coord: 0,
            }; BOARD_HEIGHT]; BOARD_WIDTH],
        };
        for x in 1..BOARD_WIDTH {
            for y in 1..BOARD_HEIGHT {
                let mut cell = board.get_cell_mut((x, y)).unwrap();
                cell.x_coord = x;
                cell.y_coord = y;
            }
        }
        board
    }

    fn get_cell(&self, coords: (usize, usize)) -> Option<&Cell> {
        // Takes a coordinate pair, returns an Option containing
        // the cell at that coordinate.
        // If the cell does not exist (example: beyond boundary
        // of what's allowed on the board), then it contains None
        
        if  coords.0 == 0 ||
            coords.1 == 0 ||
            coords.0 == BOARD_WIDTH ||
            coords.1 == BOARD_HEIGHT {
                return Option::None;
        }

        match self.board.get(coords.0) {
            Some(column) => {
                match column.get(coords.1) {
                    Some(cell) => Some(&cell),
                    None => None
                }
            }
            None => None
        }
    }

    fn get_cell_mut(&mut self, coords: (usize, usize)) -> Option<&mut Cell> {
        // Takes a coordinate pair, returns a mutable Option containing
        // the cell at that coordinate.
        // If the cell does not exist (example: beyond boundary of the board),
        // then it contains None
        if  coords.0 == 0 ||
            coords.1 == 0 ||
            coords.0 == BOARD_WIDTH ||
            coords.1 == BOARD_HEIGHT {
                return Option::None;
        }

        match self.board.get_mut(coords.0) {
            Some(column) => {
                match column.get_mut(coords.1) {
                    Some(cell) => Some(cell),
                    None => None
                }
            }
            None => None
        }
    }

    fn count_adjacent_alive(&self, coords: (usize, usize)) -> u32 {
        let mut num_adjacent_alive = 0;
        
        // For each of the eight adjacent cells,
        // get the cell with Board::get_cell(coords), then
        // check if cell is alive. If so, increment counter.
        
        // Column to the left of the cell
        if let Some(cell) = self.get_cell((coords.0 - 1, coords.1 - 1)) { 
            if cell.alive { num_adjacent_alive += 1;}
        }
        if let Some(cell) = self.get_cell((coords.0 - 1, coords.1)) { 
            if cell.alive { num_adjacent_alive += 1;}
        }
        if let Some(cell) = self.get_cell((coords.0 - 1, coords.1 + 1)) { 
            if cell.alive { num_adjacent_alive += 1;}
        }
        // Column containing the cell
        if let Some(cell) = self.get_cell((coords.0, coords.1 - 1)) { 
            if cell.alive { num_adjacent_alive += 1;}
        }
        // Skipping the target cell itself
        // if let Some(cell) = self.get_cell((coords.0, coords.1)) { 
        //     if cell.alive { num_adjacent_alive += 1;}
        // }
        if let Some(cell) = self.get_cell((coords.0, coords.1 + 1)) { 
            if cell.alive { num_adjacent_alive += 1;}
        }
        // Column to the right of the cell
        if let Some(cell) = self.get_cell((coords.0 + 1, coords.1 - 1)) { 
            if cell.alive { num_adjacent_alive += 1;}
        }
        if let Some(cell) = self.get_cell((coords.0 + 1, coords.1)) { 
            if cell.alive { num_adjacent_alive += 1;}
        }
        if let Some(cell) = self.get_cell((coords.0 + 1, coords.1 + 1)) { 
            if cell.alive { num_adjacent_alive += 1;}
        }

        //Return value
        num_adjacent_alive
    }

    fn get_cells_to_flip(&self) -> Vec<(usize, usize)>{
        // Iterates across the whole board, identifying cells
        // that need to be flipped

        let mut cells_to_flip: Vec<(usize, usize)> = Vec::new();

        for column in &self.board[1..BOARD_WIDTH] {
            for cell in &column[1..BOARD_HEIGHT] {
                let num_adjacent_alive = self.count_adjacent_alive((cell.x_coord, cell.y_coord));
                if cell.alive {
                    match num_adjacent_alive {
                        2 => (),
                        3 => (),
                        _ => cells_to_flip.push((cell.x_coord, cell.y_coord)),
                    }
                } else { 
                    match num_adjacent_alive {
                        3 => cells_to_flip.push((cell.x_coord, cell.y_coord)),
                        _ => (),
                    }
                }
            }
        }
    cells_to_flip
    }

    fn tick(&mut self) {
        let to_flip = self.get_cells_to_flip();
        for coords in to_flip.iter() {
            if let Some(cell) = self.get_cell_mut(*coords) {
                cell.flip();
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 1..BOARD_HEIGHT {
            for x in 1..BOARD_WIDTH {
                if let Some(cell) = self.get_cell((x, y)) {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    // rand boilerplate
    let dist = Bernoulli::new(0.5).unwrap();
    let mut rng = rand::thread_rng();

    //Initialize the board with randomly alive/dead cells
    let mut board = Board::new();
    for x in 1..BOARD_WIDTH {
        for y in 1..BOARD_HEIGHT {
            let mut cell = board.get_cell_mut((x, y)).unwrap();
            cell.alive = dist.sample(&mut rng);
        }
    }

    println!("{}", board);
    loop { 
        board.tick();
        print!("\x1B[2J\x1B[1;1H");
        print!("{}", board);
        sleep(time::Duration::from_millis(100));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::vec;

    fn setup() -> (usize, usize, Board, rand::prelude::ThreadRng) {
        let mut rng = rand::thread_rng();
        let board = Board::new();
        let x_coord = rng.gen_range(1..BOARD_WIDTH);
        let y_coord = rng.gen_range(1..BOARD_HEIGHT);
        (x_coord, y_coord, board, rng)
    }

    #[test]
    fn cell_flip() {

        let (x_coord, y_coord, mut board, _rng) = setup();
        let cell = board.get_cell_mut((x_coord, y_coord)).unwrap();
        cell.flip();

        assert_eq!(
            board.get_cell((x_coord, y_coord)).unwrap().alive,
            true
        )
    }

    #[test]
    fn board_get_cell() {
        let (x_coord, y_coord, mut board, _rng) = setup();
        board.board[x_coord][y_coord].alive = true;
        assert_eq!(
            board.get_cell((x_coord, y_coord)).unwrap().alive,
            true
        );
    }

    #[test]
    fn boarder_1() {
        let (_x_coord, _y_coord, board, _rng) = setup();
        assert_eq!(board.get_cell((0,0)), None);
    }

    #[test]
    fn boarder_2() {
        let (_x_coord, _y_coord, board, _rng) = setup();
        assert_eq!(board.get_cell((BOARD_WIDTH,BOARD_HEIGHT)), None);
    }

    #[test]
    fn board_count_adjacent_alive_1() {
        let (_x_coord, _y_coord, board, _rng) = setup();
        board.count_adjacent_alive((1,1)); //Should not panic
    }

    #[test]
    fn board_count_adjacent_alive_2() {
        let (x_coord, y_coord, mut board, mut rng) = setup();
        
        // Randomly select three adjacent cells to make alive
        let targets = [
            rng.gen_range(0..9),
            rng.gen_range(0..9),
            rng.gen_range(0..9)
        ];

        let mut counter = 0;
        for target in targets.iter() {
            match target {
                1 => {
                    if let Some(cell) = board.get_cell_mut((x_coord - 1, y_coord - 1)) { 
                        if !cell.alive {
                            cell.alive = true; 
                            counter += 1; 
                        }
                    }
                }
                2 => {
                    if let Some(cell) = board.get_cell_mut((x_coord - 1, y_coord)) {
                        if !cell.alive {
                            cell.alive = true; 
                            counter += 1; 
                        }
                    }
                }
                3 => {
                    if let Some(cell) = board.get_cell_mut((x_coord - 1, y_coord + 1)) {
                        if !cell.alive {
                            cell.alive = true; 
                            counter += 1; 
                        }
                    }
                }
                4 => {
                    if let Some(cell) = board.get_cell_mut((x_coord, y_coord - 1)) {
                        if !cell.alive {
                            cell.alive = true; 
                            counter += 1; 
                        }
                    }
                }
                5 => {
                    if let Some(cell) = board.get_cell_mut((x_coord, y_coord + 1)) {
                        if !cell.alive {
                            cell.alive = true; 
                            counter += 1; 
                        }
                    }
                }
                6 => {
                    if let Some(cell) = board.get_cell_mut((x_coord + 1, y_coord - 1)) {
                        if !cell.alive {
                            cell.alive = true; 
                            counter += 1; 
                        }
                    }
                }
                7 => {
                    if let Some(cell) = board.get_cell_mut((x_coord + 1, y_coord)) {
                        if !cell.alive {
                            cell.alive = true; 
                            counter += 1; 
                        }
                    }
                }
                8 => {
                    if let Some(cell) = board.get_cell_mut((x_coord + 1, y_coord + 1)) {
                        if !cell.alive {
                            cell.alive = true; 
                            counter += 1; 
                        }
                    }
                }
                _ => ()
            }
        }

        assert_eq!(board.count_adjacent_alive((x_coord, y_coord)), counter)
    }
    
    #[test]
    fn board_get_cells_to_flip_1() {
        let (_x_coord, _y_coord, mut board, mut rng) = setup();
        
        let coords1: (usize, usize) = (
            rng.gen_range(0..BOARD_WIDTH), rng.gen_range(1..BOARD_HEIGHT)
        );
        let coords2: (usize, usize) = (
            rng.gen_range(0..BOARD_WIDTH), rng.gen_range(1..BOARD_HEIGHT)
        );
        let coords3: (usize, usize) = (
            rng.gen_range(0..BOARD_WIDTH), rng.gen_range(1..BOARD_HEIGHT)
        );

        for coords in [coords1, coords2, coords3].iter() {
            board.get_cell_mut(*coords).unwrap().flip();
        }

        let mut flipped = vec![coords1, coords2, coords3];
        let mut test_result = board.get_cells_to_flip();
        flipped.sort();
        test_result.sort();
        assert_eq!(flipped, test_result);
    }

    #[test]
    fn board_get_cells_to_flip_2() {
        let (_x_coord, _y_coord, mut board, mut _rng) = setup();
        
        let coords1: (usize, usize) = (4,4);
        let coords2: (usize, usize) = (4,5);
        let coords3: (usize, usize) = (5,5);

        for coords in [coords1, coords2, coords3].iter() {
            board.get_cell_mut(*coords).unwrap().flip();
        }

        let to_flip: Vec<(usize, usize)> = vec![(5,4)];
        let test_result = board.get_cells_to_flip();
        assert_eq!(to_flip, test_result);
    }

    #[test]
    fn board_get_cells_to_flip_3() {
        let (_x_coord, _y_coord, mut board, mut _rng) = setup();
        
        let coords1: (usize, usize) = (1,1);

        for coords in [coords1].iter() {
            board.get_cell_mut(*coords).unwrap().flip();
        }

        let to_flip: Vec<(usize, usize)> = vec![(1,1)];
        let test_result = board.get_cells_to_flip();
        assert_eq!(to_flip, test_result);
    }

    #[test]
    fn board_get_cells_to_flip_4() {
        let (_x_coord, _y_coord, mut board, mut _rng) = setup();
        
        let coords1: (usize, usize) = (1,1);
        let coords2: (usize, usize) = (1,2);
        let coords3: (usize, usize) = (2,2);
        let coords4: (usize, usize) = (2,1);

        for coords in [coords1, coords2, coords3, coords4].iter() {
            board.get_cell_mut(*coords).unwrap().flip();
        }

        let to_flip: Vec<(usize, usize)> = vec![];
        let test_result = board.get_cells_to_flip();
        assert_eq!(to_flip, test_result);
    }

    #[test]
    fn board_get_cells_to_flip_5() {
        let (_x_coord, _y_coord, mut board, mut _rng) = setup();
        
        let coords1: (usize, usize) = (1,1);
        let coords2: (usize, usize) = (1,2);
        let coords3: (usize, usize) = (2,2);
        let coords4: (usize, usize) = (2,1);
        let coords5: (usize, usize) = (3,1);
        let coords6: (usize, usize) = (3,2);

        for coords in [coords1, coords2, coords3, coords4, coords5, coords6].iter() {
            board.get_cell_mut(*coords).unwrap().flip();
        }

        let mut to_flip: Vec<(usize, usize)> = vec![(2,1), (2,2), (2,3)];
        let mut test_result = board.get_cells_to_flip();
        to_flip.sort();
        test_result.sort();
        assert_eq!(to_flip, test_result);
    }


}
