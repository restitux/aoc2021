use std::collections::HashMap;
use std::fs;

use eyre::Result;

enum Axis {
    X,
    Y,
}

struct Map {
    map: Vec<Vec<bool>>,
    x: usize,
    y: usize
}

fn new_map(x: usize, y: usize) -> Map {
    let mut map = Vec::new();

    for _ in 0..y + 1 {
        map.push([false].repeat(x + 1));
    }

    Map {
        map,
        x,
        y,
    }
}

struct Instruction {
    axis: Axis,
    val: usize,
}


fn do_fold(map: Map, fold: &Instruction) -> Map {
    match fold.axis {
        Axis::X => {
            let mut new_map = new_map(map.x / 2, map.y);

            for y in 0..map.y {
                for x in 0..map.x {
                    if x < map.x / 2 {
                        if !new_map.map[y][x] {
                            new_map.map[y][x] = map.map[y][x];
                        }
                    } else {
                        if !new_map.map[y][map.x - x - 1] {
                            new_map.map[y][map.x - x - 1] = map.map[y][x];
                        }
                    }

                }
            }


            new_map
        },
        Axis::Y => {
            let mut new_map = new_map(map.x, map.y / 2);

            for y in 0..map.y {
                for x in 0..map.x {
                    if y < map.y / 2 {
                        if !new_map.map[y][x] {
                            new_map.map[y][x] = map.map[y][x];
                        }
                    } else {
                        if !new_map.map[map.y - y - 1][x] {
                            new_map.map[map.y - y - 1][x] = map.map[y][x];
                        }
                    }

                }
            }

            new_map
        },
    }
}

fn count_dots(map: &Map) -> usize {
    let mut count = 0;
    for x in 0..map.x {
        for y in 0..map.y {
            if map.map[y][x] {
                count = count + 1;
            }
        }
    }
    count
}

fn print_dim(map: &Map) {
    println!("DIM: {},{}", map.x, map.y);
}

fn print_map(map: &Map) {
    print!("\n\n");
    println!("Printing map...");
    for y in 0..map.y {
        for x in 0..map.x {
            print!("{}", if map.map[y][x] {
                "#"
            } else {
                "."
            });
        }
        println!("");
    }
}


fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut dots_t: Vec<(usize, usize)> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for i in 0..lines_s.len() {
        let p: Vec<usize> = lines_s[i].split(",").map(|x| x.to_string().parse::<usize>().unwrap()).collect();

        if p[0] > max_x {max_x = p[0]};
        if p[1] > max_y {max_y = p[1]};

        dots_t.push((p[0], p[1]));
    }

    let filename2 = "input-ins";

    let contents = fs::read_to_string(filename2).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut instructions: Vec<Instruction> = Vec::new();

    for i in 0..lines_s.len() {
        let t: Vec<String> = lines_s[i].split(" ").map(|x| x.to_string()).collect();
        let p: Vec<String> = t[2].split("=").map(|x| x.to_string()).collect();

        instructions.push(Instruction {
            axis: match p[0].as_str() {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!("INVALID AXIS"),
            },
            val: p[1].parse::<usize>().unwrap(),
        });
    }

    max_x = max_x + 1;
    max_y = max_y + 1;

    let mut map = new_map(max_x, max_y);

    for i in 0..dots_t.len() {
        println!("DOT: {}, {}", dots_t[i].0, dots_t[i].1);
        map.map[dots_t[i].1][dots_t[i].0] = true;
    }

    println!("\n\n");
    for i in 0..instructions.len() + 1 {
        print_map(&map);
        let count = count_dots(&map);
        println!("COUNT: {}", count);
        if i == instructions.len() {break};
        map = do_fold(map, &instructions[i]);
    }
    print_map(&map);

    Ok(())
}
