pub fn delete_and_backspace(s: &mut String) {
    while let Some(i) = s.find('-') {
        s.replace_range(i-1..=i, "")
    }
    while let Some(i) = s.rfind('+') {
        s.replace_range(i..=i+1, "")
    }
}

pub fn is_correct(v: &mut Vec<&str>) -> usize {
    for string in v.iter_mut() {
        let mut split = string.split('=');
        let str1 = split.next().unwrap();
        let str2 = split.next().unwrap();
        let res1 = meval::eval_str(str1).unwrap();
        let res2 = meval::eval_str(str2).unwrap();
        *string = if res1 == res2 {
            "✔"
        } else {
            "✘"
        }
    };
    v.iter().filter(|s| **s == "✔").count() * 100 / v.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_and_backspace_test() {
        let mut a = String::from("bpp--o+er+++sskroi-++lcw");

        delete_and_backspace(&mut a);
        assert_eq!("borrow", a)
    }

    #[test]
    fn is_correct_test() {
        let mut b: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=3", "5+5=10"];

        assert_eq!(75, is_correct(&mut b));
        assert_eq!(vec!["✔", "✔", "✘", "✔"], b)
    }
}