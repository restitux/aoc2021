use std::collections::HashMap;
use std::fs;

use eyre::Result;

use bit_vec::BitVec;

struct Literal {
    version: usize,
    bits: String,
    val: usize,
}

fn parse_literal(bits: &String, offset: usize, version: usize) -> (Literal, usize) {
    print!("Parsing literal packet: ");
    let mut literal_bits: String = "".to_string();

    let mut local_offset = 0;
    loop {
        let next_five_bits = bits.get(offset + local_offset..offset + local_offset+5).unwrap();
        local_offset = local_offset + 5;
        print!("Next 5: {}, ", next_five_bits);
        let rule_bit = next_five_bits.get(0..1).unwrap();
        print!("Rule bit: {}, ", rule_bit);
        literal_bits = literal_bits + next_five_bits.get(1..5).unwrap();
        if rule_bit == "0" {
            break;
        }
    }

    let val = usize::from_str_radix(literal_bits.as_str(), 2).unwrap();
    print!("Size: {}, ", local_offset);
    print!("Val: {}, ", val);
    println!("");

    (Literal {
        version,
        bits: literal_bits.clone(),
        val,
    }, local_offset)
}

struct Operator {
    version_sum: usize,
    val: usize,
}

fn parse_operator(bits: &String, operation: usize, offset: usize, version: usize) -> (Operator, usize) {
    print!("Parsing operator packet: ");
    let mut local_offset = 0;
    let length_type = bits.get(offset + local_offset..offset + local_offset+1).unwrap();
    local_offset = local_offset + 1;
    let mut version_sum = version;
    let mut vals: Vec<usize> = Vec::new();
    if length_type == "0" {
        print!("Length Type: 0, ");
        let subpacket_length_s = bits.get(offset + local_offset..offset + local_offset + 15).unwrap();
        print!("subpacket_length_s: {} ", subpacket_length_s);
        local_offset = local_offset + 15;
        let subpacket_base = local_offset;
        let subpacket_length = usize::from_str_radix(subpacket_length_s, 2).unwrap();
        println!("Length: {}", subpacket_length);
        loop {
            let (packet_version_sum, packet_size, val) = parse(bits, offset + local_offset);
            vals.push(val);
            version_sum = version_sum + packet_version_sum;
            local_offset = local_offset + packet_size;
            if local_offset - subpacket_base == subpacket_length {
                break;
            }
        }
    } else {
        print!("Length Type: 1, ");
        let num_subpackets_s = bits.get(offset + local_offset..offset +local_offset + 11).unwrap();
        println!("num_subpackets_s: {}", num_subpackets_s);
        local_offset = local_offset + 11;
        let num_subpackets = usize::from_str_radix(num_subpackets_s, 2).unwrap();

        for _ in 0..num_subpackets {
            let (packet_version_sum, packet_size, val) = parse(bits, offset + local_offset);
            vals.push(val);
            version_sum = version_sum + packet_version_sum;
            local_offset = local_offset + packet_size;
        }
    }

    let mut output_val = match operation {
        0 => {
            // do sum
            let mut sum = 0;
            for i in 0..vals.len() {
                sum = sum + vals[i];
            }
            sum
        },
        1 => {
            // do product
            let mut product = vals[0];
            for i in 1..vals.len() {
                product = product * vals[i];
            }
            product
        },
        2 => {
            // do min
            let mut min = std::usize::MAX;
            for i in 0..vals.len() {
                if vals[i] < min {
                    min = vals[i];
                }
            }
            min
        },
        3 => {
            // do max
            let mut max = 0;
            for i in 0..vals.len() {
                if vals[i] > max {
                    max = vals[i]
                }
            }
            max
        },
        5 => {
            // do greater than
            match vals[0] > vals[1] {
                true => 1,
                false => 0,
            }
        },
        6 => {
            // do less than
            match vals[0] < vals[1] {
                true => 1,
                false => 0,
            }
        },
        7 => {
            // do equal to
            match vals[0] == vals[1] {
                true => 1,
                false => 0,
            }
        },
        _ => panic!("Unknown operation"),
    };

    (Operator{version_sum, val: output_val}, local_offset)
}

