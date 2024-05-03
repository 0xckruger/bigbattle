// battleground.rs
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use crossterm::{cursor, QueueableCommand};
use crossterm::style::Print;
use rand::prelude::IndexedRandom;
use rand::{Rng, thread_rng};

use crate::units::{CharacterUnit, Race};
use crate::utilities::print_message;
use std::io::Write as IoWrite;
use std::fmt::Write as FmtWrite;
use std::thread;
use std::time::Duration;

const MESSAGE_COORDINATES: (u16, u16) = (0, 8);

pub struct Battleground {
    pub(crate) width: usize,
    pub(crate) height: usize,
    units: HashMap<(i32, i32), CharacterUnit>,
    player_count: i32,
}

impl Battleground {
    pub fn new(width: usize, height: usize) -> Self {
        Battleground {
            width,
            height,
            units: HashMap::new(),
            player_count: 0,
        }
    }

    #[allow(dead_code)]
    pub fn get_player_count(&self) -> i32 { self.player_count }

    pub fn print_race_counts(&self) {
        let race_counts = self.units.values().fold(HashMap::new(), |mut acc, unit| {
            *acc.entry(unit.race.clone()).or_insert(0) += 1;
            acc
        });

        let mut sorted_race_counts: Vec<(Race, u32)> = race_counts.into_iter().collect();
        sorted_race_counts.sort_by(|(race1, _), (race2, _)| race1.cmp(race2));

        let mut output = String::new();
        let mut first = true;

        for (race, count) in sorted_race_counts {
            if !first {
                FmtWrite::write_fmt(&mut output, format_args!(" | ")).unwrap();
            }
            FmtWrite::write_fmt(&mut output, format_args!("{:?}: {}", race, count)).unwrap();
            first = false;
        }

        print_message(format!("Current race counts: {}", output), (5, 18));
    }

    pub(crate) fn is_race_dominant(&self) -> Option<Race> {
        let race_counts = self.units.values().fold(HashMap::new(), |mut acc, unit| {
            *acc.entry(unit.race.clone()).or_insert(0) += 1;
            acc
        });

        let total_units = self.units.len();
        let dominant_race = race_counts.into_iter().find(|(_, count)| *count == total_units);

        match dominant_race {
            Some((race, _)) => Some(race),
            None => None,
        }
    }

    pub fn add_unit(&mut self, unit: CharacterUnit) {
        self.units.insert(unit.position, unit);
        self.player_count += 1;
    }

    pub fn add_unit_random_position(&mut self, mut unit: CharacterUnit) -> Result<()>{
        let position = self.random_position();
        match position {
            Some(coordinate) => {
                unit.position = coordinate;
                self.add_unit(unit);
                Ok(())
            }
            None => {
                Err(anyhow!("Couldn't place unit. Not enough space?"))
            }
        }
    }

    pub fn move_unit(
        &mut self,
        unit: &mut CharacterUnit,
        direction: char,
        spaces: i32,
    ) -> Option<(i32, i32)> {

        let supposed_position = self.units.get(&unit.position);
        match supposed_position {
            Some(char) => {
                if unit.name != char.name {
                    panic!("Fucked up positioning")
                }
            }
            None => {
                panic!("Doesn't even exist, fucked up")
            }
        }

        let mut new_position = unit.position;
        match direction {
            'n' => new_position.0 -= spaces,
            's' => new_position.0 += spaces,
            'e' => new_position.1 += spaces,
            'w' => new_position.1 -= spaces,
            'r' => return self.move_unit(unit, random_direction(), spaces),
            _ => return None,
        }

        if self.valid_position(&new_position) {
            let opponent = self.first_unit_between(&unit.position, new_position).cloned();
            if let Some(mut opponent) = opponent {
                if unit.race == opponent.race { return None }

                if unit.fight(&mut opponent) {
                    opponent.alive = false;
                    opponent.health = 0;
                    self.units.remove(&opponent.position);
                    print_death_emoji(opponent.position);
                    self.units.remove(&unit.position);
                    unit.position = new_position;
                    unit.power_level += 1;
                    self.units.insert(new_position, unit.clone());
                    self.player_count -= 1;
                    Some(new_position)
                } else {
                    opponent.power_level += 1;
                    unit.alive = false;
                    unit.health = 0;
                    self.units.remove(&unit.position);
                    self.player_count -= 1;
                    print_death_emoji(unit.position);
                    None
                }
            } else {
                self.units.remove(&unit.position);
                unit.position = new_position;
                self.units.insert(new_position, unit.clone());
                Some(new_position)
            }
        } else {
            None
        }
    }

