use Antigen::*;
use RhFactor::*;
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}
use std::cmp::{Ord};
use std::str::FromStr;
impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AB" => Ok(AB),
            "A" => Ok(A),
            "B" => Ok(B),
            "O" => Ok(O),
            _ => Err(()),
        }
    }
}
impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Positive),
            "-" => Ok(Negative),
            _ => Err(()),
        }
    }
}
impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(2..=3).contains(&s.len()) {
            return Err(());
        }
        Ok(Self {
            antigen: s[..s.len()-1].parse()?,
            rh_factor: s[s.len()-1..].parse()?,
        })
    }
}
use std::fmt::{self, Debug, Formatter};
impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.antigen, self.rh_factor)
    }
}
impl BloodType {
    #[allow(clippy::match_like_matches_macro)]
    pub fn can_receive_from(&self, other: &Self) -> bool {
        match self {
            Self {antigen: AB, rh_factor: Positive} => true,
            Self {antigen: AB, rh_factor: Negative} => match other {
                Self {rh_factor: Negative, ..} => true,
                _ => false,
            },
            Self {antigen: A, rh_factor: Positive} => match other {
                Self {antigen:O|A, ..} => true,
                _ => false,
            },
            Self {antigen: A, rh_factor: Negative} => match other {
                Self {antigen:O|A, rh_factor: Negative} => true,
                _ => false,
            },
            Self {antigen: B, rh_factor: Positive} => match other {
                Self {antigen:O|B, ..} => true,
                _ => false,
            },
            Self {antigen: B, rh_factor: Negative} => match other {
                Self {antigen:O|B, rh_factor: Negative} => true,
                _ => false,
            },
            Self {antigen: O, rh_factor: Positive} => match other {
                Self {antigen:O, ..} => true,
                _ => false,
            },
            Self {antigen: O, rh_factor: Negative} => match other {
                Self {antigen:O, rh_factor: Negative} => true,
                _ => false,
            },
        }
    }
    const ALL: [BloodType; 8] = [
        Self {antigen: AB, rh_factor: Positive},
        Self {antigen: AB, rh_factor: Negative},
        Self {antigen: A, rh_factor: Positive},
        Self {antigen: A, rh_factor: Negative},
        Self {antigen: B, rh_factor: Positive},
        Self {antigen: B, rh_factor: Negative},
        Self {antigen: O, rh_factor: Positive},
        Self {antigen: O, rh_factor: Negative},
    ];
    pub fn donors(&self) -> Vec<Self> {
        Self::ALL.into_iter().filter(|other| self.can_receive_from(other)).collect()
    }
    pub fn recipients(&self) -> Vec<BloodType> {
        Self::ALL.into_iter().filter(|other| other.can_receive_from(self)).collect()
    }
}