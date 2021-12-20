use std::collections::HashMap;
use std::fs;

use eyre::Result;

use bit_vec::BitVec;

fn main() -> Result<()> {
    let filename1 = "input-alg";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut alg_lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    alg_lines_s.pop();

    let algorithm: Vec<bool> = alg_lines_s[0]
        .chars()
        .map(|x| match x {
            '#' => true,
            '.' => false,
            _ => panic!("Unkown pixel"),
        })
        .collect();

    let filename1 = "input-img";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut image_lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    image_lines_s.pop();

    let mut image_base: Vec<Vec<bool>> = Vec::new();

    for line in 0..image_lines_s.len() {
        let image_line = image_lines_s[line]
            .chars()
            .map(|x| match x {
                '#' => true,
                '.' => false,
                _ => panic!("Unknown image pixel"),
            })
            .collect();
        image_base.push(image_line);
    }

    //for i in 0..image_base.len() {
    //    for j in 0..image_base[i].len() {
    //        match image_base[i][j] {
    //            true => print!("#"),
    //            false => print!("."),
    //        }
    //    }
    //    println!("");
    //}

    let mut image: Vec<Vec<bool>> = Vec::new();

    let padding = 1000;
    let width = image_base[0].len();

    for _ in 0..padding {
        let line = vec![false; width + (padding * 2)];
        image.push(line);
    }

    for i in 0..image_base.len() {
        let mut line = vec![false; padding];
        for j in 0..image_base[i].len() {
            line.push(image_base[i][j]);
        }
        for j in 0..padding {
            line.push(false);
        }
        image.push(line);
    }

    for _ in 0..padding {
        let line = vec![false; width + (padding * 2)];
        image.push(line);
    }

    //for i in 0..image.len() {
    //    for j in 0..image[i].len() {
    //        match image[i][j] {
    //            true => print!("#"),
    //            false => print!("."),
    //        }
    //    }
    //    println!("");
    //}
    //println!("\n\n\n");

    for _ in 0..50 {
        let mut new_image: Vec<Vec<bool>> = Vec::new();
        for _ in 0..image.len() {
            new_image.push(vec![false; width + (padding * 2)]);
        }

        for i in 1..new_image.len() - 1 {
            for j in 1..new_image[i].len() - 1 {
                let neighbors = [
                    (i - 1, j - 1),
                    (i - 1, j),
                    (i - 1, j + 1),
                    (i, j - 1),
                    (i, j),
                    (i, j + 1),
                    (i + 1, j - 1),
                    (i + 1, j),
                    (i + 1, j + 1),
                ];
                //let nejghbors = [(j - 1, i - 1), (j, i - 1), (j - 1, i + 1), (j, i - 1), (j, i), (j, i + 1), (j + 1, i - 1), (j + 1, i), (j + 1, i + 1)];

                let mut b_string = "".to_string();
                for n in neighbors {
                    match image[n.0][n.1] {
                        true => b_string.push('1'),
                        false => b_string.push('0'),
                        _ => panic!(""),
                    }
                }

                let number = usize::from_str_radix(&b_string, 2).unwrap();
                //println!("Setting pixel {}, {} to agl index {}", i, j, number);

                new_image[i][j] = algorithm[number];
            }
        }

        image = new_image;

        println!("IMAGELEN: {}", image[0].len());

        //for i in 0..image.len() {
        //    for j in 0..image[i].len() {
        //        match image[i][j] {
        //            true => print!("#"),
        //            false => print!("."),
        //        }
        //    }
        //    println!("");
        //}
        //println!("");

        let mut lit_count = 0;

        for i in 150..image.len() - 150 {
            for j in 150..image[i].len() - 150 {
                match image[i][j] {
                    true => lit_count += 1,
                    false => (),
                }
            }
        }
        println!(
            "lit_count:
            {}",
            lit_count
        );
    }

    //let mut bits = BitVec::new();
    //let mut bits: String = "".to_string();

    ////let vals: Vec<usize> = lines_s[0].chars().map(|x| usize::from_str_radix(x.to_string().as_str(), 16).unwrap()).collect();
    //let vals: Vec<String> = lines_s[0].chars().map(|x| x.to_string()).collect();

    Ok(())
}
