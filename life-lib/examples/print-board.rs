extern crate life;


fn print_board(board: &life::Board) {
    for y in 0..board.height {
        for x in 0..board.width {
            print!("{}", if board.get_board_square(&life::Coord {x,y}) == life::Cell::Alive {'█'}  else {' '});
        }
        println!("");
    }
}

fn main() {
    let mut board = life::Board::new(256, 256);
    let coords: Vec<life::Coord> = (0..256)
        .flat_map(|x| vec![life::Coord { x, y: 32 }, life::Coord { x, y: 224 }])
        .collect();
    for coord in coords.iter()
    {
        board.set_board_square(coord, life::Cell::Alive);
    }
    let mut board_b = board.clone();
    for gen in (0..).take(1000) {
        if gen % 2 == 0 {
            life::game_of_life_step(&board, &mut board_b);
        } else {
            life::game_of_life_step(&board_b, &mut board);
        }
        if board == board_b {
            break;
        }
        print_board(&board);
    }
}