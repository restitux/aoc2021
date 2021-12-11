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

fn completion_val(c: &str) -> usize {
    match c {
        "(" => 1,
        "[" => 2,
        "{" => 3,
        "<" => 4,
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

    let mut corrupted_lines: Vec<usize> = Vec::new();

    let mut error_score = 0;
    let mut completion_scores: Vec<usize> = Vec::new();

    'line: for i in 0..lines.len() {
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
                            "(" => (), //println!("FOUND MATCH"),
                            _ => {
                                println!("ERROR");
                                error_score = error_score + error_val(")");
                                corrupted_lines.push(i);
                                continue 'line;
                            }
                        },
                        None => continue 'line,
                    }
                }
                "]" => {
                    match stack.pop() {
                        Some(c) => match c.as_str() {
                            "[" => (), //println!("FOUND MATCH"),
                            _ => {
                                println!("ERROR");
                                error_score = error_score + error_val("]");
                                corrupted_lines.push(i);
                                continue 'line;
                            }
                        },
                        None => continue 'line,
                    }
                }
                "}" => {
                    match stack.pop() {
                        Some(c) => match c.as_str() {
                            "{" => (), //println!("FOUND MATCH"),
                            _ => {
                                println!("ERROR");
                                error_score = error_score + error_val("}");
                                corrupted_lines.push(i);
                                continue 'line;
                            }
                        },
                        None => continue 'line,
                    }
                }
                ">" => {
                    match stack.pop() {
                        Some(c) => match c.as_str() {
                            "<" => (), //println!("FOUND MATCH"),
                            _ => {
                                println!("ERROR");
                                error_score = error_score + error_val(">");
                                corrupted_lines.push(i);
                                continue 'line;
                            }
                        },
                        None => continue 'line,
                    }
                }
                _ => panic!("UNKNOWN CHAR"),
            }
            print!("{}", lines[i][j]);
        }
        if stack.len() > 0 {
            print!("{}", stack.len());
            print!("INCOMPLETE");
            let mut line_score = 0;
            loop {
                match stack.pop() {
                    Some(c) => {
                        line_score = line_score * 5;
                        line_score = line_score + completion_val(c.as_str());
                    }
                    None => break,
                }
            }
            completion_scores.push(line_score);
        }

        println!("");
    }

    completion_scores.sort();

    println!("ERROR SCORE: {}", error_score);
    println!("COMPLETION SCORE: {}", completion_scores[completion_scores.len() / 2]);

    Ok(())
}
