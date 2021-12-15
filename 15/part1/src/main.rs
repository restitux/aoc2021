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

    let mut map: Map = Map {
        map: Vec::new(),
        x: 0,
        y: 0,
    };

    for i in 0..lines_s.len() {
        let t = lines_s[i]
            .chars()
            .map(|x| x.to_string().parse::<isize>().unwrap())
            .collect();
        map.map.push(t);
    }

    map.x = map.map.len() as isize;
    map.y = map.map[0].len() as isize;

    //for y in 0..map.y {
    //    for x in 0..map.x {
    //        print!("{}", map.map[y][x]);
    //    }
    //    println!("");
    //}

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

    //let mut dots_t: Vec<(usize, usize)> = Vec::new();

    //let mut max_x = 0;
    //let mut max_y = 0;

    //let mut template: Vec<String> = lines_s[0].chars().map(|x| x.to_string()).collect();

    //let filename2 = "input-ins";

    //let contents = fs::read_to_string(filename2).expect("something went wrong reading the file");

    //let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    //lines_s.pop();

    ////let mut insertions: Vec<PairInsertion> = Vec::new();
    //let mut insertions: HashMap<(String, String), String> = HashMap::new();

    //for i in 0..lines_s.len() {
    //    let t: Vec<String> = lines_s[i].split(" -> ").map(|x| x.to_string()).collect();
    //    let input: Vec<String> = t[0].chars().map(|x| x.to_string()).collect();

    //    insertions.insert((input[0].clone(), input[1].clone()), t[1].clone());
    //}

    //for i in 0..template.len() {
    //    print!("{}", template[i]);
    //}
    //println!("\n");
    //for (k, v) in insertions.iter() {
    //    //println!("{}{} -> {}", insertions[i].input_a, insertions[i].input_b, insertions[i].output);
    //    println!("{}{} -> {}", k.0, k.1, v);
    //}

    //for _ in 0..10 {
    //    let mut new_template: Vec<String> = Vec::new();
    //    new_template.push(template[0].clone());
    //    for i in 0..template.len() - 1 {
    //        println!("Checking if {}{}", template[i], template[i + 1]);
    //        match insertions.get(&(template[i].clone(), template[i + 1].clone())) {
    //            Some(o) => new_template.push(o.clone()),
    //            None => ()
    //        }
    //        new_template.push(template[i + 1].clone());
    //    }
    //    for i in 0..new_template.len() {
    //        print!("{}", new_template[i]);
    //    }
    //    println!(" ({})", new_template.len());
    //    template = new_template;
    //}

    //let mut counters: HashMap<String, usize> = HashMap::new();

    //for i in 0..template.len() {
    //    if let Some(x) = counters.get_mut(&template[i]) {
    //        *x = *x + 1
    //    } else {
    //        counters.insert(template[i].clone(), 0);
    //    };
    //}

    //let mut max = 0;
    //let mut min = std::usize::MAX;

    //for (_, v) in counters.iter() {
    //    if *v > max {
    //        max = *v;
    //    }
    //    if *v < min {
    //        min = *v;
    //    }
    //}

    //println!("OUTPUT: {}", max - min);

    //    for i in 0..new_template.len() {
    //        print!("{}", new_template[i]);
    //    }
    //    panic!("TEST");
    //}

    Ok(())
}
