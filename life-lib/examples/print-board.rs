extern crate life;


fn print_board(board: &life::Board) {
    for y in 0..board.height {
        for x in 0..board.width {
            print!("{}", if *board.get_board_square(&life::Coord {x,y}) {'█'}  else {' '});
        }
        println!("");
    }
}

fn main() {
    let mut board = life::Board::new(64, 1024);
    let coords: Vec<life::Coord> = (0..64)
        .flat_map(|x| vec![life::Coord { x, y: 256 }, life::Coord { x, y: 768 }])
        .collect();
    life::set_squares_to(&mut board, &coords[..], true);
    let mut board_b = board.clone();
    // let boards = [&mut board, &mut board_b];

    for gen in (0..).take(100000) {
        if gen % 2 == 0 {
            life::game_of_life_step(&board, &mut board_b);
        } else {
            life::game_of_life_step(&board_b, &mut board);
        }
        if board == board_b {
            println!("Becomes stable after {} generations", gen);
            break;
        }
        print_board(&board);
    }
}