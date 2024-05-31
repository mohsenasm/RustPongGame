mod controller;
mod board;
mod terminal;

use rand::Rng;

fn main() {
    let random_y = rand::thread_rng().gen_range(0..6);
    let random_x = rand::thread_rng().gen_range(2..50);
    let random_x2 = rand::thread_rng().gen_range(-2..2);

    let board = &mut board::Board {
        inner_size: board::XY { x: 60, y: 15},
        ball_position: board::XY { x: random_x, y: random_y},
        ball_direction: board::XY { x: 1, y: 1},
        plate_length: 8,
        plate_position_x: random_x + random_x2,
    };
    controller::run_loop(board);
}
