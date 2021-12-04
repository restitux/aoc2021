use std::fs;

use eyre::Result;

fn main() -> Result<()> {
    let filename = "input";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut ms: Vec<&str> = contents.split("\n").collect();
    ms.pop();

    let mut pos: (i64, i64) = (0, 0);
    let mut aim = 0;

    for i in 0..ms.len() {
        let step: Vec<&str> = ms[i].split(" ").collect();
        match step[0] {
            "forward" => {
                pos.0 = pos.0 + step[1].parse::<i64>().unwrap();
                pos.1 = pos.1 + (aim * step[1].parse::<i64>().unwrap());
            }
            "up" => aim = aim - step[1].parse::<i64>().unwrap(),
            "down" => aim = aim + step[1].parse::<i64>().unwrap(),
            _ => panic!("UNKNOWN DIRECTION {}", step[0])
        }
    }

    println!("POS: {}, {}", pos.0, pos.1);
    println!("OUTPUT: {}", pos.0 * pos.1);
    Ok(())
}
