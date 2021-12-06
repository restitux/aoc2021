use std::fs;

use eyre::Result;

fn fish_spawned(days: u64, val: u64) -> u64 {
    let mut spawned = 0;
    let mut fish: u64 = val;
    for day in 0..days {
        if fish == 0 {
            fish = 6;
            spawned = spawned + 1 + fish_spawned(days - day - 1, 8);
        } else {
            fish = fish - 1;
        }
    }
    spawned
}

fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut fish: Vec<u64> = lines_s[0]
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut total = 0;

    for i in 0..fish.len() {
        total = total + 1 + fish_spawned(80, fish[i]);
    }
    println!("TOTAL: {}", total);

    Ok(())
}
