use std::cmp::Ordering;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nbr_of_games: u16,
}
impl GameSession {
    pub fn new(i: u32, pl1: String, pl2: String, n: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id: i,
            p1: (pl1, 0),
            p2: (pl2, 0),
            nbr_of_games: n,
        })
    }
    pub fn read_winner(&self) -> (String, u16) {
        match self.p1.1.cmp(&self.p2.1) {
            Ordering::Greater => self.p1.clone(),
            Ordering::Equal => ("Same score! tied".to_string(), self.p1.1),
            Ordering::Less => self.p2.clone(),
        }
    }
    pub fn update_score(&mut self, user_name: String) {
        if self.is_ended() {
            return
        }
        let mut quick_fix = ("Same score! tied".to_string(),  self.p1.1);
        let p = match user_name {
            name if self.p1.0 == name => &mut self.p1,
            name if self.p2.0 == name => &mut self.p2,
            _ => &mut quick_fix //panic!("Invalid user_name")
        };
        p.1 += 1;
    }
    fn is_ended(&self) -> bool {
        self.p1.1 + self.p2.1 >= self.nbr_of_games
            || self.p1.1 > self.nbr_of_games / 2
            || self.p2.1 > self.nbr_of_games / 2
    }
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_box_test() {
        let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);

        assert_eq!(("Same score! tied".to_string(), 0), game.read_winner());

        game.update_score(String::from("Joao"));
        game.update_score(String::from("Joao"));
        game.update_score(String::from("Susana"));
        game.update_score(String::from("Susana"));

        assert_eq!(("Same score! tied".to_string(), 2), game.read_winner());

        game.update_score(String::from("Joao"));
        game.update_score(String::from("Susana")); // this one will not count because it already 5 games played, the nbr_of_games

        assert_eq!(("Joao".to_string(), 3), game.read_winner());

        assert_eq!("game deleted: id -> 0", game.delete());
    }
}