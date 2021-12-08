use std::fs;

use eyre::Result;


fn calculate_cost(crabs: &Vec<u64>, pos: u64) -> u64 {
    let mut total_cost = 0;
    for i in 0..crabs.len() {
        let mut cost = 0;
        if crabs[i] > pos {
            cost = crabs[i] - pos;
        } else if crabs[i] < pos {
            cost = pos - crabs[i];
        }
        total_cost = total_cost + cost;
    }
    total_cost
}

fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut ans_s: Vec<String> = Vec::new();

    const DIGITS_1: u64 = 2;
    const DIGITS_4: u64 = 4;
    const DIGITS_7: u64 = 3;
    const DIGITS_8: u64 = 7;

    let mut sum = 0;

    for i in 0..lines_s.len() {
        let note_ans_split: Vec<String> = lines_s[i].split("|").map(|x| x.to_string()).collect();
        let ans_s: Vec<String> = note_ans_split[1].trim_start().to_string().split(" ").map(|x| x.to_string()).collect();
        println!("{}", note_ans_split[1]);
        for j in 0..ans_s.len() {
            print!("{}: {}({}), ", j, ans_s[j], ans_s[j].len());
            match ans_s[j].len() as u64 {
                DIGITS_1 => sum = sum + 1,
                DIGITS_4 => sum = sum + 1,
                DIGITS_7 => sum = sum + 1,
                DIGITS_8 => sum = sum + 1,
                _ => (),
            }
        }
        println!("");
    }


    println!("SUM: {}", sum);

    //let mut crabs: Vec<u64> = lines_s[0].split(",").map(|x| x.parse::<u64>().unwrap()).collect();

    //let mut max = 0;
    //for i in 0..crabs.len() {
    //    if crabs[i] > max {
    //        max = crabs[i];
    //    }
    //}
    //println!("MAX: {}", max);

    //let mut costs: Vec<u64> = Vec::new();
    //for i in 0..max {
    //    costs.push(calculate_cost(&crabs, i));
    //}

    //let mut min_cost: u64 = std::u64::MAX;
    //for i in 0..costs.len() {
    //    if costs[i] < min_cost {
    //        min_cost = costs[i];
    //    }
    //}

    //println!("MIN COST: {}", min_cost);

    //for day in 0..80 {
    //    for i in 0..fish.len() {
    //        if fish[i] == 0 {
    //            fish[i] = 6;
    //            fish.push(8);
    //        } else {
    //            fish[i] = fish[i] - 1;
    //        }
    //    }
    //    println!("Day: {}, Num: {}", day + 1, fish.len());
    //}

    Ok(())
}
