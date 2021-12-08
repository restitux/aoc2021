use std::collections::HashSet;
use std::fs;

use eyre::Result;

const DIGITS_1: usize = 2;
const DIGITS_4: usize = 4;
const DIGITS_7: usize = 3;
const DIGITS_8: usize = 7;

fn find_1_4_7_8(nums: &Vec<Vec<String>>) -> (usize, usize, usize, usize) {
    let mut one = 0;
    let mut four = 0;
    let mut seven = 0;
    let mut eight = 0;
    for i in 0..nums.len() {
        match nums[i].len() {
            DIGITS_1 => one = i,
            DIGITS_4 => four = i,
            DIGITS_7 => seven = i,
            DIGITS_8 => eight = i,
            _ => {}
        }
    }
    (one, four, seven, eight)
}

fn find_A_by_rule(one: &Vec<String>, seven: &Vec<String>) -> String {
    for i in 0..seven.len() {
        if !one.contains(&seven[i]) {
            return seven[i].clone();
        }
    }
    panic!("COULD NOT FIND A");
}

fn find_2_5(nums: &Vec<Vec<String>>) -> (usize, usize) {
    let mut five_elem_index: Vec<usize> = Vec::new();
    for i in 0..nums.len() {
        if nums[i].len() == 5 {
            five_elem_index.push(i);
        }
    }
    if five_elem_index.len() != 3 {
        panic!("MORE THEN 3 ELEMENTS WITH LEN 5");
    }

    let mut five_elem_hash_set: Vec<HashSet<String>> = Vec::new();
    for i in 0..five_elem_index.len() {
        let mut set = HashSet::new();
        for j in 0..nums[five_elem_index[i]].len() {
            set.insert(nums[five_elem_index[i]][j].clone());
        }
        five_elem_hash_set.push(set);
    }
    for i in 0..five_elem_hash_set.len() {
        for j in 0..five_elem_hash_set.len() {
            if i == j {
                continue;
            }
            let intersection_len = five_elem_hash_set[i]
                .intersection(&five_elem_hash_set[j])
                .collect::<Vec<&String>>()
                .len();
            //println!("INTERSECTION LEN: {}", intersection_len);
            if intersection_len == 3 {
                return (five_elem_index[i], five_elem_index[j]);
            }
        }
    }

    panic!("COULD NOT FIND POS 2/5");
}

fn find_D_by_rule(
    nums: &Vec<Vec<String>>,
    pos2_5_0: usize,
    pos2_5_1: usize,
    pos4: usize,
) -> String {
    let mut five_0_hash_set: HashSet<String> = HashSet::new();
    for i in 0..nums[pos2_5_0].len() {
        five_0_hash_set.insert(nums[pos2_5_0][i].clone());
    }
    let mut five_1_hash_set: HashSet<String> = HashSet::new();
    for i in 0..nums[pos2_5_1].len() {
        five_1_hash_set.insert(nums[pos2_5_1][i].clone());
    }
    let five_intersect: Vec<&String> = five_0_hash_set.intersection(&five_1_hash_set).collect();
    let mut total_five_hash_set: HashSet<String> = HashSet::new();
    for i in 0..five_intersect.len() {
        total_five_hash_set.insert(five_intersect[i].clone());
    }
    let mut four_hash_set: HashSet<String> = HashSet::new();
    for i in 0..nums[pos4].len() {
        four_hash_set.insert(nums[pos4][i].clone());
    }
    let resultant: Vec<&String> = four_hash_set.intersection(&total_five_hash_set).collect();
    if resultant.len() != 1 {
        panic!("MORE THAN 1 ELEMENTS MATCH D");
    }
    return resultant[0].clone();
}

fn find_G_by_rule(
    nums: &Vec<Vec<String>>,
    pos2_5_0: usize,
    pos2_5_1: usize,
    mapA: &String,
    mapD: &String,
) -> String {
    let mut five_0_hash_set: HashSet<String> = HashSet::new();
    for i in 0..nums[pos2_5_0].len() {
        five_0_hash_set.insert(nums[pos2_5_0][i].clone());
    }
    let mut five_1_hash_set: HashSet<String> = HashSet::new();
    for i in 0..nums[pos2_5_1].len() {
        five_1_hash_set.insert(nums[pos2_5_1][i].clone());
    }
    let five_intersect: Vec<&String> = five_0_hash_set.intersection(&five_1_hash_set).collect();
    for i in 0..five_intersect.len() {
        let val = five_intersect[i];
        if val == mapA {
            continue;
        } else if val == mapD {
            continue;
        } else {
            return val.clone();
        }
    }

    panic!("COULD NOT FIND G");
}

