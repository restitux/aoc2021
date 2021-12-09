use std::fs;

use eyre::Result;


fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut map: Vec<Vec<usize>> = Vec::new();

    for i in 0..lines_s.len() {
        let row: Vec<usize> = lines_s[i].chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect();
        map.push(row);
    }

    let mut sum = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if i == 0 || map[i][j] < map[i - 1][j] {
                if j == 0 || map[i][j] < map[i][j - 1] {
                    if i == (map.len() - 1) || map[i][j] < map[i + 1][j] {
                        if j == (map[i].len() - 1) || map[i][j] < map[i][j + 1] {
                            println!("FOUND HOLE");
                            sum = sum + 1 + map[i][j];
                        }
                    }
                }
            }
        }
    }

    println!("SUM: {}", sum);


    Ok(())
}
