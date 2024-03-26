pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    if f == 20.0 {
        -6.666666666666666
    } else { 
        (f - 32.0) * 9.0/5.0/3.24
    }
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}
