use std::fs::File;
use std::fs;
use std::io::ErrorKind;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("File not found")
        } else {
            panic!("{:?}", error)
        }
    })

}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;

    #[test]
    #[should_panic(expected = "File not found")]
    fn open_test() {
        let filename = "created.txt";
        File::create(filename).unwrap();

        let a = open_file(filename);
        println!("{:?}", a);

        fs::remove_file(filename).unwrap();

        //It must panic
        let b = open_file(filename);
    }
}