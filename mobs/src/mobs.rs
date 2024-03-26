use crate::boss::Boss;
use crate::member::{Member, Role};
pub mod boss;
pub mod member;
#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}
impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(member::new(name, Role::Associate, age))
    }
    pub fn attack(&mut self, target: &mut Mob) {
        let my_power = self.members.iter().fold(0, |acc, mem| acc + mem.role.power());
        let target_power = target.members.iter().fold(0, |acc, mem| acc + mem.role.power());
        if my_power <= target_power {
            self.members.pop();
            return;
        }
        target.members.pop();
        if target.members.is_empty() {
            self.steal(target, u32::MAX);
            self.cities.append(&mut target.cities);
        }
    }
    pub fn steal(&mut self, target: &mut Mob, amount: u32) {
        let amount = amount.min(target.wealth);
        self.wealth += amount;
        target.wealth -= amount;
    }
    pub fn conquer_city(&mut self, mobs: Vec<Mob>, city_name: String, n: u8) {
        if mobs.into_iter()
            .any(|mob| mob.cities.iter()
                    .any(|(name, _)| name == &city_name)
            ) {
            return
        }
        self.cities.push((city_name, n));
    }
}