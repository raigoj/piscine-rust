pub use chrono::*;

pub type wd = Weekday;

pub fn middle_day(year: i32) -> Option<wd>{
    let start_of_year = NaiveDate::from_yo(year, 1);
    let end_of_year = NaiveDate::from_yo(year+1, 1);
    let mid = end_of_year - start_of_year;
    if mid.num_days() % 2 == 0 {
        return None;
    }

    Some(NaiveDate::from_yo(year, 183).weekday())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_test() {
        let _ = Utc.ymd(2011, 12, 2).and_hms(21, 12, 09);

        assert_eq!(wd::Tue, middle_day(1022).unwrap());
    }

    #[test]
    fn leap_test() {
        assert!(middle_day(1020).is_none());
    }
}