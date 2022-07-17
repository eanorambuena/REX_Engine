use console::output::print;
pub mod text;

#[derive(Clone)]
pub struct Image {
    pub array: Vec<u8>,
    pub content: Vec<String>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn from_text(file_name: &str) -> Image {
        let src = file_name.to_string();
        let text = text::read(src.clone()).unwrap();
        let lines = text::split(text.content.clone(), "\r\n");
        let height = lines.len();
        let width = lines[0].len();
        return Image {
            array: vec![0; width * height],
            content: lines,
            width: width,
            height: height,
        }
    }

    pub fn display(&self) {
        for line in 0..self.height {
            print(self.content[line].clone());
        }
    }
}

pub struct Player {
    pub skin: Image,
    pub position: (u64, u64),
}
