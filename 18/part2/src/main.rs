use std::collections::HashMap;
use std::fs;

use eyre::Result;

#[derive(PartialEq, Eq, Clone, Copy)]
enum SnailNumType {
    NUMBER,
    PAIR,
}

#[derive(Clone, Copy)]
enum ParseMode {
    A,
    B,
}

#[derive(Clone)]
struct SnailNum {
    num_type: SnailNumType,
    num_val: usize,
    a_val: Option<Box<SnailNum>>,
    b_val: Option<Box<SnailNum>>,
}

fn parse_snailnum(num_s: &str) -> (SnailNum, usize) {
    let num_split: Vec<String> = num_s.chars().map(|x| x.to_string()).collect();

    let mut obj = SnailNum {
        num_type: SnailNumType::PAIR,
        num_val: 0,
        a_val: None,
        b_val: None,
    };
    let mut mode = ParseMode::A;

    let mut i = 0;
    loop {
        //print!("Parsing {} from {} ", num_split[i], num_s);
        match num_split[i].as_str() {
            "[" => {
                //println!("");
                let (sn, s_len) = parse_snailnum(num_s.get(i + 1..num_s.len()).unwrap());
                //print!("s_len: {}", s_len);
                match mode {
                    ParseMode::A => obj.a_val = Some(Box::new(sn)),
                    ParseMode::B => obj.b_val = Some(Box::new(sn)),
                }
                obj.num_type = SnailNumType::PAIR;
                i += s_len + 2;
            }
            "]" => {
                return (obj, i);
            }
            "," => {
                //print!("(changing parse mode ");
                mode = match mode {
                    ParseMode::A => {
                        //print!("A->B) ");
                        ParseMode::B
                    }
                    ParseMode::B => {
                        //print!("B->A) ");
                        ParseMode::A
                    }
                };
                i += 1;
            }
            x => {
                match mode {
                    ParseMode::A => {
                        //print!("(storing {} in A)", x);
                        obj.a_val = Some(Box::new(SnailNum {
                            num_type: SnailNumType::NUMBER,
                            num_val: x.parse::<usize>().unwrap(),
                            a_val: None,
                            b_val: None,
                        }))
                    }
                    ParseMode::B => {
                        //print!("(storing {} in B)", x);
                        obj.b_val = Some(Box::new(SnailNum {
                            num_type: SnailNumType::NUMBER,
                            num_val: x.parse::<usize>().unwrap(),
                            a_val: None,
                            b_val: None,
                        }))
                    }
                }
                i += 1;
            }
        }
        //println!("");
        if i == num_split.len() {
            break;
        }
    }

    (obj, num_s.len())
}

fn print_snailnum(sn: &SnailNum) {
    match sn.num_type {
        SnailNumType::NUMBER => print!("{}", sn.num_val),
        SnailNumType::PAIR => {
            print!("[");
            print_snailnum(&(*sn.a_val.as_ref().unwrap()));
            print!(",");
            print_snailnum(&(*sn.b_val.as_ref().unwrap()));
            print!("]");
        }
    }
}

fn explode_real_num_right(sn: &mut SnailNum, explode_val: usize) -> bool {
    match sn.num_type {
        SnailNumType::NUMBER => {
            sn.num_val += explode_val;
            return true;
        }
        SnailNumType::PAIR => {
            if explode_real_num_right(&mut sn.a_val.as_mut().unwrap(), explode_val) {
                return true;
            }
            if explode_real_num_right(&mut sn.b_val.as_mut().unwrap(), explode_val) {
                return true;
            }
            return false;
        }
    }
}

fn explode_real_num_left(sn: &mut SnailNum, explode_val: usize) -> bool {
    match sn.num_type {
        SnailNumType::NUMBER => {
            sn.num_val += explode_val;
            return true;
        }
        SnailNumType::PAIR => {
            if explode_real_num_left(&mut sn.b_val.as_mut().unwrap(), explode_val) {
                return true;
            }
            if explode_real_num_left(&mut sn.a_val.as_mut().unwrap(), explode_val) {
                return true;
            }
            return false;
        }
    }
}

