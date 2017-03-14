pub struct TodoApp {}

use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

impl TodoApp {
    pub fn run(&self, args: Vec<String>) -> String {
        return match args[0].as_ref() {
            "add" => {
                self.add(&args[1]);
                "".to_string()
            }
            "list" => self.list(),
            _ => "Unrecognized command".to_string(),
        }
    }

    fn add(&self, text: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/Users/giuseppe/.todo")
            .unwrap();
        file.write_all((text.to_string() + "\n").as_bytes());
    }

    fn list(&self) -> String {
        let mut file = OpenOptions::new()
            .read(true)
            .open("/Users/giuseppe/.todo")
            .unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer);
        return buffer;
    }
}
