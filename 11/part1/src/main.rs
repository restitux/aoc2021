use std::fs;

use eyre::Result;

//fn error_val(c: &str) -> usize {
//    match c {
//        ")" => 3,
//        "]" => 57,
//        "}" => 1197,
//        ">" => 25137,
//        _ => panic!("Invalid char"),
//    }
//}

struct Octopus {
    energy: usize,
    flashed: bool,
}


fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut ops: Vec<Vec<Octopus>> = Vec::new();

    for i in 0..lines_s.len() {
        let x: Vec<Octopus> = lines_s[i].chars().map(|x| Octopus {energy: x.to_string().parse::<usize>().unwrap(), flashed: false}).collect();
        ops.push(x);
    }

    let mut total_flashes = 0;

    for t in 0..100 {
        println!("t: {}", t);
        for i in 0..ops.len() {
            for j in 0..ops[i].len() {
                ops[i][j].energy = ops[i][j].energy + 1;
            }
        }
        'flashing: loop {
            let mut flashes = false;
            for i in 0..ops.len() {
                for j in 0..ops[i].len() {
                    if (ops[i][j].energy > 9) && !ops[i][j].flashed {
                        println!("({}, {}) flashed", i, j);
                        total_flashes = total_flashes + 1;
                        flashes = true;
                        ops[i][j].flashed = true;
                        if i > 0 {
                            ops[i - 1][j].energy = ops[i - 1][j].energy + 1;
                            if j > 0 {
                                ops[i - 1][j - 1].energy = ops[i - 1][j - 1].energy + 1;
                            }
                            if j < (ops[i].len() - 1) {
                                ops[i - 1][j + 1].energy = ops[i - 1][j + 1].energy + 1;
                            }
                        }
                        if i < (ops.len() - 1) {
                            ops[i + 1][j].energy = ops[i + 1][j].energy + 1;
                            if j > 0 {
                                ops[i + 1][j - 1].energy = ops[i + 1][j - 1].energy + 1;
                            }
                            if j < (ops[i].len() - 1) {
                                ops[i + 1][j + 1].energy = ops[i + 1][j + 1].energy + 1;
                            }
                        }
                        if j > 0 {
                            ops[i][j - 1].energy = ops[i][j - 1].energy + 1;
                        }
                        if j < (ops[i].len() - 1) {
                            ops[i][j + 1].energy = ops[i][j + 1].energy + 1;
                        }
                    }
                }
            }
            if !flashes {
                break 'flashing;
            }
        }
        for i in 0..ops.len() {
            for j in 0..ops[i].len() {
                if ops[i][j].flashed {
                    ops[i][j].energy = 0;
                    ops[i][j].flashed = false;
                }
            }
        }
    }

    println!("TOTAL FLASHES: {}", total_flashes);

    //let mut error_score = 0;

    //for i in 0..lines.len() {
    //    // for each line
    //    let mut stack: Vec<String> = Vec::new();
    //    for j in 0..lines[i].len() {
    //        match lines[i][j].as_str() {
    //            "(" => stack.push(lines[i][j].clone()),
    //            "[" => stack.push(lines[i][j].clone()),
    //            "{" => stack.push(lines[i][j].clone()),
    //            "<" => stack.push(lines[i][j].clone()),
    //            ")" => {
    //                match stack.pop() {
    //                    Some(c) => match c.as_str() {
    //                        "(" => (),//println!("FOUND MATCH"),
    //                        _ => {
    //                            println!("ERROR");
    //                            error_score = error_score + error_val(")");
    //                        },
    //                    },
    //                    None => continue,
    //                }
    //            },
    //            "]" => {
    //                match stack.pop() {
    //                    Some(c) => match c.as_str() {
    //                        "[" => (),//println!("FOUND MATCH"),
    //                        _ => {
    //                            println!("ERROR");
    //                            error_score = error_score + error_val("]");
    //                        },
    //                    },
    //                    None => continue,
    //                }
    //            },
    //            "}" => {
    //                match stack.pop() {
    //                    Some(c) => match c.as_str() {
    //                        "{" => (),//println!("FOUND MATCH"),
    //                        _ => {
    //                            println!("ERROR");
    //                            error_score = error_score + error_val("}");
    //                        },
    //                    },
    //                    None => continue,
    //                }
    //            },
    //            ">" => {
    //                match stack.pop() {
    //                    Some(c) => match c.as_str() {
    //                        "<" => (),//println!("FOUND MATCH"),
    //                        _ => {
    //                            println!("ERROR");
    //                            error_score = error_score + error_val(">");
    //                        },
    //                    },
    //                    None => continue,
    //                }
    //            },
    //            _ => panic!("UNKNOWN CHAR"),
    //        }
    //        print!("{}", lines[i][j]);
    //    }
    //    println!("");
    //}

    //println!("ERROR SCORE: {}", error_score);

    //let mut sum = 0;

    //for i in 0..map.len() {
    //    for j in 0..map[i].len() {
    //        if i == 0 || map[i][j] < map[i - 1][j] {
    //            if j == 0 || map[i][j] < map[i][j - 1] {
    //                if i == (map.len() - 1) || map[i][j] < map[i + 1][j] {
    //                    if j == (map[i].len() - 1) || map[i][j] < map[i][j + 1] {
    //                        println!("FOUND HOLE");
    //                        sum = sum + 1 + map[i][j];
    //                    }
    //                }
    //            }
    //        }
    //    }
    //}

    //println!("SUM: {}", sum);


    Ok(())
}
