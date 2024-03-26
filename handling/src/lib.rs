use std::fs::File;
use std::io::ErrorKind;
use std::fs;

pub fn open_or_create(file: &str, content: &str) {
    File::open(file).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    fs::write(file, content).expect("Unable to write to file");
    
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn handling_test() {
        let path = "a.txt";
        File::create(path).unwrap();
        open_or_create(path, "content to be written");

        let mut file = File::open(path).unwrap();

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        assert_eq!("content to be written", s);
    }
}
