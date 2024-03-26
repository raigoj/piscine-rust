pub use chrono::{Utc, NaiveDate};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FErr {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FErr {
    pub fn new(name: String, error: String, err: String) -> FErr {
        Self {
            form_values: (name, error),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub fav_colour: Color,
    pub birth_location: String,
    pub password: String,
}
impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        fav_colour: Color,
        birth_location: String,
        password: String,
    ) -> Self {
        Self {
            first_name,
            last_name,
            birth,
            fav_colour,
            birth_location,
            password,
        }
    }
    pub fn validate(&self) -> Result<Vec<&str>, FErr> {
        if self.first_name.is_empty() {
            return Err(FErr::new("first_name".to_string(),
                                 self.first_name.clone(),
                                 "No user name".to_string(),
            ));
        }
        if self.password.len() < 8 {
            return Err(FErr::new("password".to_string(),
                                 self.password.clone(),
                                 "At least 8 characters".to_string(),
            ));
        }
        if !self.password.chars().any(|ch| ch.is_alphabetic()) ||
            !self.password.chars().any(|ch| ch.is_numeric()) ||
            !self.password.chars().any(|ch| !ch.is_alphanumeric())
        {
            return Err(FErr::new("password".to_string(),
                                 self.password.clone(),
                                 "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string(),
            ));
        }
        Ok(vec!["Valid first name", "Valid password"])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_date(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    fn f(field: &str, input: &str) -> (String, String) {
        (field.to_string(), input.to_string())
    }

    #[test]
    fn form_test() {
        let mut form_output = Form::new(
            String::from("Lee"),
            String::from("Silva"),
            create_date("2015-09-05"),
            Color::Red,
            String::from("Africa"),
            String::from("qwqwsa1dty_"),
        );

        assert_eq!(vec!["Valid first name", "Valid password"], form_output.validate().unwrap());

        form_output.first_name = String::from("");
        assert_eq!(f("first_name", ""), form_output.validate().unwrap_err().form_values);
        assert_eq!("No user name", form_output.validate().unwrap_err().err);
        assert!(
            NaiveDate::parse_from_str(
                &form_output.validate().unwrap_err().date,
                "%Y-%m-%d %H:%M:%S"
            ).is_ok(),
            "Date string in incorrect format"
        );

        form_output.first_name = String::from("as");
        form_output.password = String::from("dty_1");
        assert_eq!(f("password", "dty_1"), form_output.validate().unwrap_err().form_values);
        assert_eq!("At least 8 characters", form_output.validate().unwrap_err().err);


        let err_msg = "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)";

        form_output.password = String::from("asdasASd(_");
        assert_eq!(f("password", "asdasASd(_"), form_output.validate().unwrap_err().form_values);
        assert_eq!(err_msg, form_output.validate().unwrap_err().err);

        form_output.password = String::from("asdasASd123SA");
        assert_eq!(f("password", "asdasASd123SA"), form_output.validate().unwrap_err().form_values);
        assert_eq!(err_msg, form_output.validate().unwrap_err().err);
    }
}