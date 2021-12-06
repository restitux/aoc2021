use std::fs;

use eyre::Result;


fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();


    let mut fish: Vec<u64> = lines_s[0].split(",").map(|x| x.parse::<u64>().unwrap()).collect();

    for day in 0..80 {
        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish[i] = 6;
                fish.push(8);
            } else {
                fish[i] = fish[i] - 1;
            }
        }
        println!("Day: {}, Num: {}", day + 1, fish.len());
    }

    Ok(())
}
