use macroquad::{prelude::*, rand};

#[macroquad::main("tetris")]
async fn main() {
    let mut y = 0;
    let mut x = 0;
    let mut rot = 0;
    let mut time = 0.0;
    let mut p: TetriminoType = PEICES[rand::gen_range(0, 7)].clone();
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

        draw_tetrimino(&p, x, y);

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
            p = PEICES[rand::gen_range(0, 7)].clone();
        }

        if is_key_pressed(KeyCode::W) {
            p.0 = rotate_tetrimino_brute(p.0);
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

fn rotate_tetrimino_brute(tet: u16) -> u16 {
    let mut output = 0;

    // This is honestly the worst thing i've ever done,
    // i'm so sorry, i could not be bothered to do the funny math stuff
    set_bit_if(&mut output, tet, 0xF, 0xC);
    set_bit_if(&mut output, tet, 0xE, 0x8);
    set_bit_if(&mut output, tet, 0xD, 0x4);
    set_bit_if(&mut output, tet, 0xC, 0x0);
    set_bit_if(&mut output, tet, 0xB, 0xD);
    set_bit_if(&mut output, tet, 0xA, 0x9);
    set_bit_if(&mut output, tet, 0x9, 0x5);
    set_bit_if(&mut output, tet, 0x8, 0x1);
    set_bit_if(&mut output, tet, 0x7, 0xE);
    set_bit_if(&mut output, tet, 0x6, 0xA);
    set_bit_if(&mut output, tet, 0x5, 0x6);
    set_bit_if(&mut output, tet, 0x4, 0x2);
    set_bit_if(&mut output, tet, 0x3, 0xF);
    set_bit_if(&mut output, tet, 0x2, 0xB);
    set_bit_if(&mut output, tet, 0x1, 0x7);
    set_bit_if(&mut output, tet, 0x0, 0x3);

    output
}

/// Sets `set` bit in output if condition bit is set in target
/// `set` and `condition` are indexex.
/// i.e. `set = 1` would mean the 2nd bit (left to right + zero indexing)
///
fn set_bit_if(output: &mut u16, target: u16, condition: u8, set: u8) {
    // move condition bit in target to last bit
    // & 1 to get only the last bit
    // then move it back to the set bit
    *output |= ((target >> condition) & 1) << set;
    //*output |= (target >> condition) << set;
}

// works but cuts off stuff,
fn rotate_tetrimino(tet: u16) -> u16 {
    let mut output = 0;
    let mut n = 3;

    for i in 0..16 {
        let offset = 0b_1000_0000_0000_0000 >> (n % 17);

        if tet as usize & offset != 0 {
            output |= 0b_1000_0000_0000_0000 >> i;
        }

        n += 4;
        if i % 4 == 0 {
            // we did it 4 times
            n = 3 - i / 4; //
        }
    }

    output
}

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

#[derive(Clone)]
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
