use json::object;
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}
struct Tally {
    cals: f64,
    carbs: f64,
    proteins: f64,
    fats: f64,
}
impl Tally {
    fn new() -> Self {
        Self {
            cals: 0.0,
            carbs: 0.0,
            proteins: 0.0,
            fats: 0.0
        }
    }
    fn format(&mut self) {
        self.cals = round_to(self.cals, 2);
        self.carbs = round_to(self.carbs, 2);
        self.proteins = round_to(self.proteins, 2);
        self.fats = round_to(self.fats, 2);
    }
}
pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut tally = foods.into_iter().fold(Tally::new(), |mut t, f| {
        let n = f.nbr_of_portions;
        t.cals += n * f.calories[1].strip_suffix("kcal").unwrap().parse::<f64>().unwrap();
        t.carbs += n * f.carbs;
        t.proteins += n * f.proteins;
        t.fats += n * f.fats;
        t
    });
    tally.format();
    let res = object! {
        "cals": tally.cals,
        "carbs": tally.carbs,
        "proteins": tally.proteins,
        "fats": tally.fats
    };
    res
}
fn round_to(f: f64, dec: i32) -> f64 {
    let mul = 10.0_f64.powi(dec);
    (f * mul).round() / mul
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn food_test() {
        let a = vec![
            Food {
                name: String::from("big mac"),
                calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
                proteins: 27.0,
                fats: 26.0,
                carbs: 41.0,
                nbr_of_portions: 2.0,
            },
            Food {
                name: "pizza margherita".to_string(),
                calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
                proteins: 13.89,
                fats: 11.21,
                carbs: 49.07,
                nbr_of_portions: 4.9,
            },
        ];
        let tally = calculate_macros(a);
        assert_eq!(2777.39, tally["cals"].as_f64().unwrap());
        assert_eq!(322.44, tally["carbs"].as_f64().unwrap());
        assert_eq!(122.06, tally["proteins"].as_f64().unwrap());
        assert_eq!(106.93, tally["fats"].as_f64().unwrap());
    }
}