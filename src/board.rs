struct Square {
    column: u8,
    row: u8,
}
struct Board {
    squares: Vec<Square>,
    pieces: Vec<Piece>,
}