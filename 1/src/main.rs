use std::fs;

use eyre::Result;

fn main() -> Result<()> {
    let filename = "input";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut ms : Vec<&str> = contents.split("\n").collect();
    ms.pop();
    let m: Vec<i64> = ms.iter().map(|x| x.parse().unwrap()).collect();

    let mut c = 0;

    for i in 3..m.len() {
        let sum0 = m[i - 1] + m[i - 2] + m[i - 3];
        let sum1 = m[i    ] + m[i - 1] + m[i - 2];
        print!("{} vs {}", sum1, sum0);
        if sum1 > sum0 {
            c = c + 1;
            print!(" X");
        }
        println!("");
    }
    println!("Collector: {}", c);
    Ok(())
}
