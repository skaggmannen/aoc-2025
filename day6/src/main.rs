use std::collections::VecDeque;

fn main() {
    let input = util::read_input("day6/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let lines = util::to_lines(input);
    let values: Vec<Vec<_>> = lines[0..lines.len() - 1]
        .iter()
        .map(|l| {
            l.split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let ops: Vec<_> = lines[lines.len() - 1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect();

    let result: u64 = ops
        .iter()
        .enumerate()
        .map(|(i, &op)| match op {
            "+" => values.iter().fold(0, |acc, v| acc + v[i]),
            "*" => values.iter().fold(1, |acc, v| acc * v[i]),
            _ => panic!("invalid op: {}", op),
        })
        .sum();

    format!("{}", result)
}

fn part2(input: &str) -> String {
    let lines: Vec<_> = input.split("\n").filter(|s| !s.is_empty()).collect();

    let values: Vec<Vec<_>> = lines[0..lines.len() - 1]
        .iter()
        .map(|l| l.chars().collect())
        .collect();
    let ops: Vec<_> = lines[lines.len() - 1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect();

    let mut parsed_values = VecDeque::<Vec<u64>>::new();
    let mut collected = VecDeque::<u64>::new();

    for i in 0..lines[0].len() {
        let digits: String = values.iter().map(|l| l[i]).collect();

        if digits.trim().is_empty() {
            // This is a delimiter. Store the collected values.
            parsed_values.push_back(collected.into_iter().collect());
            collected = VecDeque::<u64>::new();
        } else {
            // This is a value. Push it to the list of collected values.
            collected.push_back(digits.trim().parse().unwrap());
        }
    }

    parsed_values.push_back(collected.into_iter().collect());

    let result: u64 = ops
        .iter()
        .enumerate()
        .map(|(i, &op)| match op {
            "+" => parsed_values[i].iter().fold(0, |acc, v| acc + v),
            "*" => parsed_values[i].iter().fold(1, |acc, v| acc * v),
            _ => panic!("invalid op: {}", op),
        })
        .sum();

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    const INPUT: &[&str] = &[
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   + ",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT.join("\n").as_str()), "4277556");
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(INPUT.join("\n").as_str()), "3263827");
    }
}
