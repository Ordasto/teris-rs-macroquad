#[rustfmt::skip] // It hates consts on multiple lines, for some reason.
pub mod peices { // it's just in here so rustfmt doesn't turn the formatting to beans
    use macroquad::{color::*, color_u8};
   
    #[derive(Clone)]
    pub struct TetriminoType(pub [[i32; 4]; 4], pub Color);
    pub const PEICES: [&TetriminoType; 7] = [&I, &J, &L, &O, &S, &T, &Z];
   
    const CYAN: Color = color_u8!(0, 255, 255, 255); // Guess ill do it myself
    
    const I: TetriminoType = TetriminoType([
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],CYAN);
    
    const J: TetriminoType = TetriminoType([
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],BLUE);
    
    const L: TetriminoType = TetriminoType([
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],ORANGE);
    
    const O: TetriminoType = TetriminoType([
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],YELLOW);
    
    const S: TetriminoType = TetriminoType([
        [0, 1, 1, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0],
    ],GREEN);
    
    const T: TetriminoType = TetriminoType([
        [0, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0],
    ],PURPLE);
    
    const Z: TetriminoType = TetriminoType([
        [0, 1, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 0],
    ],RED);
}
