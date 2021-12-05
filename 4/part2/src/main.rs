use std::fs;

use eyre::Result;

struct Board {
    board: [[(u64, bool); 5]; 5],
}

impl Board {
    fn fill_in_number(&mut self, num: u64) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j].0 == num {
                    self.board[i][j].1 = true;
                }
             }
        }
    }

    fn check_if_win(&self) -> bool {
        let mut win;
        for i in 0..5 {
            win = true;
            for j in 0..5 {
                if self.board[i][j].1 == false {
                    win = false;
                    break;
                }
            }
            if win {
                return true;
            }
        }

        for j in 0..5 {
            win = true;
            for i in 0..5 {
                if self.board[i][j].1 == false {
                    win = false;
                    break;
                }
            }
            if win {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u64 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j].1 == false {
                    sum = sum + self.board[i][j].0;
                }
            }
        }
        sum
    }
}

fn main() -> Result<()> {
    let filename1 = "input1";

    let contents = fs::read_to_string(filename1).expect("Something went wrong reading the file");

    let mut numbers_s: Vec<&str> = contents.split(",").collect();
    numbers_s.pop();
    let numbers: Vec<u64> = numbers_s
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let filename2 = "input2";

    let contents = fs::read_to_string(filename2).expect("Something went wrong reading the file");

    let board_nums_s_0: Vec<String> = contents.split(" ").map(|x| x.to_string()).collect();
    let mut board_nums_s_1: Vec<String> = Vec::new();
    for i in 0..board_nums_s_0.len() {
        let x: Vec<String> = board_nums_s_0[i]
            .split("\n")
            .map(|x| x.to_string())
            .collect();
        for j in 0..x.len() {
            board_nums_s_1.push(x[j].clone());
        }
    }
    let mut board_nums_n: Vec<u64> = Vec::new();
    for i in 0..board_nums_s_1.len() {
        match board_nums_s_1[i].parse::<u64>() {
            Ok(v) => board_nums_n.push(v),
            Err(_) => (),
        };
    }

    println!("BOARD NUMS: {}", board_nums_n.len());
    if board_nums_n.len() % 25 != 0 {
        panic!("Board nums not divisible by 25");
    }
    let num_boards = board_nums_n.len() / 25;

    let mut boards: Vec<Board> = Vec::new();

    let mut num_index = 0;
    for _ in 0..num_boards {
        let mut x: Board = Board {
            board: [[(0, false); 5]; 5],
        };
        for i in 0..5 {
            for j in 0..5 {
                x.board[i][j].0 = board_nums_n[num_index];
                num_index = num_index + 1;
            }
        }
        boards.push(x);
    }
    if num_index != board_nums_n.len() {
        panic!("Something went wrong filling the boards")
    }

    let mut board_num_set = std::collections::HashSet::new();
    for i in 0..num_boards {
        board_num_set.insert(i);
    }

    let mut last_board_num = 0;
    let mut last_number_called = 0;

    'outer: for i in 0..numbers.len() {
        let current_number = numbers[i];
        println!("Pulling number {}", current_number);
        for j in 0..boards.len() {
            if !board_num_set.contains(&j) {
                continue;
            }
            boards[j].fill_in_number(current_number);
            if boards[j].check_if_win() {
                println!("BOARD {} won", j + 1);
                board_num_set.remove(&j);
            }
            if board_num_set.len() == 0 {
                last_board_num = j;
                last_number_called = current_number;
                break 'outer;
            }
        }
    }

    let sum = boards[last_board_num].sum_unmarked();
    let score = sum * last_number_called;

    println!("Last board to win: {}", last_board_num + 1);
    println!("Final score: {}", score);


    Ok(())
}
