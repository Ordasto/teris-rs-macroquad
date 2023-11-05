use macroquad::{prelude::*, rand};

#[macroquad::main("tetris")]
async fn main() {
    let mut y = 0;
    let mut x = 0;
    let mut time = 0.0;
    loop {
        clear_background(BLACK);

        // accidently quantum tetris, lmao
        let p = PEICES[rand::gen_range(0, 7)];
        draw_tetrimino(p, x, y);

        time += get_frame_time();

        if time > 0.25 {
            // every 1/4 second
            time = 0.0;
            y += 1;
        }

        next_frame().await;
    }
}

fn draw_tetrimino(tet: &TetriminoType, x: i32, y: i32) {
    for i in 0..16 {
        if tet.0 as usize & 0b_1000_0000_0000_0000 >> i != 0 {
            let x = (i % 4) + 1 + x; // + 1 to both to avoid zero indexing
            let y = (i / 4) + 1 + y;
            draw_rectangle(
                BOARD_POS.x * x as f32,
                BOARD_POS.y * y as f32,
                10.0,
                10.0,
                RED,
            );
        }
    }
}

const BOARD_POS: Vec2 = vec2(10.0, 10.0);

// mask is 0b_1111_0... >> layer of peice-1 so second layer is >> 1

// Could just mask onto a bitmap esque thing, >> by x position (0 to 10 - width) and >> by y * 10
// peice >> (x + y*4)
// not sure if it would work really,
// or,
// array of 2 byte(16bit) numbers, >> for x pos and index for y pos
//

struct TetriminoType(u16);
const PEICES: [&TetriminoType; 7] = [&I, &J, &L, &O, &S, &T, &Z];
const I: TetriminoType = TetriminoType(0b_1000_1000_1000_1000); // cyan,
const J: TetriminoType = TetriminoType(0b_0100_0100_1100_0000); // blue,
const L: TetriminoType = TetriminoType(0b_1000_1000_1100_0000); // orange,
const O: TetriminoType = TetriminoType(0b_1100_1100_0000_0000); // yellow,
const S: TetriminoType = TetriminoType(0b_0110_1100_0000_0000); // green,
const T: TetriminoType = TetriminoType(0b_1110_0100_0000_0000); // purple,
const Z: TetriminoType = TetriminoType(0b_1100_0110_0000_0000); // red
