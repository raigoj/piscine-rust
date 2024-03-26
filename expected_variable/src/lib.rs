extern crate core;
use case::CaseExt;

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    println!("compare: `{}`, expected: `{}`", compare, expected);
    if !test(compare) && !test(expected) {
        return None
    }
    let compare = compare.to_ascii_lowercase();
    let expected = expected.to_ascii_lowercase();
    let distance = edit_distance(&compare, &expected);
    println!("{}", distance);
    let max_len = compare.len().max(expected.len());
    let percent = (max_len as f64 - distance as f64) / max_len as f64 * 100.0;
    if percent < 50.0 {
        return None
    }
    Some(percent.round().to_string() + "%")
}

fn test(s: &str) -> bool {
    !s.contains(' ') && !s.contains('-')
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!(
            "{} close to it",
            expected_variable("On_Point", "on_point").unwrap()
        );
        println!(
            "{} close to it",
            expected_variable("soClose", "So_Close").unwrap()
        );
        println!(
            "{:?}",
            expected_variable("something", "something_completely_different")
        );
        println!(
            "{} close to it",
            expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
        );
    }
}
pub fn edit_distance(source: &str, target: &str) -> usize {
    if source.is_empty() || target.is_empty() {
        return source.len().min(target.len());
    };
    let source: Vec<_> = source.chars().collect();
    let mut costs: Vec<_> = (0..=source.len()).into_iter().collect();
    for (i, ch) in target.chars().enumerate() {
        costs = cycle(&source, ch, i+1, costs);

    };
    costs.remove(costs.len()-1)
}
pub fn cycle(source: &[char], target_char: char, target_i: usize, mut prev_costs: Vec<usize>) -> Vec<usize> {
    let mut top_left = prev_costs[0];
    prev_costs[0] = target_i;
    for (i, ch) in source.iter().enumerate() {
        let mut new = prev_costs[i].min(top_left).min(prev_costs[i+1]);
        if new != top_left || *ch != target_char {new += 1};
        top_left = prev_costs[i+1];
        prev_costs[i+1] = new;
    }
    prev_costs
}