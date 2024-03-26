use std::io;

fn main() {
    let mut count = 1;
    loop {  
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut anwser = String::new();
        io::stdin()
            .read_line(&mut anwser)
            .expect("Failed to read line");
        let anwser: String = match anwser.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if anwser == "The letter e" {
            if count == 1 {
                println!("It took you {} trial to get the right answer", count);
                break;
            } else {
                println!("It took you {} trials to get the right answer", count);
                break;
            }
        } else {
          count += 1;
        }
      
    }; 
}

