extern crate core;
use std::borrow::Cow;
pub fn spell(n: u64) -> String {
    match n {
        0..=999_999 => upto_999999(n).into_owned(),
        1_000_000 => "one million".to_string(),
        _ => panic!("Number greater than 1000000")
    }
}
fn upto_9(n: u64) -> Cow<'static, str> {
    Cow::from(match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!("Number greater than 9")
    })
}
fn teens(n: u64) -> Cow<'static, str> {
    Cow::from(match n {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => panic!("Number not in 10 - 19 range"),
    })
}
fn tens(n: u64) -> Cow<'static, str> {
    Cow::from(match n {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => panic!("Number not in 2 - 9 range")
    })
}
fn upto_99(n: u64) -> Cow<'static, str> {
    match n {
        0..=9 => upto_9(n),
        10..=19 => teens(n),
        20..=99 => {
            let small = n % 10;
            let big = n / 10;
            if small == 0 {
                tens(big)
            } else {
                Cow::from(format!("{}-{}", tens(big), upto_9(small)))
            }
        }
        _ => panic!("Number larger than 99")
    }
}
fn upto_999(n: u64) -> Cow<'static, str> {
    match n {
        0..=99 => upto_99(n),
        100..=999 => {
            let big = n / 100;
            let small = n % 100;
            if small == 0 {
                Cow::from(format!("{} hundred", upto_9(big)))
            } else {
                Cow::from(format!("{} hundred {}", upto_9(big), upto_99(small)))
            }
        }
        _ => panic!("Number larger than 999")
    }
}
fn upto_999999(n: u64) -> Cow<'static, str> {
    match n {
        0..=999 => upto_999(n),
        1000..=999_999 => {
            let big = n / 1000;
            let small = n % 1000;
            if small == 0 {
                Cow::from(format!("{} thousand", upto_999(big)))
            } else {
                Cow::from(format!("{} thousand {}", upto_999(big), upto_999(small)))
            }
        }
        _ => panic!("Number larger than 999999"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("{}", spell(348));
        println!("{}", spell(59696));
    }
}
