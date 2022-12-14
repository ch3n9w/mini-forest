use std::fs::{File, self};
use std::path::Path;
use std::io::Write;

const FOREST_FILE: &str = "/tmp/forest_data_file";

pub struct Server<'a> {
    pub path: &'a str, 
}

impl<'a> Server<'a> {

    pub const fn new() -> Server<'a> {
        Server {
            path: FOREST_FILE
        }
        
    }

    pub fn destruct(&self) {
        match fs::remove_file(self.path) {
            Err(_) => panic!("Can not clean temp file!"),
            Ok(_) => {}
        }
    }

    pub fn read_status(&self) -> String {
        match fs::read_to_string(&self.path) {
            Err(_) => { String::from("Start?") },
            Ok(text) => {text}
        }
    }

    pub fn write_status(&self, time: &str) -> bool {
        let path = Path::new(&self.path);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(time.as_bytes()) {
            Err(why) => {
                println!("Can not write cache file:{}", why);
                return false;
            },
            Ok(_) => {
                // println!("succeefully write to {}", display);
                return true;
            },
        };
    }
}



