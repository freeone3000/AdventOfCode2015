pub mod common {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};
    use std::path::Path;

    #[allow(dead_code)]
    pub fn read_file<T>(filename: &str, callback: &dyn Fn(&String) -> T) -> T {
        let path = Path::new(filename);
        let mut file = match File::open(&path) {
            Err(why) => panic!("Error opening file: {}", why),
            Ok(file) => file
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("Error reading file: {}", why),
            Ok(_) => return callback(&s)
        };
    }

    #[allow(dead_code)]
    pub fn read_file_linewise<T>(filename: &str, callback: &dyn Fn(&String) -> T) -> Vec<T> {
        let mut mapped = Vec::new();
        let file = match File::open(&filename) {
            Err(why) => panic!("Error opening file: {}", why),
            Ok(file) => file
        };

        let mut reader = BufReader::new(file);
        let mut line = String::new();

        loop {
            match reader.read_line(&mut line) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }
                    let value = callback(&line);
                    mapped.push(value);
                    line.clear();
                },
                Err(why) => panic!("Error reading line: {}", why)
            }
        };
        return mapped;
    }
}