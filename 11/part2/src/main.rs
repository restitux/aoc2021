use std::fs;

use eyre::Result;


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

    println!("DIM: {}, {}", ops.len(), ops[0].len());
    let num_octopus = ops.len() * ops[0].len();

    let mut total_flashes = 0;

    let mut t = 0;

    loop {
        t = t + 1;
        println!("t: {}", t);
        let mut iteration_flashes = 0;
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
                        //println!("({}, {}) flashed", i, j);
                        iteration_flashes = iteration_flashes + 1;
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
        if num_octopus == iteration_flashes {
            panic!("ALL FLASHES AT {}", t);
        }
        total_flashes = total_flashes + iteration_flashes;
    }

    Ok(())
}
