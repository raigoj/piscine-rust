
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}
impl Role {
    pub fn power(&self) -> u32 {
        match self {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}
impl Member {
    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Underboss => Role::Underboss,
            Role::Caporegime => Role::Underboss,
            Role::Soldier => Role::Caporegime,
            Role::Associate => Role::Soldier,
        }
    }
}
pub fn new(name: &str, role: Role, age: u8) -> Member {
    Member { name: name.to_string(), role, age }
}