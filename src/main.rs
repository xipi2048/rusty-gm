use raylib::prelude::*;

fn main() {
    let w = 960;
    let h = 540;

    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("Game Master")
        .build();    

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
    }
}