fn explode(sn: &mut SnailNum, depth: usize) -> Option<(Option<usize>, Option<usize>)> {
    //print!("Exploding Snailnum: ");
    //print_snailnum(sn);
    //print!(" ");

    if sn.num_type == SnailNumType::NUMBER {
        return None;
    }
    if sn.a_val.as_ref().unwrap().num_type == SnailNumType::NUMBER
        && sn.b_val.as_ref().unwrap().num_type == SnailNumType::NUMBER
    {
        //println!("EXPLODED");
        if depth >= 4 {
            sn.num_type = SnailNumType::NUMBER;
            sn.num_val = 0;
            return Some((
                Some(sn.a_val.as_ref().unwrap().num_val),
                Some(sn.b_val.as_ref().unwrap().num_val),
            ));
        } else {
            return None;
        }
    } else {
        //println!("");
        // if a explodes
        if let Some(val) = explode(&mut sn.a_val.as_mut().unwrap(), depth + 1) {
            //print!("Processing snailnum: ");
            //print_snailnum(sn);
            //print!(" Child A exploded, ");
            // if b is a number
            if let Some(b_val) = val.1 {
                // set b to a.b + b
                if explode_real_num_right(&mut sn.b_val.as_mut().unwrap(), b_val) {
                    return Some((val.0, None));
                }
                //print!("New value: ");
                //print_snailnum(sn);
                //println!("");
            }
            //println!("");
            return Some(val);
        } else if let Some(val) = explode(&mut sn.b_val.as_mut().unwrap(), depth + 1) {
            //println!("Pocessing snailnum: ");
            //print_snailnum(sn);
            //print!(" Child B exploded, ");
            // if b is a number
            if let Some(a_val) = val.0 {
                // set a to b.a + a
                if explode_real_num_left(&mut sn.a_val.as_mut().unwrap(), a_val) {
                    return Some((None, val.1));
                }
                //print!("New value: ");
                //print_snailnum(sn);
                //println!("");
            }
            //println!("");
            return Some(val);
        } else {
            return None;
        }
    }
}

fn split(sn: &mut SnailNum) -> bool {
    match sn.a_val.as_ref().unwrap().num_type {
        SnailNumType::NUMBER => {
            let a = sn.a_val.as_ref().unwrap();
            if a.num_val >= 10 {
                sn.a_val = Some(Box::new(SnailNum {
                    num_type: SnailNumType::PAIR,
                    num_val: 0,
                    a_val: Some(Box::new(SnailNum {
                        num_type: SnailNumType::NUMBER,
                        num_val: a.num_val / 2,
                        a_val: None,
                        b_val: None,
                    })),
                    b_val: Some(Box::new(SnailNum {
                        num_type: SnailNumType::NUMBER,
                        num_val: (a.num_val / 2) + (a.num_val % 2),
                        a_val: None,
                        b_val: None,
                    })),
                }));
                return true;
            }
        }
        SnailNumType::PAIR => {
            let ret = split(&mut sn.a_val.as_mut().unwrap());
            if ret {
                return ret;
            }
        }
    }
    match sn.b_val.as_ref().unwrap().num_type {
        SnailNumType::NUMBER => {
            let b = sn.b_val.as_ref().unwrap();
            if b.num_val >= 10 {
                sn.b_val = Some(Box::new(SnailNum {
                    num_type: SnailNumType::PAIR,
                    num_val: 0,
                    a_val: Some(Box::new(SnailNum {
                        num_type: SnailNumType::NUMBER,
                        num_val: b.num_val / 2,
                        a_val: None,
                        b_val: None,
                    })),
                    b_val: Some(Box::new(SnailNum {
                        num_type: SnailNumType::NUMBER,
                        num_val: (b.num_val / 2) + (b.num_val % 2),
                        a_val: None,
                        b_val: None,
                    })),
                }));
                return true;
            }
        }
        SnailNumType::PAIR => {
            let ret = split(&mut sn.b_val.as_mut().unwrap());
            if ret {
                return true;
            }
        }
    }
    return false;
}

