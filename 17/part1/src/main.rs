use std::collections::HashMap;
use std::fs;

use eyre::Result;


fn main() -> Result<()> {
    //let filename1 = "input";

    //let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    //let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    //lines_s.pop();

    //let x_bounds: (i64, i64) = (20, 30); //these are positive
    //let y_bounds: (i64, i64) = (-5, -10); // these are actually negative
    let x_bounds: (i64, i64) = (287, 309); //these are positive
    let y_bounds: (i64, i64) = (-48, -76); // these are actually negative


    let y_distance = -5;
    let x_distance = 20;

    let max_x_val = x_bounds.1 + 1;
    let mut min_x_val = 0;
    'outer: for i in 0..max_x_val {
        let mut pos = 0;
        for step in 1..i + 1 {
            pos += step;
            if pos >= x_bounds.0 {
                min_x_val = i;
                break 'outer;
            }
        }
    }



    println!("X Vals: {}->{}", min_x_val, max_x_val);

    let mut valid_points: Vec<(i64, i64, i64)> = vec![];

    'outer: for x in min_x_val..max_x_val {
        let mut valid_steps = vec![];
        let mut pos = 0;
        let mut vel = x;
        let mut steps = 0;
        let mut start_valid = false;
        loop {
            if pos >= x_bounds.0 && pos <= x_bounds.1 {
                println!("X value of {} valid at {} steps", x, steps);
                valid_steps.push(steps);
                start_valid = true;
            } else {
                if start_valid {
                    break;
                }
            }
            pos += vel;
            if vel > 0 {
                vel -= 1;
            }
            if steps == x {
                break;
            }
            steps += 1;
        }

        for step in valid_steps {
            if step != x {
                // intermediary step
                let max_y = (step / 2) + 1;
                for y in 0..max_y {
                    let mut pos = 0;
                    let mut max_height = std::i64::MIN;
                    let mut vel = y;
                    for i in (0..step).rev() {
                        if pos > max_height {
                            max_height = pos;
                        }
                        pos += vel;
                        vel -= 1;
                    }
                    if pos >= y_bounds.1 && pos <= x_bounds.0 {
                        println!("Valid Trajectory: {}, {} -> {}", x, y, max_height);
                        valid_points.push((x, y, max_height));
                    }
                }

            } else {
                for y in 0..y_bounds.1.abs() {
                    let mut pos = 0;
                    let mut max_height = std::i64::MIN;
                    let mut vel = y;
                    loop {
                        pos += vel;
                        if pos > max_height {
                            max_height = pos;
                        }
                        if pos >= y_bounds.1 && pos <= x_bounds.0 {
                            println!("Valid Trajectory: {}, {} -> {}", x, y, max_height);
                            valid_points.push((x, y, max_height));
                        }
                        if pos < y_bounds.0 {
                            break;
                        }
                        vel -= 1;
                    }
                }
                // infinite step
            }
        }
    }


    let mut total_max_height = std::i64::MIN;
    for (x, y, max_height) in valid_points {
        if max_height > total_max_height {
            total_max_height = max_height;
        }
    }

    println!("MAX HEIGHT: {}", total_max_height);

    //let points: Vec<usize, usize, usize> = vec![];

    //// starting x value must be within range when resolved out to zero

    //let mut x_velocity_bounds = (0, 0);
    //let mut x = 1;
    //let mut lower_bound_found = false;
    //loop {
    //    print!("X: {} ", x);
    //    let mut pos = 0;
    //    for step in 1..x + 1 {
    //        pos += step;
    //    }
    //    print!("pos: {} ", pos);

    //    let success = (pos >= x_bounds.0) && (pos <= x_bounds.1);
    //    print!("Success: {} ", success);
    //    println!("");

    //    if lower_bound_found && !success {
    //        x_velocity_bounds.1 = x;
    //        break;
    //    } else if !lower_bound_found && success {
    //        x_velocity_bounds.0 = x;
    //        lower_bound_found = true;
    //    }
    //    x = x + 1;
    //}

    //println!("Valid X values: {}->{}", x_velocity_bounds.0, x_velocity_bounds.1);

    //let max_y_val = 0;

    //for x_val in x_velocity_bounds.0..x_velocity_bounds.1 {
    //    for y in 0..std::usize::MAX {
    //        // calculate max height for y val
    //        let mut max_height = 0;
    //        for i in 1..y + 1 {
    //            max_height += i;
    //        }
    //        let min_distance_to_range = max_height + y_bounds.0;
    //        let max_distance_to_range = max_height + y_bounds.1;

    //        for i in min_distance_to_range..max_distance_to_range + 1 {
    //            // if i is a triangle number, add to
    //        }

    //        // determine if y val can make it to the range
    //    }
    //}




    //let vals: Vec<String> = lines_s[0].chars().map(|x| x.to_string()).collect();



    Ok(())
}
