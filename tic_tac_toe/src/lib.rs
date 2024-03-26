pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let winner = ["X", "O"].iter().find(|&&player| has_won(player, &table));
    match winner {
        Some(player) => format!("player {} won", player),
        None => "Tie".to_string(),
    }
}
fn has_won(player: &str, table: &Vec<Vec<&str>>) -> bool {
    [diagonals, horizontal, vertical].iter().any(|f| f(player, table))
}
pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    (0..width(table)).zip(0..height(table)).all(|(x, y)| table[y][x] == player) ||
        (0..width(table)).zip((0..height(table)).rev()).all(|(x, y)| table[y][x] == player)
}
pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    (0..height(table)).any(|y|
        (0..width(table)).all(|x|
            table[y][x] == player
        )
    )
}
pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    (0..width(table)).any(|x|
        (0..height(table)).all(|y|
            table[y][x] == player
        )
    )
}
fn height(table: &Vec<Vec<&str>>) -> usize {
    table.len()
}
fn width(table: &Vec<Vec<&str>>) -> usize {
    table[0].len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(
            "Tie",
            tic_tac_toe(vec![
                vec!["O", "X", "O"],
                vec!["O", "O", "X"],
                vec!["X", "#", "X"]
            ])
        );

        assert_eq!(
            "player O won",
            tic_tac_toe(vec![
                vec!["X", "O", "O"],
                vec!["X", "O", "O"],
                vec!["#", "O", "X"]
            ])
        );

        assert_eq!(
            "player X won",
            tic_tac_toe(vec![
                vec!["O", "O", "X"],
                vec!["O", "X", "O"],
                vec!["X", "#", "X"],
            ])
        );
    }

    #[test]
    fn horizontal_test() {
        assert_eq!(
            "player X won",
            tic_tac_toe(vec![
                vec!["O", "O", "X"],
                vec!["X", "O", "O"],
                vec!["X", "X", "X"],
            ])
        );

        assert!(horizontal(
            "X",
            &vec![
                vec!["O", "X", "O", "O"],
                vec!["X", "X", "X", "X"],
                vec!["X", "#", "O", "X"],
                vec!["X", "X", "O", "O"]
            ]
        ));
    }

    #[test]
    fn vertical_test() {
        assert_eq!(
            "player O won",
            tic_tac_toe(vec![
                vec!["O", "X", "O"],
                vec!["O", "#", "O"],
                vec!["X", "X", "O"],
            ])
        );
    }
}