use std::fs;

use eyre::Result;

#[derive(PartialEq, Eq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Basin {
    num: Option<usize>,
}


fn step_point(map: &Vec<Vec<usize>>, pos: Point) -> Point {
    if pos.x != 0 {
        if map[pos.x - 1 ][pos.y] < map[pos.x][pos.y] {
            return Point{x: pos.x - 1, y: pos.y};
        }
    }
    if pos.x != (map.len() - 1) {
        if map[pos.x + 1][pos.y] < map[pos.x][pos.y] {
            return Point{x: pos.x + 1, y: pos.y};
        }
    }
    if pos.y != 0 {
         if map[pos.x][pos.y - 1] < map[pos.x][pos.y] {
            return Point{x: pos.x, y: pos.y - 1};
        }
    }
    if pos.y != (map.len() - 1) {
        if map[pos.x][pos.y + 1] < map[pos.x][pos.y] {
            return Point{x: pos.x, y: pos.y + 1};
        }
    }
    panic!("Could not step point");
}


fn find_low_point(map: &Vec<Vec<usize>>, basin_map: &Vec<Vec<Option<Basin>>>, low_points: &Vec<Point>, pos: Point) -> Basin {
    if map[pos.x][pos.y] == 9 {
        Basin {
            num: None,
        }
    } else {
        let mut current_point = pos;
        loop {
            if let Some(b) = basin_map[current_point.x][current_point.y] {
                return b;
            }
            if low_points.contains(&current_point) {
                return Basin {
                    num: Some(low_points.iter().position(|&r| r == current_point).unwrap().clone())
                };
            }
            current_point = step_point(&map, current_point);

        }
    }
}

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

    let mut low_points: Vec<Point> = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if i == 0 || map[i][j] < map[i - 1][j] {
                if j == 0 || map[i][j] < map[i][j - 1] {
                    if i == (map.len() - 1) || map[i][j] < map[i + 1][j] {
                        if j == (map[i].len() - 1) || map[i][j] < map[i][j + 1] {
                            println!("FOUND LOW POINT");
                            low_points.push(Point{x: i, y: j});
                        }
                    }
                }
            }
        }
    }

    let mut basin_map: Vec<Vec<Option<Basin>>> = Vec::new();

    for i in 0..map.len() {
        basin_map.push(Vec::new());
        for j in 0..map[i].len() {
            basin_map[i].push(None);
        }
    }



    for i in 0..map.len() {
        for j in 0..map[i].len() {
            basin_map[i][j] = Some(find_low_point(&map, &basin_map, &low_points, Point{x: i, y: j}));
        }
    }

    let mut basin_sizes: Vec<usize> = Vec::new();
    for i in 0..low_points.len() {
        basin_sizes.push(0);
    }

    for i in 0..basin_map.len() {
        for j in 0..basin_map[i].len() {
            if let Some(b) = basin_map[i][j] {
                if let Some(n) = b.num {
                    basin_sizes[n] = basin_sizes[n] + 1;
                }
            }
        }
    }

    basin_sizes.sort();
    let sum = basin_sizes[basin_sizes.len() - 1] * basin_sizes[basin_sizes.len() - 2] * basin_sizes[basin_sizes.len() - 3];
    println!("SUM: {}", sum);


    Ok(())
}
