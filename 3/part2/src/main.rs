use std::fs;

use eyre::Result;

fn get_bit(val: &u64, bit: u64) -> u64 {
    //println!("Getting bit {} from number {}", bit, val);
    let mask = 0x1 << bit;
    let val_masked = val & mask;
    return val_masked >> bit;
}

fn get_oxygen_bit(nums: &Vec<u64>, bit: u64) -> u64 {
    let mut num_ones = 0;
    for i in 0..nums.len() {
        num_ones = num_ones + get_bit(&nums[i], bit);
    }
    let threshold = nums.len() as f32 / 2.0;
    return (num_ones as f32 >= threshold) as u64
}

fn get_co2_bit(nums: &Vec<u64>, bit: u64) -> u64 {
    let mut num_ones = 0;
    for i in 0..nums.len() {
        num_ones = num_ones + get_bit(&nums[i], bit);
    }
    let threshold = nums.len() as f32 / 2.0;
    return !(num_ones as f32 >= threshold) as u64
}


fn main() -> Result<()> {
    let filename = "input";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut ms: Vec<&str> = contents.split("\n").collect();
    ms.pop();
    let mi: Vec<u64> = ms
        .iter()
        .map(|x| u64::from_str_radix(x, 2).unwrap())
        .collect();

    let val_width = 12;
    let mut num_vals = Vec::<u64>::new();
    for _ in 0..val_width {
        num_vals.push(0);
    }

    // oxygen
    let mut oxygen = 0;
    let mut input_vals : Vec<u64> = Vec::new();
    for i in 0..mi.len() {
        input_vals.push(mi[i]);
    }

    for bit in 0..val_width {
        let mut new_input_vals: Vec<u64> = Vec::new();

        if input_vals.len() == 1 {
            break;
        }

        let bit_width = val_width - 1 - bit;
        let oxygen_bit = get_oxygen_bit(&input_vals, bit_width);

        for i in 0..input_vals.len() {
            if get_bit(&input_vals[i], bit_width) == oxygen_bit {
                new_input_vals.push(input_vals[i]);
            }
        }
        input_vals = new_input_vals;
    }
    oxygen = input_vals[0];

    // co2
    let mut co2 = 0;
    let mut input_vals : Vec<u64> = Vec::new();
    for i in 0..mi.len() {
        input_vals.push(mi[i]);
    }

    for bit in 0..val_width {
        let mut new_input_vals: Vec<u64> = Vec::new();

        for i in 0..input_vals.len() {
            print!("{}, ", input_vals[i]);
        }
        println!("");
        if input_vals.len() == 1 {
            break;
        }

        let bit_width = val_width - 1 - bit;
        let co2_bit = get_co2_bit(&input_vals, bit_width);
        println!("co2 bit: {}", co2_bit);

        for i in 0..input_vals.len() {
            if get_bit(&input_vals[i], bit_width) == co2_bit {
                new_input_vals.push(input_vals[i]);
            }
        }
        input_vals = new_input_vals;
    }
    co2 = input_vals[0];
    // oxygen
    //let oxygen = calculate_oxygen_val(mi.clone(), val_width);
    //// co2
    //let co2 = calculate_co2_val(mi.clone(), val_width);
    println!("OXYGEN: {}", oxygen);
    println!("CO2: {}", co2);

    println!("OUTPUT: {}", oxygen * co2);

    Ok(())
}
