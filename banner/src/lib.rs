use std::collections::HashMap;
use std::num::ParseFloatError;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Self {
            short_hand: format!("-{}", l_h.chars().nth(0).unwrap()),
            long_hand: format!("--{}", l_h),
            desc: d.to_string(),
        }
        
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        let callback_f = self.flags.get(&flag).unwrap();
        callback_f(argv[0], argv[1]).unwrap_or_else(|err| err.to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_f = a.parse::<f32>()?;
    let b_f = b.parse::<f32>()?;
    let res = a_f/b_f;
    Ok(res.to_string())
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_f = a.parse::<f32>()?;
    let b_f = b.parse::<f32>()?;
    let res = a_f%b_f;
    Ok(res.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_test() {
        let mut handler = FlagsHandler { flags: HashMap::new() };

        let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
        let r = Flag::opt_flag(
            "remainder",
            "remainder of the division between two values, formula (a % b)",
        );

        handler.add_flag((d.short_hand, d.long_hand), div);
        handler.add_flag((r.short_hand, r.long_hand), rem);

        let div_key = ("-d".to_string(), "--division".to_string());
        let rem_key = ("-r".to_string(), "--remainder".to_string());

        assert_eq!("0.5", handler.exec_func(div_key.clone(), &["1.0", "2.0"]));

        assert_eq!("0", handler.exec_func(rem_key.clone(), &["2.0", "2.0"]));

        assert_eq!("invalid float literal", handler.exec_func(div_key.clone(), &["a", "2.0"]));

        assert_eq!("invalid float literal", handler.exec_func(rem_key.clone(), &["2.0", "fd"]));

        assert_eq!("0.6666667", handler.exec_func(div_key.clone(), &["2.0", "3.0"]), "Floats need to be f32");
    }
}