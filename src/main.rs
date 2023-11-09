use macroquad::{prelude::*, rand};

#[macroquad::main("tetris")]
async fn main() {
    let mut y = 0;
    let mut x = 0;
    let mut rot = 0;
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
        draw_tetrimino_rot(p, x + 10, y + 10, rot % 4);

        // my smol brain hasn't figured out yet how to have something run every 1/n seconds, or
        // whaterver
        time += get_frame_time();

        if time > 0.25 {
            // every 1/4 second
            time = 0.0;
            //             y += 1;
            // p = PEICES[rand::gen_range(0, 7)];
        }
        if is_key_pressed(KeyCode::R) {
            p = PEICES[rand::gen_range(0, 7)];
        }

        if is_key_pressed(KeyCode::W) {
            rot += 1;
        }
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

// get one rotation working, just need to figure the rest
fn draw_tetrimino_rot(tet: &TetriminoType, x: i32, y: i32, rot: i32) {
    let mut n = 3;
    for i in 0..16 {
        let val = 0b_1000_0000_0000_0000 >> (n % 17);
        if tet.0 as usize & val != 0 {
            let x = (i % 4) + 1 + x; // + 1 to both to avoid zero indexing
            let y = (i / 4) + 1 + y;

            // this is probably pretty inefficient
            draw_rectangle(
                BOARD_POS.x + (x * 10) as f32,
                BOARD_POS.y + (y * 10) as f32,
                10.0,
                10.0,
                tet.1,
            );
        }
        n += 4;
        if i % 4 == 0 {
            // we did it 4 times
            n = 3 - i / 4; //
        }
    }
}

const BOARD_POS: Vec2 = vec2(10.0, 10.0);

struct TetriminoType(u16, Color);

const PEICES: [&TetriminoType; 7] = [&I, &J, &L, &O, &S, &T, &Z];

const CYAN: Color = color_u8!(0, 255, 255, 255); // Guess ill do it myself

const I: TetriminoType = TetriminoType(0b_1000_1000_1000_1000, CYAN); // cyan,
const J: TetriminoType = TetriminoType(0b_0100_0100_1100_0000, BLUE); // blue,
const L: TetriminoType = TetriminoType(0b_1000_1000_1100_0000, ORANGE); // orange,
const O: TetriminoType = TetriminoType(0b_1100_1100_0000_0000, YELLOW); // yellow,
const S: TetriminoType = TetriminoType(0b_0110_1100_0000_0000, GREEN); // green,
const T: TetriminoType = TetriminoType(0b_1110_0100_0000_0000, PURPLE); // purple,
const Z: TetriminoType = TetriminoType(0b_1100_0110_0000_0000, RED); // red
