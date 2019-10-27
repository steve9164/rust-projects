// #![feature(test)]
// extern crate test;


#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    #[test]
    fn test_board() {
        let board = Board::new(8, 8);
        assert_eq!(board.get_board_square(&Coord { x: 0, y: 0 }), Cell::Dead);
    }

    #[test]
    fn test_neighbour_count_corners() {
        let mut board = Board::new(8, 8);
        assert_eq!(board.count_alive_neighbours(&Coord { x: 0, y: 0 }), 0);
        assert_eq!(board.count_alive_neighbours(&Coord { x: 7, y: 0 }), 0);
        assert_eq!(board.count_alive_neighbours(&Coord { x: 0, y: 7 }), 0);
        assert_eq!(board.count_alive_neighbours(&Coord { x: 7, y: 7 }), 0);
    }

    #[test]
    fn test_neighbour_count() {
        let mut board = Board::new(8, 8);
        board.set_board_square(&Coord { x: 3, y: 3 }, Cell::Alive);
        assert_eq!(board.count_alive_neighbours(&Coord { x: 2, y: 3 }), 1);
    }

    #[test]
    fn test_neighbour_count_excludes_itself() {
        let mut board = Board::new(8, 8);
        board.set_board_square(&Coord { x: 2, y: 3 }, Cell::Alive);
        assert_eq!(board.count_alive_neighbours(&Coord { x: 2, y: 3 }), 0);
    }

    #[test]
    fn test_game_step_stays_empty() {
        let board = Board::new(8, 8);
        let mut board2 = board.clone();
        game_of_life_step(&board, &mut board2);
        assert_eq!(board.squares, board2.squares);
    }

    #[test]
    fn test_game_step_block_is_stable() {
        let mut board = Board::new(8, 8);
        board.set_board_square(&Coord { x: 5, y: 5 }, Cell::Alive);
        board.set_board_square(&Coord { x: 5, y: 6 }, Cell::Alive);
        board.set_board_square(&Coord { x: 6, y: 5 }, Cell::Alive);
        board.set_board_square(&Coord { x: 6, y: 6 }, Cell::Alive);
        let mut board2 = board.clone();
        game_of_life_step(&board, &mut board2);
        assert_eq!(board.squares, board2.squares);
    }

    #[test]
    fn test_game_step_tub_is_stable() {
        let mut board = Board::new(8, 8);
        board.set_board_square(&Coord { x: 0, y: 6 }, Cell::Alive);
        board.set_board_square(&Coord { x: 2, y: 6 }, Cell::Alive);
        board.set_board_square(&Coord { x: 1, y: 7 }, Cell::Alive);
        board.set_board_square(&Coord { x: 1, y: 5 }, Cell::Alive);

        let mut board2 = board.clone();
        game_of_life_step(&board, &mut board2);
        assert_eq!(board.squares, board2.squares);
    }

    #[test]
    fn test_game_step_blinker() {
        let mut board = Board::new(8, 8);
        board.set_board_square(&Coord { x: 3, y: 5 }, Cell::Alive);
        board.set_board_square(&Coord { x: 3, y: 6 }, Cell::Alive);
        board.set_board_square(&Coord { x: 3, y: 7 }, Cell::Alive);

        let mut board2 = board.clone();
        let mut board3 = board.clone();
        let mut board4 = board.clone();
        game_of_life_step(&board, &mut board2);
        game_of_life_step(&board2, &mut board3);
        game_of_life_step(&board3, &mut board4);
        assert_eq!(board.squares, board3.squares);
        assert_eq!(board2.squares, board4.squares);
        assert_ne!(board.squares, board2.squares);
    }

    // #[bench]
    // fn test_game_step_large_board(b: &mut Bencher) {
    //     b.iter(|| {
    //         let mut board = Board::new(1024, 1024);
    //         let coords: Vec<Coord> = (0..1023)
    //             .flat_map(|x| vec![Coord { x, y: 256 }, Coord { x, y: 768 }])
    //             .collect();
    //         for coord in coords.iter() {
    //             board.set_board_square(coord, Cell::Alive);
    //         }
    //         let mut board_b = board.clone();

    //         for gen in 0..5 {
    //             if gen % 2 == 0 {
    //                 game_of_life_step(&board, &mut board_b);
    //             } else {
    //                 game_of_life_step(&board_b, &mut board);
    //             }
    //         }
    //         board
    //     })
    // }
}

mod utils;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    squares: Vec<Cell>,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

#[wasm_bindgen]
impl Coord {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            squares: vec![Cell::Dead; width * height],
        }
    }

    #[inline]
    pub fn get_board_square(&self, c: &Coord) -> Cell {
        self.squares[c.y * self.width + c.x]
    }

    #[inline]
    pub fn set_board_square(&mut self, c: &Coord, val: Cell) {
        self.squares[c.y * self.width + c.x] = val;
    }

    pub fn count_alive_neighbours(&self, c: &Coord) -> u8 {
        let mut count = 0;
        let min_x = if c.x == 0 { 0 } else { c.x - 1 };
        let min_y = if c.y == 0 { 0 } else { c.y - 1 };
        let max_x = if c.x == self.width - 1 {
            self.width - 1
        } else {
            c.x + 1
        };
        let max_y = if c.y == self.height - 1 {
            self.height - 1
        } else {
            c.y + 1
        };

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if !(x == c.x && y == c.y) && self.get_board_square(&Coord { x, y }) == Cell::Alive
                {
                    count += 1;
                };
            }
        }
        count
    }
}

#[wasm_bindgen]
pub fn game_of_life_step(b: &Board, next_board: &mut Board) {
    assert!(
        b.width == next_board.width,
        "next_board.width must be the same as b.width"
    );
    assert!(
        b.height == next_board.height,
        "next_board.height must be the same as b.height"
    );

    let mut coord: Coord;
    let mut alive_neighbours: u8;
    for x in 0..b.width {
        for y in 0..b.height {
            coord = Coord { x, y };
            alive_neighbours = b.count_alive_neighbours(&coord);
            // Game of life rules, from Wikipedia
            if b.get_board_square(&coord) == Cell::Alive {
                // Live cell only continues living if it has 2 or 3 living neighbours
                next_board.set_board_square(
                    &coord,
                    if alive_neighbours == 2 || alive_neighbours == 3 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    },
                );
            } else {
                // Dead cell only becomes live with 3 live neighbours
                next_board.set_board_square(
                    &coord,
                    if alive_neighbours == 3 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    },
                );
            }
        }
    }
}
