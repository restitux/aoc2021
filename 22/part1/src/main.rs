use std::collections::HashMap;
use std::fs;
use std::ops::Range;

use eyre::Result;


struct Volume {
    x: Range<isize>,
    y: Range<isize>,
    z: Range<isize>,
}

struct Grid {
    cubes: Vec<Vec<Vec<bool>>>,
}


impl Grid {
    fn new() -> Grid {
        let mut grid = Grid {
            cubes: Vec::new()
        };
        for x in 0..101 {
            grid.cubes.push(vec![Vec::new()]);
            for y in 0..101 {
                grid.cubes[x].push(Vec::new());
                for z in 0..101 {
                    grid.cubes[x][y].push(false);
                }
            }
        }
        grid
    }
 
    fn set(&mut self, x: isize, y: isize, z: isize, val: bool) {
        if x < -50 || y < -50 || z < -50 {
            panic!("Invalid coords: ({}, {}, {})", x, y, z)
        }

        self.cubes[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = val;
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for x in 0..101 {
            for y in 0..101 {
                for z in 0..101 {
                    if self.cubes[x][y][z] {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

struct Move {
    val: bool,
    x: Range<isize>,
    y: Range<isize>,
    z: Range<isize>,
}

impl Move {
    fn from_string(string: &String) -> Option<Move> {
        let split: Vec<&str> = string.split(" ").collect();
        let val = match split[0] {
            "on" => true,
            "off" => false,
            _ => panic!("Invalid cube value"),
        };
        let split: Vec<&str> = split[1].split(",").collect();
        let mut ranges: Vec<Range<isize>> = Vec::new();
        for i in 0..3 {
            let split: Vec<isize> = split[i].split("=").collect::<Vec<&str>>()[1].split("..").map(|x| x.parse::<isize>().unwrap()).collect();
            if split[0] > 50 || split[0] < -50 || split[1] > 50 || split[1] < -50 {
                return None
            }
            ranges.push((split[0]..(split[1] + 1)));
        }
        Some(Move {
            val,
            x: ranges[0].clone(),
            y: ranges[1].clone(),
            z: ranges[2].clone(),
        })
    }

    fn to_string(&self) -> String {
        let mut string = "".to_string();
        string.push_str(match self.val {
            true => "on ",
            false => "off ",
        });
        string.push_str("x=");
        string.push_str(&self.x.start.to_string());
        string.push_str("..");
        string.push_str(&self.x.end.to_string());
        string.push_str(",");
        string.push_str("y=");
        string.push_str(&self.y.start.to_string());
        string.push_str("..");
        string.push_str(&self.y.end.to_string());
        string.push_str(",");
        string.push_str("z=");
        string.push_str(&self.z.start.to_string());
        string.push_str("..");
        string.push_str(&self.z.end.to_string());
        string.push_str(",");
        string
    }
}

fn main() -> Result<()> {
    let filename = "input-test";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();


    //let mut cubes = Grid::new(-50..51);
    let mut cubes = Grid::new();



    let mut moves: Vec<Move> = Vec::new();

    //let vals: Vec<usize> = lines_s[0].chars().map(|x| usize::from_str_radix(x.to_string().as_str(), 16).unwrap()).collect();
    //let vals: Vec<String> = lines_s[0].chars().map(|x| x.to_string()).collect();

    for i in 0..lines_s.len() {
        match Move::from_string(&lines_s[i]) {
            Some(m) => moves.push(m),
            None => (),
        };
    }

    for i in 0..moves.len() {
        for x in moves[i].x.clone() {
            for y in moves[i].y.clone() {
                for z in moves[i].z.clone() {
                    cubes.set(x, y, z, moves[i].val);
                }
            }
        }
    }

    let count = cubes.count();

    println!("COUNT: {}", count);



    Ok(())
}
