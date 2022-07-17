use std::fs::File;
use std::io::Read;

#[derive(Clone)]
pub struct Text {
    // Text is a string struct
    pub name: String,
    pub array: Vec<u8>,
    pub content: String,
}

impl Text {
    fn new(data: Vec<u8>, name: String) -> Text {
        Text {
            name: name,
            array: data.clone(),
            content: String::from_utf8(data).unwrap(),
        }
    }
}

pub fn split(text: String, ch: &str) -> Vec<String> {
    // Split a string into a vector of strings, separated by a given character
    let mut result: Vec<String> = Vec::new();
    let splited_text = text.split(ch);
    let mut sub_string;

    for sub_text in splited_text {
        sub_string = sub_text.to_string();
        result.push(sub_string.clone());
    }
    return result;
}

fn read_a_file(path: String) -> std::io::Result<Vec<u8>> {
    let mut file = (File::open(path))?;

    let mut data = Vec::new();
    (file.read_to_end(&mut data))?;

    return Ok(data);
}

pub fn read(path: String) -> std::io::Result<Text> {
    let data = read_a_file(path.clone()).unwrap();
    return Ok(Text::new(data, path));
}
