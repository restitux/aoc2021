use std::collections::HashMap;
use std::fs;

use eyre::Result;


fn roll_die(die_num: &mut usize) -> usize {
    let roll = *die_num;
    if *die_num == 100 {
        *die_num = 1;
    } else {
        *die_num += 1;
    }
    println!("ROLL: {}", roll);
    return roll;
}

fn roll(die_num: &mut usize) -> usize {
    roll_die(die_num) + roll_die(die_num) + roll_die(die_num)
}

fn turn(pos: &mut usize, score: &mut usize, die: &mut usize) -> bool {
    let roll = roll(die);
    *pos += roll;
    *pos %= 10;
    *score += *pos + 1;
    if *score >= 1000 {
        return true;
    }
    return false;
}

fn main() -> Result<()> {
    //let filename1 = "input";

    //let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    //let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    //lines_s.pop();


    ////let mut bits = BitVec::new();
    //let mut bits: String = "".to_string();

    ////let vals: Vec<usize> = lines_s[0].chars().map(|x| usize::from_str_radix(x.to_string().as_str(), 16).unwrap()).collect();
    ////let vals: Vec<String> = lines_s[0].chars().map(|x| x.to_string()).collect();


    let mut p1 = 6;
    let mut p2 = 5;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut num_rolls = 0;

    let mut die: usize = 1;


    loop {
        // p1 turn
        let mut win = turn(&mut p1, &mut p1_score, &mut die);
        println!("P1 score: {}", p1_score);
        num_rolls += 3;
        if win {
            println!("P1 WINS");
            println!("RESULT: {}", num_rolls * p2_score);
            break;
        }

        // p2 turn
        win = turn(&mut p2, &mut p2_score, &mut die);
        println!("P2 score: {}", p2_score);
        num_rolls += 3;
        if win {
            println!("P2 WINS");
            println!("RESULT: {}", num_rolls * p1_score);
            break;
        }
    }

    println!("NUM ROLLS: {}", num_rolls);


    Ok(())
}
