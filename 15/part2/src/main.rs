use std::collections::HashMap;
use std::fs;

use eyre::Result;

use priority_queue::PriorityQueue;

#[derive(PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

struct Map {
    map: Vec<Vec<isize>>,
    x: isize,
    y: isize,
}


fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut input_map: Map = Map {
        map: Vec::new(),
        x: 0,
        y: 0,
    };

    for i in 0..lines_s.len() {
        let t = lines_s[i]
            .chars()
            .map(|x| x.to_string().parse::<isize>().unwrap())
            .collect();
        input_map.map.push(t);
    }

    input_map.x = input_map.map.len() as isize;
    input_map.y = input_map.map[0].len() as isize;

    let mut map: Map = Map {
        map: Vec::new(),
        x: input_map.x * 5,
        y: input_map.x * 5,
    };

    for y in 0..map.y {
        map.map.push(Vec::new());
        for _ in 0..map.x {
            map.map[y as usize].push(0);
        }
    }

    println!("len: {}, {}", map.map.len(), map.map[0].len());

    for y in 0..input_map.y {
        for x in 0..input_map.x {
            for big_y in 0..5 {
                for big_x in 0..5 {
                    //println!("Setting {}, {}", y * big_y, x * big_x);
                    map.map[(y + (input_map.y * big_y)) as usize][(x + (input_map.x * big_x)) as usize] = (input_map.map[y as usize][x as usize] + big_y + big_x - 1) % 9 + 1;
                }
            }
        }
    }


    for y in 0..map.y {
        for x in 0..map.x {
            print!("{}", map.map[y as usize][x as usize]);
        }
        println!("");
    }

    let mut distance_map: Vec<Vec<isize>> = Vec::new();
    for y in 0..map.y {
        distance_map.push(vec![]);
        for _ in 0..map.x {
            distance_map[y as usize].push(std::isize::MAX);
        }
    }
    let mut previous: HashMap<Point, Option<Point>> = HashMap::new();
    let mut pq = PriorityQueue::<Point, isize>::new();

    for y in 0..map.y {
        for x in 0..map.x {
            if x != 0 || y != 0 {
                previous.insert(Point{x, y}, None);
            } else {
                distance_map[y as usize][x as usize] = 0;
            }
            pq.push(Point{x, y}, std::isize::MAX - distance_map[y as usize][x as usize]);

        }
    }

    while !pq.is_empty() {
        if let Some((point, distance)) = pq.pop() {
            // Update neighbor priority
            //println!("Processing point: {}, {}", point.x, point.y);
            for neighbor in [Point{x: point.x + 1, y: point.y}, Point{x: point.x - 1, y: point.y}, Point{x: point.x, y: point.y + 1}, Point{x: point.x, y: point.y - 1}].iter() {
                if let None = pq.get(neighbor) {
                    continue;
                }
                //print!("    Processing neighbor: {}, {} ", neighbor.x, neighbor.y);
                if neighbor.x >= 0 && neighbor.x < map.x && neighbor.y >= 0 && neighbor.y < map.y {
                    //print!("VALID ");
                    let alt = distance_map[point.y as usize][point.x as usize] + map.map[neighbor.y as usize][neighbor.x as usize];
                    //print!("ALT: {} ", alt);
                    //print!("distance_map[{}][{}]: {}", neighbor.y, neighbor.x, distance_map[neighbor.y as usize][neighbor.x as usize]);
                    if alt < distance_map[neighbor.y as usize][neighbor.x as usize] {
                        //print!("Setting distance_map[{}][{}] to {}", neighbor.y, neighbor.x, alt);
                        distance_map[neighbor.y as usize][neighbor.x as usize] = alt;
                        previous.insert(neighbor.clone(), Some(point));
                        pq.change_priority(neighbor, std::isize::MAX - alt);
                    }
                }
                //println!("");
            }

        }
    }


    for y in 0..map.y {
        for x in 0..map.x {
            print!("{} ", distance_map[y as usize][x as usize]);
        }
        println!("");
    }



    Ok(())
}
