fn main() {
    let input = util::read_input("day3/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let result: u32 = util::to_lines(input)
        .iter()
        .map(|s| find_combination(s, 2).parse::<u32>().unwrap())
        .sum();

    format!("{}", result)
}

fn part2(input: &str) -> String {
    let result: u64 = util::to_lines(input)
        .iter()
        .map(|s| find_combination(s, 12).parse::<u64>().unwrap())
        .sum();

    format!("{}", result)
}

fn find_combination(s: &str, count: usize) -> String {
    // Check if there are even enough digits left to choose from.
    if s.len() <= count {
        // Nope, so just return the rest of the string.
        return s.to_string();
    } else if count == 0 {
        return "".to_string();
    }

    // Choose the first instance of the highest digit in the string.
    let mut max_i: usize = 0;
    let mut max_c: char = '0';

    for (i, c) in s[0..s.len() - (count - 1)].chars().enumerate() {
        if c > max_c {
            max_c = c;
            max_i = i;
        }
    }

    let mut next = find_combination(&s[max_i + 1..], count - 1);
    next.insert(0, max_c);
    next
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
    fn test_part2() {
        assert_eq!(super::part2(INPUT), "3121910778619");
    }
}
