use std::{array, iter::zip, time::UNIX_EPOCH};

use macroquad::{prelude::*, rand};

mod tetriminos;
pub use crate::tetriminos::peices::*;

// where the board starts
// const BLOCK_SIZE: Vec2 = vec(25.0, 25.0);
const BLOCK_SIZE: f32 = 20.0; // don't need size for x and y

#[macroquad::main("tetris")]
async fn main() {
    // all for a god damn random number...
    rand::srand(
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    //    let board_pos = vec2(((screen_width() / BLOCK_SIZE) / 2.0) - 10.0, 5.0);
    let board_pos = vec2((screen_width() / 2.) - (5.0 * BLOCK_SIZE), 5.0);
    let mut y = 2.0;
    let mut x = 5.0 + rand::gen_range(2, 8) as f32;

    let mut time = 0.0;
    let mut p = PEICES[rand::gen_range(0, 7)].clone();

    loop {
        clear_background(BLACK);

        draw_rectangle(
            board_pos.x,
            board_pos.y,
            10. * BLOCK_SIZE,
            20. * BLOCK_SIZE,
            GRAY,
        );
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
        }

        if is_key_pressed(KeyCode::R) {
            p = PEICES[rand::gen_range(0, 7)].clone();
        }

        if is_key_pressed(KeyCode::W) {
            rotate(&mut p.0);
        }
        if is_key_pressed(KeyCode::S) {
            y += 1.;
        }
        if is_key_pressed(KeyCode::A) && x > board_pos.x {
            x -= 1.;
        }
        if is_key_pressed(KeyCode::D) && x < board_pos.x + 10.0 {
            x += 1.;
        }
        next_frame().await;
    }
}

fn rotate(matrix: &mut [[i32; 4]; 4]) {
    // Is this fast?, probably not
    // do i care?, definatly not
    // Not even sure what's going on here.
    *matrix = array::from_fn(|i| array::from_fn(|j| matrix[j][i]));
    matrix.iter_mut().for_each(|r| r.reverse());
}
// will probably need to draw each square seperatly as they need to disapear seperatly
// further version could use textures with larger destinations sizes and move the active
// tet into a board array when it hits something,
fn draw_tetrimino(tet: &TetriminoType, x: f32, y: f32) {
    let mut y = y;
    for i in tet.0 {
        let mut xt = x;
        for j in i {
            if j != 0 {
                draw_rectangle(
                    xt * BLOCK_SIZE,
                    y * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    tet.1,
                );
            }
            xt += 1.;
        }
        y += 1.;
    }
}
