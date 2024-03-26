pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (
        c,
        (c as f64).exp(),
        (c as f64).abs().ln(),
    )
}

pub fn str_function(a: String) -> (String, String) {
    let nbs = a.split(" ").map(|x| x.parse::<f64>().unwrap().exp().to_string()).collect::<Vec<_>>().join(" ");
    (a, nbs)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let logs = b.iter().map(|v| (*v as f64).ln()).collect();
    (b, logs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nbr_function_test() {
        let a: i32 = 0;
        assert_eq!((0, 1.0, -f64::INFINITY), nbr_function(a));
    }

    #[test]
    fn str_function_test() {
        let b = String::from("1 2 4 5 6");

        assert_eq!(
            (
                "1 2 4 5 6".to_string(), 
                "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_string()
            ),
            str_function(b)
        );
    }

    #[test]
    fn vec_function_test() {
        let c = vec![1, 2, 4, 5];

        assert_eq!(
            (
                vec![1, 2, 4, 5],
                vec![
                    0.0,
                    0.6931471805599453,
                    1.3862943611198906,
                    1.6094379124341003
                ]
            ),
            vec_function(c)
        );
    }
}
