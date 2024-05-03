// units.rs
use std::cmp::max;
use rand::{Rng, thread_rng};
use std::fmt;
use enum_derived::Rand;
use rnglib::{Language, RNG};
use crate::battleground::Battleground;
use crate::utilities::print_message;

const MESSAGE_COORDINATES: (u16, u16) = (5, 15);

#[derive(Debug, Clone, Rand, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Race {
    Goblin,
    Orc,
    Elf,
    Dwarf,
    Skeleton,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Race::Goblin => write!(f, "Goblin"),
            Race::Orc => write!(f, "Orc"),
            Race::Elf => write!(f, "Elf"),
            Race::Dwarf => write!(f, "Dwarf"),
            Race::Skeleton => write!(f, "Skeleton"),
        }
    }
}

impl Race {
    fn weapon(&self) -> &str {
        match self {
            Race::Goblin => "Rusty Knife 🔪",
            Race::Orc => "Big Club 🏏",
            Race::Elf => "Magical Sword 🗡️",
            Race::Dwarf => "Big Hammer 🔨",
            Race::Skeleton => "Bone 🦴",
        }
    }
    pub(crate) fn symbol(&self) -> &str {
        match self {
            Race::Goblin => "👺",
            Race::Orc => "🧌",
            Race::Elf => "🧝",
            Race::Dwarf => "🧔🏽",
            Race::Skeleton => "🩻",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharacterUnit {
    pub name: String,
    pub(crate) health: u16,
    pub(crate) power_level: u8,
    pub race: Race,
    pub position: (i32, i32),
    pub(crate) alive: bool,
}

impl CharacterUnit {
    pub fn new(
        name: String,
        health: u16,
        power_level: u8,
        race: Race,
        position: (i32, i32),
    ) -> Self {
        CharacterUnit {
            name,
            health,
            power_level,
            race,
            position,
            alive: true,
        }
    }

    pub fn new_random() -> Self {
        let race = Race::rand();
        let name = random_name(race.clone());
        let health = 100;
        let power_level = random_powerlevel();
        let position = (-1, -1);
        CharacterUnit::new(
            name,
            health,
            power_level,
            race,
            position,
        )
    }

    pub fn move_unit(
        &mut self,
        playing_field: &mut Battleground,
        direction: char,
        spaces: i32,
    ) -> Option<(i32, i32)> {
        match playing_field.move_unit(self, direction, spaces) {
            Some(position) => {
                self.position = position;
                Option::from(position)
            }
            None => None
        }
    }

    pub fn move_unit_randomly(&mut self, bg: &mut Battleground) -> Option<(i32, i32)> {
        let mut rng = thread_rng();
        let range = 1..=max(bg.height, bg.width);
        let spaces = rng.gen_range(range);

        self.move_unit(bg,'r', spaces as i32)
    }

    pub fn fight(&mut self, opponent: &CharacterUnit) -> bool {
        if self.power_level > opponent.power_level {
            print_message(format!(
                "{} DESTROYS {} with their {}",
                self.name,
                opponent.name,
                self.race.weapon())
            , MESSAGE_COORDINATES);
            true
        } else if self.power_level < opponent.power_level {
            print_message(format!(
                "{} FALLS to {}'s {}",
                self.name,
                opponent.name,
                opponent.race.weapon())
            , MESSAGE_COORDINATES);
            false
        }
        else {
            print_message("An equal matched battle!", (20, 20));
            return thread_rng().gen_bool(0.5)
        }
    }
}

fn random_name(race: Race) -> String {
    let rng;
    match race {
        Race::Elf => {rng = RNG::try_from(&Language::Elven).unwrap()},
        Race::Orc => {rng = RNG::try_from(&Language::Demonic).unwrap()},
        Race::Goblin => {rng = RNG::try_from(&Language::Goblin).unwrap()},
        Race::Dwarf => {rng = RNG::try_from(&Language::Roman).unwrap()},
        Race::Skeleton => {rng = RNG::try_from(&Language::Curse).unwrap()},
    }
    rng.generate_name()
}

fn random_powerlevel() -> u8 {
    let mut rng = thread_rng();
    let random_u8: u8 = rng.gen();
    random_u8
}
