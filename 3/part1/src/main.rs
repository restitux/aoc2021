use std::fs;

use eyre::Result;

fn main() -> Result<()> {
    let filename = "input";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut ms: Vec<&str> = contents.split("\n").collect();
    ms.pop();

    let val_width = 12;
    let mut num_vals = Vec::<u64>::new();
    for j in 0..val_width {
        num_vals.push(0);
    }

    for i in 0..ms.len() {
        let val = u64::from_str_radix(ms[i], 2).unwrap();
        for j in 0..val_width {
            let mask = 0x1 << j;
            let val_masked = val & mask;
            let digit_is = val_masked >> j;
            num_vals[j] = num_vals[j] + digit_is;
        }
    }

    let threshold = (ms.len() / 2) as u64;

    let mut epsilon: u64 = 0;
    let mut gamma: u64 = 0;

    for j in 0..val_width {
        if num_vals[j] > threshold {
            epsilon = epsilon | (0x1 << j);
        } else {
            gamma = gamma | (0x1 << j);
        }
    }


    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);

    let output = epsilon * gamma;
    println!("OUTPUT: {}", output);

    //let mut pos: (i64, i64) = (0, 0);
    //let mut aim = 0;

    //for i in 0..ms.len() {
    //    let step: Vec<&str> = ms[i].split(" ").collect();
    //    match step[0] {
    //        "forward" => {
    //            pos.0 = pos.0 + step[1].parse::<i64>().unwrap();
    //            pos.1 = pos.1 + (aim * step[1].parse::<i64>().unwrap());
    //        }
    //        "up" => aim = aim - step[1].parse::<i64>().unwrap(),
    //        "down" => aim = aim + step[1].parse::<i64>().unwrap(),
    //        _ => panic!("UNKNOWN DIRECTION {}", step[0])
    //    }
    //}

    //println!("POS: {}, {}", pos.0, pos.1);
    //println!("OUTPUT: {}", pos.0 * pos.1);
    Ok(())
}
