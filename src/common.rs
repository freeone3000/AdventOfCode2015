pub mod common {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn read_file(filename: &str, callback: &dyn Fn(String) -> i32) {
        let path = Path::new(filename);
        let mut file = match File::open(&path) {
            Err(why) => panic!("Error opening file: {}", why),
            Ok(file) => file
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("Error reading file: {}", why),
            Ok(_) => println!("Result: {}", callback(s))
        };
    }
}