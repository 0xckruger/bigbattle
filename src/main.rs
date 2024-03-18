use crate::playingfield::PlayingField;
use crate::units::{CharacterUnit, Race};

mod playingfield;
mod units;

fn main() {
    let mut battleground = PlayingField {
        height: 5,
        width: 5,
        units: vec![],
    };
    battleground.initialize();

    let mut goblin1 = CharacterUnit::new("Gorgo".to_string(), 100, 1, Race::Goblin, (0, 0), true);
    let mut orc1 = CharacterUnit::new("Orko".to_string(), 100, 1, Race::Orc, (4, 4), true);
    let orc2 = CharacterUnit::new("Orgo".to_string(), 100, 2, Race::Orc, (1, 3), true);

    battleground.units[0][0] = Some(goblin1.clone());
    battleground.units[4][4] = Some(orc1.clone());
    battleground.units[2][1] = Some(orc2.clone());

    match battleground.unit_at(0, 0) {
        Ok(unit) => match unit {
            Some(c) => println!("{}", c),
            None => println!("Empty space"),
        },
        Err(e) => println!("ERROR: {}", e),
    }

    print!("{}", battleground);

    match goblin1.move_unit(&mut battleground, 's', 1) {
        Ok(new_position) => {
            println!(
                "{goblin1} is at {}, {} now",
                new_position.0, new_position.1
            );
            print!("{}", battleground);
        }
        Err(e) => eprintln!("{}", e),
    }

    match orc1.move_unit(&mut battleground, 'n', 1) {
        Ok(new_position) => {
            println!(
                "{orc1} is at {}, {} now",
                new_position.0, new_position.1
            );
            print!("{}", battleground);
        }
        Err(e) => eprintln!("{}", e),
    }

    match orc1.move_unit(&mut battleground, 'w', 3) {
        Ok(new_position) => {
            println!(
                "{orc1} is at {}, {} now",
                new_position.0, new_position.1
            );
            print!("{}", battleground);
        }
        Err(e) => eprintln!("{}", e),
    }

    match goblin1.move_unit(&mut battleground, 'e', 1) {
        Ok(new_position) => {
            println!(
                "{goblin1} is at {}, {} now",
                new_position.0, new_position.1
            );
            print!("{}", battleground);
        }
        Err(e) => eprintln!("{}", e),
    }

    match orc1.move_unit(&mut battleground, 'n', 1) {
        Ok(new_position) => {
            println!(
                "{orc1} is at {}, {} now",
                new_position.0, new_position.1
            );
            print!("{}", battleground);
        }
        Err(e) => eprintln!("{}", e),
    }
}
