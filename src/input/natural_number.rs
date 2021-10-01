use std::io;

pub(crate) enum NaturalNumberInput {
    Number(usize),
    TooHigh(usize),
    TooLow(usize),
    NaN(String),
}

pub(crate) fn parse(lower_bound: usize, upper_bound: usize) -> NaturalNumberInput {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(input) if input < lower_bound => NaturalNumberInput::TooLow(input),
        Ok(input) if input > upper_bound => NaturalNumberInput::TooHigh(input),
        Ok(input) => NaturalNumberInput::Number(input),
        Err(_) => NaturalNumberInput::NaN(input.trim().to_string()),
    }
}
