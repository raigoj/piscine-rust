mod mobs;
pub use mobs::Mob;
pub use mobs::member;
pub use mobs::boss;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let (mafia1, mafia2) = (
            Mob {
                name: "Hairy Giants".to_string(),
                boss: boss::new("Louie HaHa", 36),
                cities: vec![("San Francisco".to_string(), 7)],
                members: vec![
                    member::new("Benny Eggs", member::Role::Soldier, 28),
                    member::new("Jhonny", member::Role::Associate, 17),
                    member::new("Greasy Thumb", member::Role::Soldier, 30),
                    member::new("No Finger", member::Role::Caporegime, 32),
                ],
                wealth: 100000,
            },
            Mob {
                name: "Red Thorns".to_string(),
                boss: boss::new("Big Tuna", 30),
                cities: vec![("San Jose".to_string(), 5)],
                members: vec![
                    member::new("Knuckles", member::Role::Soldier, 25),
                    member::new("Baldy Dom", member::Role::Caporegime, 36),
                    member::new("Crazy Joe", member::Role::Underboss, 23),
                ],
                wealth: 70000,
            },
        );
        println!("{:?}\n{:?}", mafia1, mafia2);
    }
}