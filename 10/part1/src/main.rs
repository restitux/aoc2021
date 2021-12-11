use std::fs;

use eyre::Result;

//macro_rules! match_char {
//    ()
//}

fn error_val(c: &str) -> usize {
    match c {
        ")" => 3,
        "]" => 57,
        "}" => 1197,
        ">" => 25137,
        _ => panic!("Invalid char"),
    }
}


fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut lines: Vec<Vec<String>> = Vec::new();

    for i in 0..lines_s.len() {
        let line: Vec<String> = lines_s[i].chars().map(|x| x.to_string()).collect();
        lines.push(line);
    }

    let mut error_score = 0;

    for i in 0..lines.len() {
        // for each line
        let mut stack: Vec<String> = Vec::new();
        for j in 0..lines[i].len() {
            match lines[i][j].as_str() {
                "(" => stack.push(lines[i][j].clone()),
                "[" => stack.push(lines[i][j].clone()),
                "{" => stack.push(lines[i][j].clone()),
                "<" => stack.push(lines[i][j].clone()),
                ")" => {
                    match stack.pop() {
                        Some(c) => match c.as_str() {
                            "(" => (),//println!("FOUND MATCH"),
                            _ => {
                                println!("ERROR");
                                error_score = error_score + error_val(")");
                            },
                        },
                        None => continue,
                    }
                },
                "]" => {
                    match stack.pop() {
                        Some(c) => match c.as_str() {
                            "[" => (),//println!("FOUND MATCH"),
                            _ => {
                                println!("ERROR");
                                error_score = error_score + error_val("]");
                            },
                        },
                        None => continue,
                    }
                },
                "}" => {
                    match stack.pop() {
                        Some(c) => match c.as_str() {
                            "{" => (),//println!("FOUND MATCH"),
                            _ => {
                                println!("ERROR");
                                error_score = error_score + error_val("}");
                            },
                        },
                        None => continue,
                    }
                },
                ">" => {
                    match stack.pop() {
                        Some(c) => match c.as_str() {
                            "<" => (),//println!("FOUND MATCH"),
                            _ => {
                                println!("ERROR");
                                error_score = error_score + error_val(">");
                            },
                        },
                        None => continue,
                    }
                },
                _ => panic!("UNKNOWN CHAR"),
            }
            print!("{}", lines[i][j]);
        }
        println!("");
    }

    println!("ERROR SCORE: {}", error_score);

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
