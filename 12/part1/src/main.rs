use std::collections::HashMap;
use std::fs;

use eyre::Result;

#[derive(Clone)]
enum CaveType {
    BIG,
    SMALL,
}

#[derive(Clone)]
struct Node {
    name: String,
    n_type: CaveType,
    connections: Vec<String>,
}

struct Path {
    path: Vec<String>,
    small_caves: Vec<String>,
}

fn find_paths(
    node_name: String,
    map: &HashMap<String, Node>,
    mut small_caves: Vec<String>,
) -> Vec<Path> {

    println!("find_paths({})", node_name);
    if node_name == "end".to_string() {
        println!("WE HAVE REACHED THE END");
        vec![Path {
            path: vec!["end".to_string()],
            small_caves: small_caves,
        }]
    } else {
        let entry = map.get(&node_name).clone().unwrap();
        if let CaveType::SMALL = entry.n_type {
           small_caves.push(entry.name.clone());
        }
        let mut paths: Vec<Path> = Vec::new();
        for conn in 0..entry.connections.len() {
            if !small_caves.contains(&entry.connections[conn]) {
                let new_paths = find_paths(entry.connections[conn].clone(), &map, small_caves.clone());
                for i in 0..new_paths.len() {
                    let mut v = vec![entry.name.clone()];
                    v.extend(new_paths[i].path.iter().cloned());
                    let mut a = small_caves.clone();
                    a.extend(new_paths[i].small_caves.iter().cloned());
                    paths.push(Path {
                        path: v,
                        small_caves: a,
                    });
                }
            }
        }
        paths
    }
}

fn insert_node(
    mut map: &mut HashMap<String, Node>,
    name: String,
    connection_name: String,
) {
        if map.contains_key(&name) {
            map.get_mut(&name).unwrap().connections.push(connection_name.clone());
            //map[&name].connections.push(x[1].clone());
        } else {
            let n_type = if name == "start" || name == "end" {
                CaveType::SMALL
            } else if name.chars().nth(0).unwrap().is_ascii_uppercase() {
                CaveType::BIG
            } else {
                CaveType::SMALL
            };

            map.insert(
                name.clone(),
                Node {
                    name: name.clone(),
                    n_type,
                    connections: vec![connection_name.clone()],
                },
            );
        }
}

fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    //let mut connections: Vec<Connection> = Vec::new();
    let mut map: HashMap<String, Node> = HashMap::new();

    for i in 0..lines_s.len() {
        //let x: Vec<Octopus> = lines_s[i].chars().map(|x| Octopus {energy: x.to_string().parse::<usize>().unwrap(), flashed: false}).collect();
        let x: Vec<String> = lines_s[i].split("-").map(|x| x.to_string()).collect();

        insert_node(&mut map, x[0].to_string(), x[1].to_string());
        insert_node(&mut map, x[1].to_string(), x[0].to_string());
    }

    for (_, v) in &map {
        print!("{}: [", v.name);
        for i in 0..v.connections.len() {
            print!("{}, ", v.connections[i]);
        }
        println!("]")
    }

    let paths: Vec<Path> = find_paths("start".to_string(), &map, vec![]);

    for i in 0..paths.len() {
        for j in 0..paths[i].path.len() {
            print!("{},", paths[i].path[j]);
        }
        println!("");
    }

    println!("NUM PATHS: {}", paths.len());

    Ok(())
}
