use std::fs;

fn parse_string<'a>(input: &'a str, to_match: &'static str) -> Option<&'a str> {
    if input.len() < to_match.len() || &input[0..to_match.len()] != to_match {
        None
    } else {
        Some(&input[to_match.len()..])
    }
}

fn parse_number_length(input: &str, length: usize) -> Option<(&str, u32)> {
    if input.len() < length {
        return None;
    }

    match input[..length].parse::<u32>() {
        Ok(a) => Some((&input[length..], a)),
        Err(_) => None,
    }
}

fn parse_number(input: &str) -> Option<(&str, u32)> {
    [3, 2, 1]
        .iter()
        .filter_map(|length| parse_number_length(input, *length))
        .next()
}

fn parse_instruction(to_check: &str) -> Option<u32> {
    let rest = parse_string(to_check, "mul(")?;
    let (rest, a) = parse_number(rest)?;
    let rest = parse_string(rest, ",")?;
    let (rest, b) = parse_number(rest)?;
    parse_string(rest, ")")?;

    Some(a * b)
}

fn parse_do(input: &str) -> Option<bool> {
    if parse_string(input, "do()").is_some() {
        Some(true)
    } else if parse_string(input, "don't()").is_some() {
        Some(false)
    } else {
        None
    }
}

fn run(input: &str) {
    let mul_sum_part_1: u32 = (0..input.len())
        .filter_map(|start| parse_instruction(&input[start..]))
        .sum();

    println!("Sum of valid mul instructions part 1: {mul_sum_part_1}");

    let (_, mul_sum_part_2) = (0..input.len()).fold((true, 0), |(enabled, mut sum), start| {
        let rest = &input[start..];
        let enabled = parse_do(rest).unwrap_or(enabled);

        if enabled {
            sum += parse_instruction(rest).unwrap_or(0);
        }

        (enabled, sum)
    });
    println!("Sum of valid mul instructions part 2: {mul_sum_part_2}");
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    run(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Run with cargo test -- --nocapture
    #[test]
    fn sample_input_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        run(&input);
    }
}
