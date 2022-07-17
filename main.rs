use std::thread::sleep;
use data::{Player, Image};
use console::output::{print_bar, clear};

mod data;
mod console;
mod flow;

fn main() {
    // Game Initialization
    let title = "Jumping in Rust".to_string();
    let fps = 30;
    let mut frame = flow::Frame::new(fps.clone());

    let bg = Image::from_text("background.txt");
    let pl = Image::from_text("player.txt");

    let player = Player {
        skin: pl,
        position: (0, 0),
    };

    // Game Setup
    let _x = player.skin.content[0].clone().chars().nth(0).unwrap();

    // Game loop
    for frame_index in 0..100 {
        frame.start();
        clear();
        
        print_bar(title.clone(), frame_index, fps.into());
        bg.display();

        frame.end();
        sleep(frame.left_delta_time());
    }
}
