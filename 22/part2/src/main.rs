use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::cmp;

use eyre::Result;


#[derive(Copy, Clone)]
struct Volume {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

struct Grid {
    volumes: Vec<Volume>,
}

fn are_intersecting(a: Volume, b: Volume) -> bool {
    if (b.z.1 <= a.z.0) || (b.z.0 >= a.z.1) {
        return false;
    }
    if (b.y.1 <= a.y.0) || (b.y.0 >= a.y.1) {
        return false;
    }
    if (b.x.1 <= a.x.0) || (b.x.0 >= a.x.1) {
        return false;
    }
    return true;
    //if (b.z.0 > a.z.0 && b.z.0 < a.z.1) || (b.z.1 > a.z.0 && b.z.1 < a.z.1) {
    //    if (b.y.0 > a.y.0 && b.y.0 < a.y.1) || (b.y.1 > a.y.0 && b.y.1 < a.y.1) {
    //        if (b.x.0 >= a.x.0 && b.x.0 < a.x.1) || (b.x.1 > a.x.0 && b.x.1 <= a.x.1) {
    //            println!("INTERESCTING: HARD EDGE X");
    //            return true;
    //        }
    //    }
    //};
    //if (b.z.0 >= a.z.0 && b.z.0 < a.z.1) || (b.z.1 > a.z.0 && b.z.1 <= a.z.1) {
    //    if (b.y.0 > a.y.0 && b.y.0 < a.y.1) || (b.y.1 > a.y.0 && b.y.1 < a.y.1) {
    //        if (b.x.0 > a.x.0 && b.x.0 < a.x.1) || (b.x.1 > a.x.0 && b.x.1 < a.x.1) {
    //            println!("INTERESCTING: HARD EDGE Z");
    //            return true;
    //        }
    //    }
    //};
    //if (b.z.0 > a.z.0 && b.z.0 < a.z.1) || (b.z.1 > a.z.0 && b.z.1 < a.z.1) {
    //    if (b.y.0 >= a.y.0 && b.y.0 < a.y.1) || (b.y.1 > a.y.0 && b.y.1 <= a.y.1) {
    //        if (b.x.0 > a.x.0 && b.x.0 < a.x.1) || (b.x.1 > a.x.0 && b.x.1 < a.x.1) {
    //            println!("INTERESCTING: HARD EDGE Y");
    //            return true;
    //        }
    //    }
    //};
    //return false;
}

impl Grid {
    fn new() -> Grid {
        Grid {
            volumes: Vec::new(),
        }
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for i in 0..self.volumes.len() {
            let x_dim = (self.volumes[i].x.1 - self.volumes[i].x.0);
            let y_dim = (self.volumes[i].y.1 - self.volumes[i].y.0);
            let z_dim = (self.volumes[i].z.1 - self.volumes[i].z.0);
            count += x_dim * y_dim * z_dim;
        }
        count as usize
    }

    fn apply_move(&mut self, m: Move) {
        self.print();
        println!("MOVE: {}", m.to_string());

        let mut new_volumes: Vec<Volume> = Vec::new();
        // if volume is additive, add both the old split volume and the new volume
        // else, just add the split parts of the old volume
        let b = m.volume;
        match m.val {
            true => new_volumes.push(b),
            false => (),
        };


        for i in 0..self.volumes.len() {
            let a = self.volumes[i];

            if !are_intersecting(a, b) {
                new_volumes.push(a);
                continue;
            }

            if (b.z.0 > a.z.0 && b.z.0 < a.z.1) {
                new_volumes.push(Volume {
                    x: a.x,
                    y: a.y,
                    z: (a.z.0, b.z.0),
                });
            }
            if (b.z.1 > a.z.0 && b.z.1 < a.z.1) {
                new_volumes.push(Volume {
                    x: a.x,
                    y: a.y,
                    z: (b.z.1, a.z.1),
                });
            }
            if (b.y.0 > a.y.0 && b.y.0 < a.y.1) {
                new_volumes.push(Volume {
                    x: a.x,
                    y: (a.y.0, b.y.0),
                    z: (cmp::max(a.z.0, b.z.0), cmp::min(a.z.1, b.z.1)),
                });
            }
            if (b.y.1 > a.y.0 && b.y.1 < a.y.1) {
                new_volumes.push(Volume {
                    x: a.x,
                    y: (b.y.1, a.y.1),
                    z: (cmp::max(a.z.0, b.z.0), cmp::min(a.z.1, b.z.1)),
                });
            }
            if (b.x.0 > a.x.0 && b.x.0 < a.x.1) {
                new_volumes.push(Volume {
                    x: (a.x.0, b.x.0),
                    y: (cmp::max(a.y.0, b.y.0), cmp::min(a.y.1, b.y.1)),
                    z: (cmp::max(a.z.0, b.z.0), cmp::min(a.z.1, b.z.1)),
                });
            }
            if (b.x.1 > a.x.0 && b.x.1 < a.x.1) {
                new_volumes.push(Volume {
                    x: (b.x.1, a.x.1),
                    y: (cmp::max(a.y.0, b.y.0), cmp::min(a.y.1, b.y.1)),
                    z: (cmp::max(a.z.0, b.z.0), cmp::min(a.z.1, b.z.1)),
                });
            }
        }
        self.volumes = new_volumes;
        self.print();
        println!("\n\n");
    }

    fn print(&self) {
        //println!("COUNT: {}", self.count());
        for i in 0..self.volumes.len() {
            let v = self.volumes[i];
            println!("Volume: x={}..{},y={}..{},z={}..{}", v.x.0, v.x.1, v.y.0, v.y.1, v.z.0, v.z.1);
            //for x in v.x.0..v.x.1 + 1 {
            //    for y in v.y.0..v.y.1 + 1 {
            //        for z in v.z.0..v.z.1 + 1 {
            //            println!("{},{},{}", x, y, z);
            //        }
            //    }
            //}
        }

    }

}

#[derive(Copy, Clone)]
struct Move {
    val: bool,
    volume: Volume,
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
        let mut ranges: Vec<(isize, isize)> = Vec::new();
        for i in 0..3 {
            let split: Vec<isize> = split[i].split("=").collect::<Vec<&str>>()[1].split("..").map(|x| x.parse::<isize>().unwrap()).collect();
            // ADD PLUS 1 back?
            ranges.push((split[0], split[1] + 1));
        }
        Some(Move {
            val,
            volume: Volume {
                x: ranges[0],
                y: ranges[1],
                z: ranges[2],
            }
        })
    }

    fn to_string(&self) -> String {
        let mut string = "".to_string();
        string.push_str(match self.val {
            true => "on ",
            false => "off ",
        });
        string.push_str("x=");
        string.push_str(&self.volume.x.0.to_string());
        string.push_str("..");
        string.push_str(&self.volume.x.1.to_string());
        string.push_str(",");
        string.push_str("y=");
        string.push_str(&self.volume.y.0.to_string());
        string.push_str("..");
        string.push_str(&self.volume.y.1.to_string());
        string.push_str(",");
        string.push_str("z=");
        string.push_str(&self.volume.z.0.to_string());
        string.push_str("..");
        string.push_str(&self.volume.z.1.to_string());
        string.push_str(",");
        string
    }
}

fn main() -> Result<()> {
    let filename1 = "input-test";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();


    //let mut cubes = Grid::new(-50..51);
    let mut grid = Grid::new();



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
        grid.apply_move(moves[i]);
    }

    grid.print();
    let count = grid.count();

    println!("COUNT: {}", count);



    Ok(())
}
