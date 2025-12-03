use std::collections::VecDeque;

fn main() {
    let input = util::read_input("day3/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let result: u32 = util::to_lines(input)
        .iter()
        .map(|s| {
            let values: Vec<_> = s
                .chars()
                .map(|c| format!("{}", c).parse::<u32>().unwrap())
                .collect();

            let mut combinations = VecDeque::new();
            for i in 0..values.len() {
                for j in i + 1..values.len() {
                    combinations.push_back(values[i] * 10 + values[j]);
                }
            }

            let max = combinations.iter().max().unwrap();

            println!("{} -> {}", s, max);

            *max
        })
        .sum();

    format!("{}", result)
}

fn part2(_input: &str) -> String {
    format!("0")
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT), "357");
    }

    #[test]
    fn test_part2() {}
}
