use std::{cmp::max, collections::VecDeque, ops::Range};

fn main() {
    let input = util::read_input("day5/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut parts = input.split("\n\n").filter(|s| !s.is_empty());

    let fresh = parse_fresh(parts.next().unwrap());
    let ingredients = parse_ingredients(parts.next().unwrap());

    let fresh_count = ingredients
        .iter()
        .filter(|i| fresh.iter().any(|r| r.contains(i)))
        .count();

    format!("{}", fresh_count)
}

fn part2(input: &str) -> String {
    let mut parts = input.split("\n\n").filter(|s| !s.is_empty());

    let mut fresh = parse_fresh(parts.next().unwrap());

    fresh.sort_by(|a, b| a.start.cmp(&b.start));

    let mut curr = 0;
    let mut res = 0;

    for r in fresh {
        if curr > r.end {
            continue;
        }

        let start = max(curr, r.start);
        if start < r.end {
            res += r.end - start
        }

        curr = r.end;
    }

    format!("{}", res)
}

fn parse_fresh(s: &str) -> Vec<Range<u64>> {
    util::to_lines(s)
        .iter()
        .map(|s| {
            let mut parts = s.split("-");

            let first = parts.next().unwrap().parse::<u64>().unwrap();
            let last = parts.next().unwrap().parse::<u64>().unwrap();

            first..last + 1
        })
        .collect()
}

fn parse_ingredients(s: &str) -> Vec<u64> {
    util::to_lines(s)
        .iter()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "
    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
    ";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT), "3")
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(INPUT), "14")
    }
}
