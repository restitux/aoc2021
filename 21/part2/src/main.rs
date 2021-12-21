use std::collections::HashMap;
use std::fs;

use eyre::Result;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Turn {
    PLAYER1,
    PLAYER2,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct State {
    p1_pos: usize,
    p2_pos: usize,
    p1_score: usize,
    p2_score: usize,
}

fn turn(state: &mut State, die: usize, turn: Turn) -> bool {
    match turn {
        Turn::PLAYER1 => {
            state.p1_pos += die;
            state.p1_pos %= 10;
            state.p1_score += state.p1_pos + 1;
            state.p1_score >= 21
        },
        Turn::PLAYER2 => {
            state.p2_pos += die;
            state.p2_pos %= 10;
            state.p2_score += state.p2_pos + 1;
            state.p2_score >= 21
        }
    }
}

fn main() -> Result<()> {
    let mut state_map: HashMap<State, usize> = HashMap::new();
    let mut complete_state_map: HashMap<State, usize> = HashMap::new();

    let inital_state = State {
        p1_pos: 6,
        p2_pos: 5,
        p1_score: 0,
        p2_score: 0,
    };

    let mut current_turn = Turn::PLAYER1;

    state_map.insert(inital_state, 1);

    loop {
        if state_map.len() == 0 {
            println!("No more states left to process");
            break;
        }
        let mut n_state_map = HashMap::new();

        for (state, count) in state_map {
            for i in 1..4 {
                for j in 1..4 {
                    for k in 1..4 {
                        let mut new_state = state.clone();
                        if turn(&mut new_state, i + j + k, current_turn) {
                            if let Some(l_count) = complete_state_map.get_mut(&new_state) {
                                *l_count += count;
                            } else {
                                complete_state_map.insert(new_state, count);
                            }
                        } else {
                            if let Some(l_count) = n_state_map.get_mut(&new_state) {
                                *l_count += count;
                            } else {
                                n_state_map.insert(new_state, count);
                            }
                        }
                    }
                }
            }
        }

        let mut current_states = 0;
        for (_, count) in n_state_map.iter() {
            current_states += count;
        }

        println!("n_state_map.len(): {}", n_state_map.len());
        println!("current_states: {}", current_states);

        current_turn = match current_turn {
            Turn::PLAYER1 => Turn::PLAYER2,
            Turn::PLAYER2 => Turn::PLAYER1,
        };

        state_map = n_state_map;
    }

    let mut p1_wins = 0;
    let mut p2_wins = 0;

    let mut total_states = 0;
   
    for (state, count) in complete_state_map {
        println!("State: (count: {})", count);
        println!("    P1: pos: {}, score {}", state.p1_pos, state.p1_score);
        println!("    P2: pos: {}, score {}", state.p2_pos, state.p2_score);
        if state.p1_score > state.p2_score {
            p1_wins += count;
        } else {
            p2_wins += count;
        }
        total_states += count;
    }

    println!("Total states: {}", total_states);
    println!("P1 wins: {}, P2 wins: {}", p1_wins, p2_wins);
    println!("{}", p1_wins > p2_wins);

    Ok(())
}
