// main.rs

use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use crate::battleground::Battleground;


mod battleground;
mod units;
mod utilities;


/// A simple game with units on a playing field
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Height of the playing field
    #[arg(value_parser = clap::value_parser!(u16).range(1..=100))]
    height: u16,

    /// Width of the playing field
    #[arg(value_parser = clap::value_parser!(u16).range(1..=100))]
    width: u16,

    /// Number of players (units)
    #[arg(value_parser = clap::value_parser!(u16).range(1..=50))]
    player_count: u16,

    /// Time in milliseconds between each unit move
    #[arg(value_parser = clap::value_parser!(u64))]
    milliseconds: u64,
}


fn initialize_battleground(height: i32, width: i32, player_count: i32) -> Battleground {
    let mut battleground = Battleground::new(width as usize, height as usize);
    for _ in 0..player_count {
        let new_unit = units::CharacterUnit::new_random();
        match battleground.add_unit_random_position(new_unit) {
            Ok(()) => continue,
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
    battleground
}

fn start_battle(mut bg: Battleground, print_sleep_duration: Duration) {
    while let None = bg.is_race_dominant() {
        bg.get_and_move_random_unit();
        bg.print_race_counts();
        print_battleground(&bg, print_sleep_duration).expect("Couldn't print battlefield");
    }
    println!("It's over! The {}s won! THE END", bg.is_race_dominant().unwrap());

}

fn print_battleground(battleground: &Battleground, sleep_ms: Duration) -> Result<(), std::io::Error> {
    battleground.get_field_string();
    sleep(sleep_ms);

    Ok(())
}

fn main() {
    let args = Args::parse();

    let sleep_duration = Duration::from_millis(args.milliseconds);

    let battleground = initialize_battleground(args.height.into(), args.width.into(), args.player_count.into());

    start_battle(battleground, sleep_duration);
}