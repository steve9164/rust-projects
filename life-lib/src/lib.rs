#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct Board {
    width: usize;
    height: usize;
    squares: Vec<boolean>;
}

fn get_board_square(Board &b, Coord &c): &boolean {
    &b.squares[c.y * b.width + c.x]
}

fn get_board_square_mut(Board &mut b, Coord &c): &mut boolean {
    &mut b.squares[c.y * b.width + c.x]
}

struct Coord {
    x: usize;
    y: usize;
}

fn count_alive_neighbours(Board &board, Coord &coord) {
    let mut usize count = 0;
    let mut Coord c
}

