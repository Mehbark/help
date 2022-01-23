mod space;
mod state;

use itertools::Itertools;

use crate::{space::*, state::*};

const PADDING: usize = 13;
const START_X: usize = 4311;
const START_Y: usize = 0;

fn main() {
    let test = State::new([Space::Empty; 9], Turn::X);
    // println!("{test}");
    let mut all_states = all_successors(&test, &Vec::new());
    all_states.sort();
    all_states.dedup();
    all_states.reverse();
    // println!("{all_states:#?}");
    // println!("{}", all_states.len());

    let positioned: Vec<((usize, usize), State)> = all_states
        .iter()
        .enumerate()
        .map(|(i, s)| ((START_X, START_Y + PADDING * i), *s))
        .collect();

    let chunks: Vec<String> = all_states
        .into_iter()
        .map(|s| {
            format!(
                "{s}\n{}",
                s.successors()
                    .into_iter()
                    .map(|succ| {
                        let (x, y) = positioned
                            .iter()
                            .find(|(_, state)| *state == succ.1)
                            .unwrap()
                            .0;
                        // because counting from zero is annoying
                        format!("{}: @{x},{y}", succ.0 + 1)
                    })
                    .join("\n")
            )
        })
        .collect();

    for chunk in chunks {
        println!(
            "{chunk}{}",
            "\n".repeat(PADDING - chunk.split('\n').count())
        );
    }
}

fn all_successors(s: &State, path: &[usize]) -> Vec<State> {
    let mut successors = vec![*s];
    for (i, state) in s.successors() {
        let mut path = Vec::from(path);
        path.push(i);
        let more = all_successors(&state, &path);
        successors.extend(more.into_iter());
    }
    successors
}
