use std::collections::HashMap;
use std::fs;

use eyre::Result;

struct PairInsertion {
    input_a: String,
    input_b: String,
    output: String,
}

fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut dots_t: Vec<(usize, usize)> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;

    let mut template: Vec<String> = lines_s[0].chars().map(|x| x.to_string()).collect();


    let filename2 = "input-ins";

    let contents = fs::read_to_string(filename2).expect("something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    //let mut insertions: Vec<PairInsertion> = Vec::new();
    let mut insertions: HashMap<(String, String), String> = HashMap::new();

    for i in 0..lines_s.len() {
        let t: Vec<String> = lines_s[i].split(" -> ").map(|x| x.to_string()).collect();
        let input: Vec<String> = t[0].chars().map(|x| x.to_string()).collect();

        insertions.insert((input[0].clone(), input[1].clone()), t[1].clone());
    }

    for i in 0..template.len() {
        print!("{}", template[i]);
    }
    println!("\n");
    for (k, v) in insertions.iter() {
        //println!("{}{} -> {}", insertions[i].input_a, insertions[i].input_b, insertions[i].output);
        println!("{}{} -> {}", k.0, k.1, v);
    }

    for _ in 0..10 {
        let mut new_template: Vec<String> = Vec::new();
        new_template.push(template[0].clone());
        for i in 0..template.len() - 1 {
            println!("Checking if {}{}", template[i], template[i + 1]);
            match insertions.get(&(template[i].clone(), template[i + 1].clone())) {
                Some(o) => new_template.push(o.clone()),
                None => ()
            }
            new_template.push(template[i + 1].clone());
        }
        for i in 0..new_template.len() {
            print!("{}", new_template[i]);
        }
        println!(" ({})", new_template.len());
        template = new_template;
    }

    let mut counters: HashMap<String, usize> = HashMap::new();

    for i in 0..template.len() {
        if let Some(x) = counters.get_mut(&template[i]) {
            *x = *x + 1
        } else {
            counters.insert(template[i].clone(), 0);
        };
    }

    let mut max = 0;
    let mut min = std::usize::MAX;

    for (_, v) in counters.iter() {
        if *v > max {
            max = *v;
        }
        if *v < min {
            min = *v;
        }
    }

    println!("OUTPUT: {}", max - min);

    //    for i in 0..new_template.len() {
    //        print!("{}", new_template[i]);
    //    }
    //    panic!("TEST");
    //}


    Ok(())
}
