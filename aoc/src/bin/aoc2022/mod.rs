use crate::{Selector, Solution};

mod aoc2022_01;
mod aoc2022_02;
mod aoc2022_03;
mod aoc2022_04;
mod aoc2022_05;
mod aoc2022_06;

use aoc2022_01::*;
use aoc2022_02::*;
use aoc2022_03::*;
use aoc2022_04::*;
use aoc2022_05::*;
use aoc2022_06::*;

pub fn run_2022(which: Selector) {
    let mut day_01 = Aoc2022_01::new();
    let mut day_02 = Aoc2022_02::new();
    let mut day_03 = Aoc2022_03::new();
    let mut day_04 = Aoc2022_04::new();
    let mut day_05 = Aoc2022_05::new();
    let mut day_06 = Aoc2022_06::new();

    let mut days: Vec<&mut dyn Solution> = vec![
        &mut day_01, &mut day_02, &mut day_03, &mut day_04, &mut day_05,
        &mut day_06
    ];

    match which {
        Selector::Last => {
            let last = days.len() -1;
            let d = &mut days[last];
            crate::run_solution(*d);
        },
        Selector::All => {
            for d in days {
                crate::run_solution(d);
            }
        },
        Selector::Day(day) => {
            let d = &mut days[day - 1];
            crate::run_solution(*d);
        },
        _ => {}
    }
}