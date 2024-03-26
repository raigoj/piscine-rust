pub fn edit_distance(source: &str, target: &str) -> usize {
    println!("source: \"{}\"; target: \"{}\"", source, target);
    if source.is_empty() || target.is_empty() {
        return source.len().min(target.len());
    };
    let source: Vec<_> = source.chars().collect();
    let mut costs: Vec<_> = (0..=source.len()).into_iter().collect();
    println!("{:?}", costs);
    for (i, ch) in target.chars().enumerate() {
        costs = cycle(&source, ch, i+1, costs);
        println!("{:?}", costs);
    };
    costs.remove(costs.len()-1)
}
pub fn cycle(source: &[char], target_char: char, target_i: usize, mut prev_costs: Vec<usize>) -> Vec<usize> {
    let mut top_left = *prev_costs.first().unwrap();
    prev_costs[0] = target_i;
    for (i, ch) in source.iter().enumerate() {
        let mut new = prev_costs[i].min(top_left).min(prev_costs[i+1]);
        if *ch != target_char {new += 1};
        top_left = prev_costs[i+1];
        prev_costs[i+1] = new;
    }
    prev_costs
}
#[cfg(test)]
mod tests {
    use crate::edit_distance;
    #[test]
    fn edit_distance_test() {
        assert_eq!(2, edit_distance("alignment", "assignment"));
        assert_eq!(2, edit_distance("gumbo", "gambol"));
        assert_eq!(3, edit_distance("elephant", "relevant"));
        assert_eq!(8, edit_distance("Google", "Facebook"));
    }
}
