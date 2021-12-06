use std::fs;

use eyre::Result;


fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();


    let mut fish: Vec<u64> = lines_s[0].split(",").map(|x| x.parse::<u64>().unwrap()).collect();

    let mut freq_vec: Vec<u64> = Vec::new();

    for _ in 0..9 {
        freq_vec.push(0);
    }

    for i in 0..fish.len() {
        freq_vec[(fish[i]) as usize] = freq_vec[(fish[i]) as usize] + 1;
    }

    for day in 0..257 {
        let mut new_vec: Vec<u64> = Vec::new();

        for _ in 0..9 {
            new_vec.push(0);
        }
        for i in 0..9 {
            if i == 0 {
                new_vec[8] = freq_vec[0];
                new_vec[6] = freq_vec[0];
            } else {
                new_vec[i - 1] = new_vec[i - 1] + freq_vec[i];
            }
        }

        let mut sum = 0;
        for i in 0..8 {
            sum = sum + new_vec[i];
        }
        freq_vec = new_vec;
        println!("Day: {}, Num: {}", day + 1, sum);
    }

    Ok(())
}
