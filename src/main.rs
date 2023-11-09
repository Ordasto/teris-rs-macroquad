use macroquad::{models::Vertex, prelude::*, rand};

mod tetriminos;
pub use crate::tetriminos::peices::*;

// where the board starts
const BOARD_POS: Vec2 = vec2(10.0, 10.0);

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

        draw_tetrimino(p, x, y, 0);

        // my smol brain hasn't figured out yet how to have something run every 1/n seconds, or
        // whaterver
        time += get_frame_time();

        if time > 0.25 {
            // every 1/4 second
            time = 0.0;
            //             y += 1;
            p = PEICES[rand::gen_range(0, 7)];
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
// fn draw_tetrimino(tet: &TetriminoType, x: i32, y: i32) {
//     for i in 0..16 {
//         if tet.0 as usize & 0b_1000_0000_0000_0000 >> i != 0 {
//             let x = (i % 4) + 1 + x; // + 1 to both to avoid zero indexing
//             let y = (i / 4) + 1 + y;
//
//             // this is probably pretty inefficient
//             draw_rectangle(
//                 BOARD_POS.x * x as f32,
//                 BOARD_POS.y * y as f32,
//                 10.0,
//                 10.0,
//                 tet.1,
//             );
//         }
//     }
// }

// will probably need to draw each square seperatly as they need to disapear seperatly
// further version could use textures with larger destinations sizes and move the active
// tet into a board array when it hits something,
fn draw_tetrimino(tet: &TetriminoType, x: f32, y: f32, rot: i32) {
    let mut y = y;
    for i in tet.0 {
        let mut xt = x;
        for j in i {
            if j != 0 {
                draw_rectangle(xt * 10.0, y * 10.0, 10.0, 10.0, tet.1);
            }
            xt += 1.;
        }
        y += 1.;
    }
}
