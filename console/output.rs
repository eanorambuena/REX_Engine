use std::process::Command;

pub fn print(message: String) {
    println!("{}", message);
}

pub fn _print_v(message: Vec<u8>) {
    println!("{:?}", message);
}

pub fn _print_vs(message: Vec<String>) {
    println!("{:?}", message);
}

pub fn print_bar(title: String, frame: u64, fps: u64) {
    println!("{}      {}      {} FPS", frame, title.clone(), fps);
}

pub fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    }
    else {
        Command::new("clear");
    }
}
