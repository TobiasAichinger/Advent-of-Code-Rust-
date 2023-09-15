use crate::day::{Input, InputError, InputResult, Output, Wrapper};

fn part_one(inp: &Vec<String>) -> Output {
    Output::Nu32(
        inp.iter()
            .map(|s| {
                let half = s.len() / 2;
                get_value(s, &s[..half], &s[half..])
            })
            .sum::<u32>(),
    )
}

fn part_two(inp: &Vec<String>) -> Output {
    Output::Nu32(
        (0..inp.len())
            .step_by(3)
            .map(|i| get_value(&inp[i], &inp[i + 1], &inp[i + 2]))
            .sum::<u32>(),
    )
}

fn get_value(fst: &str, scn: &str, thr: &str) -> u32 {
    let c = fst
        .chars()
        .nth(fst.find(|c| scn.contains(c) && thr.contains(c)).unwrap())
        .unwrap();

    if c.is_uppercase() {
        (c as u8 - ('A' as u8 - 27u8)) as u32
    } else {
        (c as u8 - ('a' as u8 - 1u8)) as u32
    }
}

pub fn run(input: Input) -> (Output, Output) {
    let input = input.unwrap();

    (part_one(&input), part_two(&input))
}

pub fn parse() -> InputResult<Input> {
    let input: Vec<String> = std::fs::read_to_string("../input/03")?
        .lines()
        .map(|l| l.to_string())
        .collect();

    if input.len() % 3 != 0 {
        return Err(InputError::InvalidInput);
    }

    Ok(Input::Vstr(input))
}