    fn first_unit_between(
        &self,
        start: &(i32, i32),
        end: (i32, i32),
    ) -> Option<&CharacterUnit> {
        let mut x = start.0;
        let mut y = start.1;

        while (x, y) != end {
            let (dx, dy) = get_direction(x, y, end.0, end.1);
            x += dx;
            y += dy;

            if let Some(unit) = self.units.get(&(x, y)) {
                return Some(unit);
            }
        }
        None
    }
    fn valid_position(&self, position: &(i32, i32)) -> bool {
        position.0 >= 0
            && position.0 < self.height as i32
            && position.1 >= 0
            && position.1 < self.width as i32
    }

    pub fn random_position(&self) -> Option<(i32, i32)> {
        let mut tries = 0;
        let mut rng = thread_rng();
        let mut x = 0; let mut y = 0;
        while self.unit_at(&(x, y)).is_some() {
            x = rng.gen_range(0..self.width) as i32;
            y = rng.gen_range(0..self.height) as i32;
            tries += 1;
            if tries > self.height * self.width {
                return None
            }
        }
        Some((x, y))
    }

    fn unit_at(&self, coordinate: &(i32, i32)) -> Option<&CharacterUnit> {
        if let Some(character) = self.units.get(coordinate) {
            Some(character)
        } else {
            None
        }

    }

    pub fn get_field_string(&self) -> () {
        let mut output = String::new();
        let mut stdout = std::io::stdout();

        // Move the cursor to the desired starting position
        output.push_str(&cursor::MoveTo(0, 2).to_string());

        for y in 0..self.height {
            for x in 0..self.width {
                let position = (x as i32, y as i32);
                if let Some(unit) = self.units.get(&position) {
                    output.push_str(&format!("{:?}", unit.race.symbol()))
                } else {
                    output.push_str(". ");
                }
            }
            output.push('\n');
        }

        // Flush the output to the terminal
        stdout.queue(Print(output)).unwrap();
        stdout.flush().unwrap();

    }
    pub fn get_and_move_random_unit(&mut self) -> Option<CharacterUnit> {
        let mut rng = thread_rng();

        let keys: Vec<(i32, i32)> = self.units.keys()
            .filter(|key| self.units.get(key).map_or(false, |unit| unit.alive))
            .cloned()
            .collect();

        if keys.is_empty() {
            print_message("No unit could be found", MESSAGE_COORDINATES);
            return None;
        }

        let key = keys.choose(&mut rng);

        let mut unit = self.units.get_mut(key.expect("Bad key")).cloned().expect("Could not clone");
        let _ = unit.move_unit_randomly(self);
        Some(unit)
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

fn random_direction() -> char {
    let directions = ['n', 's', 'e', 'w'];
    let mut rng = thread_rng();
    *directions.choose(&mut rng).unwrap()
}

fn print_death_emoji(position: (i32, i32)) {
    let mut stdout = std::io::stdout();
    let pos0: u16 = (position.0 + 2) as u16;
    stdout
        .queue(cursor::MoveTo(position.1 as u16, pos0))
        .unwrap();
    stdout.queue(Print("☠️")).unwrap();
    stdout.flush().unwrap();

    thread::sleep(Duration::from_millis(120));

    stdout
        .queue(cursor::MoveTo(position.1 as u16, position.0 as u16))
        .unwrap();
    stdout.queue(Print(" ")).unwrap();
    stdout.flush().unwrap();
}