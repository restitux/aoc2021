use std::fs;

use eyre::Result;

struct Line {
    src: Point,
    dst: Point,
}

struct Point {
    x: i64,
    y: i64,
}

fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut lines: Vec<Line> = Vec::new();

    for i in 0..lines_s.len() {
        let src_dst: Vec<String> = lines_s[i].split(" -> ").map(|x| x.to_string()).collect();
        let src_s: Vec<String> = src_dst[0].split(",").map(|x| x.to_string()).collect();
        let dst_s: Vec<String> = src_dst[1].split(",").map(|x| x.to_string()).collect();
        let src = Point {
            x: src_s[0].parse().unwrap(),
            y: src_s[1].parse().unwrap(),
        };
        let dst = Point {
            x: dst_s[0].parse().unwrap(),
            y: dst_s[1].parse().unwrap(),
        };
        let line = Line { src, dst };
        lines.push(line);
    }

    let mut map: [[i64; 1000]; 1000] = [[0; 1000]; 1000];

    for i in 0..lines.len() {
        println!(
            "Line {}: {},{} -> {},{}",
            i, lines[i].src.x, lines[i].src.y, lines[i].dst.x, lines[i].dst.y
        );
        if lines[i].src.x == lines[i].dst.x {
            let x = lines[i].src.x;
            if lines[i].src.y > lines[i].dst.y {
                for i in lines[i].dst.y..(lines[i].src.y + 1) {
                    println!("Incrementing {},{}", x, i);
                    map[x as usize][i as usize] = map[x as usize][i as usize] + 1;
                }
            } else {
                for i in lines[i].src.y..(lines[i].dst.y + 1) {
                    println!("Incrementing {},{}", x, i);
                    map[x as usize][i as usize] = map[x as usize][i as usize] + 1;
                }
            }
        } else if lines[i].src.y == lines[i].dst.y {
            let y = lines[i].src.y;
            if lines[i].src.x > lines[i].dst.x {
                for i in lines[i].dst.x..(lines[i].src.x + 1) {
                    println!("Incrementing {},{}", i, y);
                    map[i as usize][y as usize] = map[i as usize][y as usize] + 1;
                }
            } else {
                for i in lines[i].src.x..(lines[i].dst.x + 1) {
                    println!("Incrementing {},{}", i, y);
                    map[i as usize][y as usize] = map[i as usize][y as usize] + 1;
                }
            }
        } else {
            let (src_x, dst_x, src_y, dst_y) = {
                if lines[i].src.x > lines[i].dst.x {
                    (lines[i].dst.x, lines[i].src.x, lines[i].dst.y, lines[i].src.y)
                } else {
                    (lines[i].src.x, lines[i].dst.x, lines[i].src.y, lines[i].dst.y)
                }
            };
            let mut i = 0;
            loop {
                if dst_y > src_y {
                    println!("Increamenting {},{}", src_x + i, src_y + i);
                    map[(src_x + i) as usize][(src_y + i) as usize] = map[(src_x + i) as usize][(src_y + i) as usize] + 1;
                } else {
                    println!("Increamenting {},{}", src_x + i, src_y - i);
                    map[(src_x + i) as usize][(src_y - i) as usize] = map[(src_x + i) as usize][(src_y - i) as usize] + 1;
                }


                if src_x + i == dst_x {
                    break;
                }
                i = i +1;
            }
        }
    }

    let mut total_over_2 = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            if map[i][j] == 0 {
                print!(".");
            } else {
                print!("{}", map[i][j]);
            }
            if map[i][j] >= 2 {
                total_over_2 = total_over_2 + 1;
            }
        }
        println!("");
    }

    println!("OVER 2: {}", total_over_2);

    Ok(())
}