fn find_B_by_rule(nums: &Vec<Vec<String>>, pos1: usize, pos4: usize, mapD: &String) -> String {
    for i in 0..nums[pos4].len() {
        if nums[pos1].contains(&nums[pos4][i]) {
            continue;
        } else if &nums[pos4][i] == mapD {
            continue;
        } else {
            return nums[pos4][i].clone();
        }
    }

    panic!("COULD NOT FIND B");
}

fn find_5(nums: &Vec<Vec<String>>, pos2_5_0: usize, pos2_5_1: usize, mapB: &String) -> usize {
    for i in 0..nums[pos2_5_0].len() {
        if &nums[pos2_5_0][i] == mapB {
            return pos2_5_0;
        }
    }
    for i in 0..nums[pos2_5_1].len() {
        if &nums[pos2_5_1][i] == mapB {
            return pos2_5_1;
        }
    }
    panic!("COULD NOT FIND 5");
}

fn find_F_by_rule(
    nums: &Vec<Vec<String>>,
    pos5: usize,
    mapA: &String,
    mapB: &String,
    mapD: &String,
    mapG: &String,
) -> String {
    for i in 0..nums[pos5].len() {
        if &nums[pos5][i] == mapA {
            continue;
        } else if &nums[pos5][i] == mapB {
            continue;
        } else if &nums[pos5][i] == mapD {
            continue;
        } else if &nums[pos5][i] == mapG {
            continue;
        } else {
            return nums[pos5][i].clone();
        }
    }
    panic!("COULD NOT FIND F");
}

fn find_C_by_rule(nums: &Vec<Vec<String>>, pos1: usize, mapF: &String) -> String {
    for i in 0..nums[pos1].len() {
        if &nums[pos1][i] == mapF {
            continue;
        } else {
            return nums[pos1][i].clone();
        }
    }
    panic!("COULD NOT FIND C");
}

fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let mut ans_s: Vec<String> = Vec::new();

    let mut sum = 0;

    for line in 0..lines_s.len() {
        let note_ans_split: Vec<String> = lines_s[line].split("|").map(|x| x.to_string()).collect();
        let note_s: Vec<String> = note_ans_split[0]
            .trim_end()
            .to_string()
            .split(" ")
            .map(|x| x.to_string())
            .collect();
        println!("{}", note_ans_split[0].trim_end());
        for j in 0..note_s.len() {
            print!("{}: {}({}), ", j, note_s[j], note_s[j].len());
        }
        println!("");

        let note_split: Vec<Vec<String>> = note_s
            .iter()
            .map(|x| x.chars().map(|y| y.to_string()).collect())
            .collect();

        let letters: Vec<String> = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
        ];

        // step 1: find 1, 4, 7, 8
        let (pos1, pos4, pos7, pos8) = find_1_4_7_8(&note_split);
        println!("{}, {}, {}, {}", pos1, pos4, pos7, pos8);

        // step 2: find mapping for A by rule (rule: in 7 but not in 1)
        let mapA = find_A_by_rule(&note_split[pos1], &note_split[pos7]);
        println!("mapA: {}", mapA);

        // step 3: find 2/5
        let (pos2_5_0, pos2_5_1) = find_2_5(&note_split);
        println!("2/5 pos: {}/{}", pos2_5_0, pos2_5_1);

        // step 4: find mapping for D by rule (rule: in 2 and 4 and 5)
        let mapD = find_D_by_rule(&note_split, pos2_5_0, pos2_5_1, pos4);
        println!("mapD: {}", mapD);

        // step 5: find mapping for G by rule (rule: not A or D, in 2 and 5)
        let mapG = find_G_by_rule(&note_split, pos2_5_0, pos2_5_1, &mapA, &mapD);
        println!("mapG: {}", mapG);

        // step 6: find mapping for B by rule (rule: in 4 but not in 1, not D)
        let mapB = find_B_by_rule(&note_split, pos1, pos4, &mapD);
        println!("mapB: {}", mapB);

        // step 7: find 5 (has B)
        let pos5 = find_5(&note_split, pos2_5_0, pos2_5_1, &mapB);
        println!("5 pos: {}", pos5);

        // step8: find mapping for F by rule (rule: not A B D or G, in 5)
        let mapF = find_F_by_rule(&note_split, pos5, &mapA, &mapB, &mapD, &mapG);
        println!("mapF: {}", mapF);

        // step9: find mapping for C by rule (rule: in 1, not F)
        let mapC = find_C_by_rule(&note_split, pos1, &mapF);
        println!("mapC: {}", mapC);

        // step10: find mapping for e (leftover)
        let mut mapE = "".to_string();
        for i in 0..letters.len() {
            if letters[i] == mapA {
                continue;
            } else if letters[i] == mapB {
                continue;
            } else if letters[i] == mapC {
                continue;
            } else if letters[i] == mapD {
                continue;
            } else if letters[i] == mapF {
                continue;
            } else if letters[i] == mapG {
                continue;
            } else {
                mapE = letters[i].clone();
                break;
            }
        }
        println!("mapE: {}", mapE);

        let mut base_maps: Vec<Vec<String>> = Vec::new();
        base_maps.push(
            ["a", "b", "c", "e", "f", "g"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        );
        base_maps.push(["c", "f"].iter().map(|x| x.to_string()).collect());
        base_maps.push(
            ["a", "c", "d", "e", "g"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        );
        base_maps.push(
            ["a", "c", "d", "f", "g"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        );
        base_maps.push(["b", "c", "d", "f"].iter().map(|x| x.to_string()).collect());
        base_maps.push(
            ["a", "b", "d", "f", "g"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        );
        base_maps.push(
            ["a", "b", "d", "e", "f", "g"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        );
        base_maps.push(["a", "c", "f"].iter().map(|x| x.to_string()).collect());
        base_maps.push(
            ["a", "b", "c", "d", "e", "f", "g"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        );
        base_maps.push(
            ["a", "b", "c", "d", "f", "g"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        );

        let mut new_maps: Vec<Vec<String>> = Vec::new();
        for i in 0..base_maps.len() {
            let mut val: Vec<String> = Vec::new();
            for j in 0..base_maps[i].len() {
                match base_maps[i][j].as_str() {
                    "a" => val.push(mapA.clone()),
                    "b" => val.push(mapB.clone()),
                    "c" => val.push(mapC.clone()),
                    "d" => val.push(mapD.clone()),
                    "e" => val.push(mapE.clone()),
                    "f" => val.push(mapF.clone()),
                    "g" => val.push(mapG.clone()),
                    _ => panic!("INVALID"),
                }
            }
            new_maps.push(val);
        }
        //println!("new_maps:");
        //for i in 0..new_maps.len() {
        //    for j in 0..new_maps[i].len() {
        //        print!("{}", new_maps[i][j]);
        //    }
        //    println!("");
        //}



        println!("{}", note_ans_split[1]);

        let ans_s: Vec<String> = note_ans_split[1]
            .trim_start()
            .to_string()
            .split(" ")
            .map(|x| x.to_string())
            .collect();
        let ans_split: Vec<Vec<String>> = ans_s
            .iter()
            .map(|x| x.chars().map(|y| y.to_string()).collect())
            .collect();

        let mut display_nums: Vec<usize> = Vec::new();

        for i in 0..ans_split.len() {
            let mut num = 0;
            'map_loop: for map in 0..new_maps.len() {
                if new_maps[map].len() != ans_split[i].len() {
                    continue 'map_loop;
                }
                for j in 0..ans_split[i].len() {
                    //print!("{}, ", ans_split[i][j]);
                    if !new_maps[map].contains(&ans_split[i][j]) {
                        continue 'map_loop;
                    }
                }
                print!("matching map: ");
                for letter in 0..new_maps[map].len() {
                    print!("{}", new_maps[map][letter]);
                }
                println!("");
                num = map;
                break 'map_loop;
            }
            display_nums.push(num);
        }

        let mut display: String = String::new();
        for i in 0..display_nums.len() {
            display.push_str(&display_nums[i].to_string());
        }
        print!("DISPLAY: {}", display);

        sum = sum + display.parse::<u64>().unwrap();

        println!("\n");
    }

    println!("SUM: {}", sum);

    Ok(())
}
