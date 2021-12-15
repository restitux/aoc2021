use std::collections::HashMap;
use std::fs;

use eyre::Result;

struct PairInsertion {
    input_a: String,
    input_b: String,
    output: String,
}

fn insert_into_template(
    template: &mut HashMap<(String, String), usize>,
    k: (String, String),
    v: usize,
) {
    if let Some(n_v) = template.get_mut(&k) {
        //println!("Already seen {}{}", k.0, k.1);
        *n_v = *n_v + v;
    } else {
        //println!("New {}{}", k.0, k.1);
        template.insert(k, v);
    };
}

fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut template_t: Vec<String> = lines_s[0].chars().map(|x| x.to_string()).collect();
    let begin = template_t[0].clone();
    let end = template_t[1].clone();
    let mut template: HashMap<(String, String), usize> = HashMap::new();

    for i in 0..template_t.len() - 1 {
        if let Some(x) = template.get_mut(&(template_t[i].clone(), template_t[i + 1].clone())) {
            *x = *x + 1;
        } else {
            template.insert((template_t[i].clone(), template_t[i + 1].clone()), 1);
        };
    }

    for (k, v) in template.iter() {
        println!("{}{}: {}", k.0, k.1, v);
    }

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

    println!("\n");
    for (k, v) in insertions.iter() {
        println!("{}{} -> {}", k.0, k.1, v);
    }

    for _ in 0..40 {
        let mut new_template: HashMap<(String, String), usize> = HashMap::new();
        for (k, v) in template.iter() {
            print!("{}{}: {}, ", k.0, k.1, v);
            match insertions.get(&k) {
                Some(o) => {
                    println!("Creating {}{}, and {}{}", k.0, o, o, k.1);
                    insert_into_template(&mut new_template, (k.0.clone(), o.clone()), *v);
                    insert_into_template(&mut new_template, (o.clone(), k.1.clone()), *v);
                }
                None => insert_into_template(&mut new_template, k.clone(), *v),
            };
        }
        println!("");
        template = new_template;
    }

    //for _ in 0..10 {
    //    let mut new_template: Vec<String> = Vec::new();
    //    new_template.push(template[0].clone());
    //    for i in 0..template.len() - 1 {
    //        match insertions.get(&(template[i].clone(), template[i + 1].clone())) {
    //            Some(o) => new_template.push(o.clone()),
    //            None => ()
    //        }
    //        new_template.push(template[i + 1].clone());
    //    }
    //    println!(" ({})", new_template.len());
    //    template = new_template;
    //}

    let mut counters: HashMap<String, usize> = HashMap::new();

    for (k, v) in template.iter() {
        if let Some(x) = counters.get_mut(&k.0) {
            *x = *x + v;
        } else {
            counters.insert(k.0.clone(), v.clone());
        }
        if let Some(x) = counters.get_mut(&k.1) {
            *x = *x + v;
        } else {
            counters.insert(k.1.clone(), v.clone());
        }
    }

    if let Some(x) = counters.get_mut(&begin) {
        *x = *x + 1;
    }
    if let Some(x) = counters.get_mut(&end) {
        *x = *x + 1;
    }

    let mut max = 0;
    let mut min = std::usize::MAX;

    for (k, v) in counters.iter_mut() {
        print!("{}: {}, ", k, v);
        if *v % 2 == 0 {
            *v = *v / 2;
        } else {
            *v = *v / 2;
            *v = *v + 1;
        }
        if *v > max {
            max = *v;
        }
        if *v < min {
            min = *v;
        }
    }
    println!("");

    println!("OUTPUT: {}", max - min);

    //for i in 0..template.len() {
    //    if let Some(x) = counters.get_mut(&template[i]) {
    //        *x = *x + 1
    //    } else {
    //        counters.insert(template[i].clone(), 0);
    //    };
    //}

    ////    for i in 0..new_template.len() {
    ////        print!("{}", new_template[i]);
    ////    }
    ////    panic!("TEST");
    ////}

    Ok(())
}
