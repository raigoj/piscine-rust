pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut new_vec = Vec::new();
    let mut new_string = String::new();
    let mut new_string2 = String::new();
    let pat = " ";
    for i in names {
        let patrn = i.find(pat);
        
        let index_of = match patrn {
            Some(x) => x,
            None => 0,
        };
        new_string.push(i.chars().nth(0).unwrap());
        new_string2.push(i.chars().nth(index_of + 1).unwrap());
        let inits = new_string.clone() + ". " + &new_string2.clone() + ".";
        new_string.clear();
        new_string2.clear(); 
        new_vec.push(inits);
        
    };
    // 
    //     let spl = i.split_whitespace();
    //     let first = spl.;
    //     
    //     new_vec.push(".");

    // 
    return new_vec;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initials_test() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        let inits = vec!["H. P.", "S. E.", "J. L.", "B. O."];

        assert_eq!(inits, initials(names));
    }
}
