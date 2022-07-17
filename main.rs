use std::time::{Duration, Instant};
use std::thread::sleep;
use data::{Player, Image};
use data::text::read;
use console::output::{print_bar, clear};

mod data;
mod console;

fn main() {
    // Game Initialization
    let title = "Jumping in Rust".to_string();
    let fps = 30;
    let mut start;
    let mut end;

    let bg = Image::_from_text("background.txt");

    let _src_player = "player.txt".to_string();
    let _pl = read(_src_player.clone()).unwrap();

    let _player = Player {
        position: (0, 0),
    };

    // Game Setup
    let frame_duration = Duration::from_millis((1.0 / (fps as f64) * 1000.0) as u64);

    let _x = _pl.content.clone().chars().nth(0).unwrap();

    // Game loop
    for frame in 0..1 {
        start = Instant::now();
        clear();
        print_bar(title.clone(), frame, fps);
        bg.display();
        end = start.elapsed();
        sleep(frame_duration.saturating_sub(end));
    }
}
