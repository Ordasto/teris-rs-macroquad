use macroquad::{prelude::*, rand};

#[macroquad::main("tetris")]
async fn main() {
    let mut y = 0;
    let mut x = 0;
    let mut time = 0.0;
    let mut p = PEICES[rand::gen_range(0, 7)];
    loop {
        clear_background(BLACK);

        // accidently quantum tetris, lmao
        // [IDEA]
        // have peice flicker between two or more different shapes and have the
        // bottom of screen shows what peice might turn into with percent chance,
        // e.g. 30% -> T and 50% -> O or something like that
        // have it colapse into one of the shapes at the end of the timer or on some input,
        //  settings?
        // could also have the color be a mix of the two, would need to rewrite the drawing thing
        // though as it stores the color of the block,
        // Gambling tetris
        // Make normal tetris first

        draw_tetrimino(p, x, y);

        // my smol brain hasn't figured out yet how to have something run every 1/n seconds, or
        // whaterver
        time += get_frame_time();

        if time > 0.25 {
            // every 1/4 second
            time = 0.0;
            //             y += 1;
            // p = PEICES[rand::gen_range(0, 7)];
        }

        //         if is_key_pressed(KeyCode::W) {}
        if is_key_pressed(KeyCode::S) {
            y += 1;
        }
        if is_key_pressed(KeyCode::A) {
            x -= 1;
        }
        if is_key_pressed(KeyCode::D) {
            x += 1;
        }
        next_frame().await;
    }
}

// Could have a color overwrite Option<Color> Some() or tet.1
fn draw_tetrimino(tet: &TetriminoType, x: i32, y: i32) {
    for i in 0..16 {
        if tet.0 as usize & 0b_1000_0000_0000_0000 >> i != 0 {
            let x = (i % 4) + 1 + x; // + 1 to both to avoid zero indexing
            let y = (i / 4) + 1 + y;

            // this is probably pretty inefficient
            draw_rectangle(
                BOARD_POS.x * x as f32,
                BOARD_POS.y * y as f32,
                10.0,
                10.0,
                tet.1,
            );
        }
    }
}

// where the board starts
const BOARD_POS: Vec2 = vec2(10.0, 10.0);

struct TetriminoType([[i32; 4]; 4], Color);

const TETRIMINO_I: [[i32; 4]; 4] = [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]];

const TETRIMINO_J: [[i32; 3]; 3] = [[0, 0, 1], [1, 1, 1], [0, 0, 0]];

const TETRIMINO_L: [[i32; 3]; 3] = [[1, 0, 0], [1, 1, 1], [0, 0, 0]];

const TETRIMINO_O: [[i32; 2]; 2] = [[1, 1], [1, 1]];

const TETRIMINO_S: [[i32; 3]; 3] = [[0, 1, 1], [1, 1, 0], [0, 0, 0]];

const TETRIMINO_T: [[i32; 3]; 3] = [[0, 1, 0], [1, 1, 1], [0, 0, 0]];

const TETRIMINO_Z: [[i32; 3]; 3] = [[1, 1, 0], [0, 1, 1], [0, 0, 0]];

const CYAN: Color = color_u8!(0, 255, 255, 255); // Guess ill do it myself
