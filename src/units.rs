use crate::playingfield::PlayingField;
use anyhow::{anyhow, Result};
use rand::prelude::*;
use std::fmt;
use std::fmt::Formatter;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Unit {
    Character(CharacterUnit),
}
impl Clone for Unit {
    fn clone(&self) -> Self {
        match self {
            Unit::Character(chara) => Unit::Character(chara.clone()),
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Character(chara) => write!(f, "{}", chara),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Race {
    Goblin,
    Orc,
}
impl fmt::Display for Race {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Race::Goblin => write!(f, "Goblin"),
            Race::Orc => write!(f, "Orc"),
        }
    }
}

impl Race {
    fn weapon(&self) -> String {
        match self {
            Race::Goblin => String::from("Magical Knife 🗡️"),
            Race::Orc => String::from("Big Hammer 🔨")
        }
    }
}

#[derive(Debug)]
pub(crate) struct CharacterUnit {
    pub(crate) name: String,
    health: u16,
    power_level: u8,
    opponent_position: Option<(i32, i32)>,
    race: Race,
    pub(crate) position: (i32, i32),
    pub(crate) alive: bool,
}

impl CharacterUnit {
    pub(crate) fn new(
        name: String,
        health: u16,
        power_level: u8,
        race: Race,
        position: (i32, i32),
        alive: bool,
    ) -> Self {
        CharacterUnit {
            name,
            health,
            power_level,
            opponent_position: None,
            race,
            position,
            alive,
        }
    }
}

impl Clone for CharacterUnit {
    fn clone(&self) -> Self {
        CharacterUnit {
            name: self.name.clone(),
            health: self.health,
            power_level: self.power_level,
            opponent_position: self.opponent_position.clone(), // Clone the Option<Box<MaybeUnit>>
            race: self.race,
            position: self.position,
            alive: self.alive,
        }
    }
}

impl fmt::Display for CharacterUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\"{}\" @ ({}, {})\nRace: {}\nHealth: {}\nPower Level: {}\nFighting:",
            self.name, self.position.0, self.position.1, self.race, self.health, self.power_level
        )?;

        match &self.opponent_position {
            None => write!(f, " Nobody")?,
            Some((x, y)) => write!(f, "Opponent at ({}, {})", x, y)?,
        }

        write!(f, "\n")
    }
}

impl CharacterUnit {
    pub(crate) fn move_unit(
        &mut self,
        playing_field: &mut PlayingField,
        direction: char,
        spaces: i32,
    ) -> Result<(i32, i32)> {
        let mut new_coordinate = self.position.clone();
        match direction {
            'n' => new_coordinate.0 -= spaces,
            's' => new_coordinate.0 += spaces,
            'e' => new_coordinate.1 += spaces,
            'w' => new_coordinate.1 -= spaces,
            'r' => return self.move_unit(playing_field, random_direction(), spaces),
            _ => return Err(anyhow!("Received an unknown direction!: {}", direction)),
        }

        match playing_field.place_unit(self, new_coordinate.0, new_coordinate.1) {
            Ok(existing_opponent) => {
                match existing_opponent {
                    Some(winner) => {
                        if winner.name == self.name {
                            println!("{} won a fight to get to new position", self.name);
                            Ok((new_coordinate.0, new_coordinate.1))
                        } else {
                            println!("Lost a fight and died. RIP: {}", self.name);
                            Ok((-1, -1))
                        }
                    }
                    None => {
                        println!("Unit moved");
                        Ok((new_coordinate.0, new_coordinate.1))
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    pub(crate) fn fight(&mut self, opponent: &mut CharacterUnit) -> bool {
        if self.power_level > opponent.power_level {
            println!("{} DESTROYS {} with their {}", self.name, opponent.name, self.race.weapon());
            true
        } else {
            println!("{} FALLS to {}'s {}", self.name, opponent.name, opponent.race.weapon());
            false
        }
    }
}

fn random_direction() -> char {
    let letters = ['n', 's', 'e', 'w'];
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..letters.len());
    letters[random_index]
}