fn reduce_snailnum(sn: &mut SnailNum) {
    print!("Reducing snailnum: ");
    //print_snailnum(sn);
    //println!("");
    loop {
        print_snailnum(sn);
        println!("");

        // rule 1
        let exploded = match explode(sn, 0) {
            Some(_) => true,
            None => false,
        };
        if exploded {
            continue;
        }

        // rule 2
        let split = split(sn);
        if split {
            continue;
        }

        break;
        //if !exploded && !split {
        //    break;
        //}


    }
}

fn calculate_magnitude(sn: &SnailNum) -> usize {
    match sn.num_type {
        SnailNumType::NUMBER => sn.num_val,
        SnailNumType::PAIR => {
            let l = 3 * calculate_magnitude(sn.a_val.as_ref().unwrap());
            let r = 2 * calculate_magnitude(sn.b_val.as_ref().unwrap());
            l + r
        }
    }

}

fn add_snailnums(sn0: SnailNum, sn1: SnailNum) -> SnailNum {
    SnailNum {
        num_type: SnailNumType::PAIR,
        num_val: 0,
        a_val: Some(Box::new(sn0)),
        b_val: Some(Box::new(sn1)),
    }
}

fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();

    let (mut summer, len) = parse_snailnum(&lines_s[0].get(1..lines_s[0].len() - 1).unwrap());

    let mut snailnums: Vec<SnailNum> = Vec::new();
    for i in 1..lines_s.len() {
        let (sn, len) = parse_snailnum(&lines_s[i].get(1..lines_s[i].len() - 1).unwrap());
        snailnums.push(sn);
    }

    //let explode_test_nums = [
    //    "[[[[9,8],1],2],3],4",
    //    "7,[6,[5,[4,[3,2]]]]",
    //    "[6,[5,[4,[3,2]]]],1",
    //    "[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]",
    //];
    //for num in explode_test_nums {
    //    let (mut sn, size) = parse_snailnum(num);
    //    println!("");
    //    print_snailnum(&sn);
    //    println!("");
    //    println!("");
    //    reduce_snailnum(&mut sn);
    //    println!("");
    //    print_snailnum(&mut sn);
    //    println!("");
    //}

    let add_test_nums = [
        ("[[[4,3],4],4],[7,[[8,4],9]]", "1,1"),
        //("", ""),
        //("", ""),
        //("", ""),
    ];
    for (num0, num1) in add_test_nums {
        let (mut sn0, size0) = parse_snailnum(num0);
        let (mut sn1, size1) = parse_snailnum(num1);
        println!("");
        print_snailnum(&sn0);
        println!("");
        print_snailnum(&sn1);
        println!("");
        let mut sn2 = add_snailnums(sn0, sn1);
        print_snailnum(&sn2);
        println!("");
        reduce_snailnum(&mut sn2);
        println!("");
        print_snailnum(&sn2);
        println!("");
    }

    let add_list_nums = [
        "[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]",
        "[[5,[2,8]],4],[5,[[9,9],0]]",
        "6,[[[6,2],[5,6]],[[7,6],[4,7]]]",
        "[[6,[0,7]],[0,9]],[4,[9,[9,0]]]",
        "[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]",
        "[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]",
        "[[[5,4],[7,7]],8],[[8,3],8]",
        "[9,3],[[9,9],[6,[4,9]]]",
        "[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]",
        "[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]",
    ];



   }
    }

    print_snailnum(&summer);
    println!("");
    for i in 1..lines_s.len() {
        let (sn, len) = parse_snailnum(&lines_s[i].get(1..lines_s[i].len() - 1).unwrap());
        summer = add_snailnumsailnums[i](summer, sn);
        reduce_snailnum(&mut summer);
        print_snailnum(&summer);
        println!("");
    }

    let sum = calculate_magnitude(&summer);
    println!("SUM: {}", sum);

    //for i in 0..snailnums.len() {
    //    print!("\nPrinting snailnum: ");
    //    print_snailnum(&snailnums[i]);
    //}

    Ok(())
}
