use crate::units::CharacterUnit;
use anyhow::{anyhow, Result};
use std::fmt;
use std::fmt::Formatter;

pub(crate) struct PlayingField {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) units: Vec<Vec<Option<CharacterUnit>>>,
}
impl fmt::Display for PlayingField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.units {
            write!(f, "|")?;
            for value in row {
                match value {
                    None => write!(f, " X |")?,
                    Some(chara) => write!(f, " {} |", chara.name)?,
                }
            }
            writeln!(f)?;
        }
        println!();
        Ok(())
    }
}

impl PlayingField {
    pub(crate) fn initialize(&mut self) {
        self.units = (0..self.height).map(|_| vec![None; self.width]).collect();
    }

    pub(crate) fn unit_at(&self, x: i32, y: i32) -> Result<&Option<CharacterUnit>> {
        if self.valid_coordinate(x, y) {
            Ok(&self.units[x as usize][y as usize])
        } else {
            Err(anyhow!(
                "Coordinates ({}, {}) are invalid for the PlayingField",
                x,
                y
            ))
        }
    }

    pub(crate) fn place_unit(
        &mut self,
        character: &mut CharacterUnit,
        x: i32,
        y: i32,
    ) -> Result<Option<&CharacterUnit>> {
        let original_coordinates = character.position;
        if self.valid_coordinate(x, y) {
            let existing_character = self.first_unit_between(&original_coordinates.0, &original_coordinates.1, x, y);

            if let Some(existing_character) = existing_character {
                eprintln!(
                    "Can't place unit there! {} already at ({}, {})",
                    existing_character.name,
                    existing_character.position.0,
                    existing_character.position.1
                );
                {
                    if character.fight(existing_character) {
                        self.kill_unit(existing_character);
                        self.move_unit(character, x, y)?;
                        Ok(Some(character)) // New unit won
                    } else {
                        self.kill_unit(character);
                        Ok(Option::from(self.get_unit_ref(existing_character.position.0, existing_character.position.1))) // Old unit won
                    }
                }

            } else {
                {
                    let mut temp_borrow = &mut *self;
                    temp_borrow.move_unit( character, x, y)
                }
            }
        } else {
            Err(anyhow!(
                "Invalid coordinate provided for move: {}, {}",
                x,
                y
            ))
        }
    }

    fn is_valid_placement(&self, character: &CharacterUnit, start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> Result<(), anyhow::Error> {
        if let Some(existing_character) = self.first_unit_between(&start_x, &start_y, end_x, end_y) {
            return Err(anyhow!(
                "Can't place unit there! {} already at ({}, {})",
                existing_character.name,
                existing_character.position.0,
                existing_character.position.1
            ));
        }
        Ok(())
    }

    pub(crate) fn kill_unit(&mut self, character: &mut CharacterUnit) {
        let dead_unit_location = &mut self.units[character.position.0 as usize][character.position.1 as usize];
        *dead_unit_location = None;
        character.alive = false;
    }


    fn move_unit(
        &mut self,
        character: &mut CharacterUnit,
        x: i32,
        y: i32,
    ) -> Result<Option<&CharacterUnit>> {
        let old_unit_location = &mut self.units[character.position.0 as usize][character.position.1 as usize];
        *old_unit_location = None;
        self.units[x as usize][y as usize] = Some(character.clone());
        character.position = (x, y);
        Ok(None)
    }

    fn first_unit_between(&self, start_x: &i32, start_y: &i32, end_x: i32, end_y: i32) -> Option<&mut CharacterUnit> {
        let mut x = *start_x;
        let mut y = *start_y;

        while (x, y) != (end_x, end_y) {
            let (dx, dy) = get_direction(x, y, end_x, end_y);
            x += dx;
            y += dy;

            if let Some(character) = self.get_unit(x, y) {
                return Some(character);
            }
        }
        None
    }

    fn get_unit_ref(&self, x: i32, y: i32) -> &Option<CharacterUnit> {
        &self.units[x as usize][y as usize]
    }

    pub(crate) fn get_unit_mut(&mut self, x: i32, y: i32) -> Option<&mut CharacterUnit> {
        if self.valid_coordinate(x, y) {
            let row = x as usize;
            let col = y as usize;
            self.units[row][col].as_mut().map(|c| c)
        } else {
            None
        }
    }

    fn valid_coordinate(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            eprintln!("Coordinates ({}, {}) are less than 0", x, y);
            false
        } else if x > self.width as i32 || y > self.height as i32 {
            eprintln!("Coordinates ({}, {}) disobey PlayingField dimensions", x, y);
            false
        } else {
            true
        }
    }
}

fn get_direction(x: i32, y: i32, end_x: i32, end_y: i32) -> (i32, i32) {
    if x < end_x {
        (1, 0)
    } else if x > end_x {
        (-1, 0)
    } else if y < end_y {
        (0, 1)
    } else {
        (0, -1)
    }
}