fn parse(bits: &String, offset: usize) -> (usize, usize, usize) {
    let version_s = bits.get(offset..offset + 3).unwrap();
    let id_s = bits.get(offset + 3..offset + 6).unwrap();
    let version = usize::from_str_radix(version_s, 2).unwrap();
    let id = usize::from_str_radix(id_s, 2).unwrap();
    print!("Version: {}, ", version_s);
    print!("Type ID: {}, ", id_s);
    print!("Version: {}, ", version);
    println!("Type ID: {}, ", id);

    match id {
        4 => {
            let (l, n_offset) = parse_literal(&bits, offset + 6, version);
            //println!("{}", l.bits);
            return (l.version, n_offset + 6, l.val);
        }
        _ => {
            let (o, n_offset) = parse_operator(&bits, id, offset + 6, version);
            //println!("{}", o.bits);
            return (o.version_sum, n_offset + 6, o.val);
        }
    }
}

fn main() -> Result<()> {
    let filename1 = "input";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    lines_s.pop();


    //let mut bits = BitVec::new();
    let mut bits: String = "".to_string();

    //let vals: Vec<usize> = lines_s[0].chars().map(|x| usize::from_str_radix(x.to_string().as_str(), 16).unwrap()).collect();
    let vals: Vec<String> = lines_s[0].chars().map(|x| x.to_string()).collect();

    for i in 0..vals.len() {
        let val = match vals[i].as_str() {
            "0" => "0000",
            "1" => "0001",
            "2" => "0010",
            "3" => "0011",
            "4" => "0100",
            "5" => "0101",
            "6" => "0110",
            "7" => "0111",
            "8" => "1000",
            "9" => "1001",
            "A" => "1010",
            "B" => "1011",
            "C" => "1100",
            "D" => "1101",
            "E" => "1110",
            "F" => "1111",
            _ => panic!(""),
        };
        bits = bits + val;
        //for bit in 0..4 {
        //    let val = vals[i] & (0x1 << (3 - bit));
        //    bits.push(val != 0);
        //}
    }

    println!("{}", bits);

    let (version_sum, offset, val) = parse(&bits, 0);
    println!("VSUM: {}", version_sum);
    println!("VAL: {}", val);

    //for i in 0..bits.len() {
    //    match bits.get(i) {
    //        Some(true) => print!("1"),
    //        Some(false) => print!("0"),
    //        None => panic!(""),
    //    };
    //}
    //println!("");

    //for i in 0..lines_s.len() {
    //    let t = lines_s[i]
    //        .chars()
    //        .map(|x| x.to_string().parse::<isize>().unwrap())
    //        .collect();
    //    map.map.push(t);
    //}

    //map.x = map.map.len() as isize;
    //map.y = map.map[0].len() as isize;

    ////for y in 0..map.y {
    ////    for x in 0..map.x {
    ////        print!("{}", map.map[y][x]);
    ////    }
    ////    println!("");
    ////}

    //let mut distance_map: Vec<Vec<isize>> = Vec::new();
    //for y in 0..map.y {
    //    distance_map.push(vec![]);
    //    for _ in 0..map.x {
    //        distance_map[y as usize].push(std::isize::MAX);
    //    }
    //}
    //let mut previous: HashMap<Point, Option<Point>> = HashMap::new();
    //let mut pq = PriorityQueue::<Point, isize>::new();

    //for y in 0..map.y {
    //    for x in 0..map.x {
    //        if x != 0 || y != 0 {
    //            previous.insert(Point{x, y}, None);
    //        } else {
    //            distance_map[y as usize][x as usize] = 0;
    //        }
    //        pq.push(Point{x, y}, std::isize::MAX - distance_map[y as usize][x as usize]);

    //    }
    //}

    //while !pq.is_empty() {
    //    if let Some((point, distance)) = pq.pop() {
    //        for neighbor in [Point{x: point.x + 1, y: point.y}, Point{x: point.x - 1, y: point.y}, Point{x: point.x, y: point.y + 1}, Point{x: point.x, y: point.y - 1}].iter() {
    //            if let None = pq.get(neighbor) {
    //                continue;
    //            }
    //            if neighbor.x >= 0 && neighbor.x < map.x && neighbor.y >= 0 && neighbor.y < map.y {
    //                let alt = distance_map[point.y as usize][point.x as usize] + map.map[neighbor.y as usize][neighbor.x as usize];
    //                if alt < distance_map[neighbor.y as usize][neighbor.x as usize] {
    //                    distance_map[neighbor.y as usize][neighbor.x as usize] = alt;
    //                    previous.insert(neighbor.clone(), Some(point));
    //                    pq.change_priority(neighbor, std::isize::MAX - alt);
    //                }
    //            }
    //        }
    //    }
    //}


    //for y in 0..map.y {
    //    for x in 0..map.x {
    //        print!("{} ", distance_map[y as usize][x as usize]);
    //    }
    //    println!("");
    //}

    //let mut dots_t: Vec<(usize, usize)> = Vec::new();

    //let mut max_x = 0;
    //let mut max_y = 0;

    //let mut template: Vec<String> = lines_s[0].chars().map(|x| x.to_string()).collect();

    //let filename2 = "input-ins";

    //let contents = fs::read_to_string(filename2).expect("something went wrong reading the file");

    //let mut lines_s: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    //lines_s.pop();

    ////let mut insertions: Vec<PairInsertion> = Vec::new();
    //let mut insertions: HashMap<(String, String), String> = HashMap::new();

    //for i in 0..lines_s.len() {
    //    let t: Vec<String> = lines_s[i].split(" -> ").map(|x| x.to_string()).collect();
    //    let input: Vec<String> = t[0].chars().map(|x| x.to_string()).collect();

    //    insertions.insert((input[0].clone(), input[1].clone()), t[1].clone());
    //}

    //for i in 0..template.len() {
    //    print!("{}", template[i]);
    //}
    //println!("\n");
    //for (k, v) in insertions.iter() {
    //    //println!("{}{} -> {}", insertions[i].input_a, insertions[i].input_b, insertions[i].output);
    //    println!("{}{} -> {}", k.0, k.1, v);
    //}

    //for _ in 0..10 {
    //    let mut new_template: Vec<String> = Vec::new();
    //    new_template.push(template[0].clone());
    //    for i in 0..template.len() - 1 {
    //        println!("Checking if {}{}", template[i], template[i + 1]);
    //        match insertions.get(&(template[i].clone(), template[i + 1].clone())) {
    //            Some(o) => new_template.push(o.clone()),
    //            None => ()
    //        }
    //        new_template.push(template[i + 1].clone());
    //    }
    //    for i in 0..new_template.len() {
    //        print!("{}", new_template[i]);
    //    }
    //    println!(" ({})", new_template.len());
    //    template = new_template;
    //}

    //let mut counters: HashMap<String, usize> = HashMap::new();

    //for i in 0..template.len() {
    //    if let Some(x) = counters.get_mut(&template[i]) {
    //        *x = *x + 1
    //    } else {
    //        counters.insert(template[i].clone(), 0);
    //    };
    //}

    //let mut max = 0;
    //let mut min = std::usize::MAX;

    //for (_, v) in counters.iter() {
    //    if *v > max {
    //        max = *v;
    //    }
    //    if *v < min {
    //        min = *v;
    //    }
    //}

    //println!("OUTPUT: {}", max - min);

    //    for i in 0..new_template.len() {
    //        print!("{}", new_template[i]);
    //    }
    //    panic!("TEST");
    //}

    Ok(())
}
