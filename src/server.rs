
use std::fs::{File, self};
use std::io::{Write, Read};
use std::path::Path;

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


    pub fn read_status(&self) -> String {
        // let path = Path::new(&self.path);
        // let display = path.display();
        // let mut file = match File::open(&path) {
        //     Err(why) => panic!("couldn't open {}: {}", display, why),
        //     Ok(file) => file,
        // };

        // let mut s = String::new();
        // match file.read_to_string(&mut s) {
        //     Err(why) => panic!("couldn't read {}:{}", display, why),
        //     Ok(_) => print!("{} contains:\n {}", display, s),
        // };
        // return s;
        match fs::read_to_string(&self.path) {
            Err(_) => { String::from("No forest data") },
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
                println!("Can not write {}:{}", display, why);
                return false;
            },
            Ok(_) => {
                // println!("succeefully write to {}", display);
                return true;
            },
        };
    }
}



