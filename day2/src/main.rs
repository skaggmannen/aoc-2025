use std::fmt::Debug;

fn main() {
    let input = util::read_input("day2/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let result: u64 = input
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| Range::from_str(s.trim()))
        .flat_map(|r| r.invalid_ids())
        .sum();

    format!("{}", result)
}

fn part2(input: &str) -> String {
    let result: u64 = input
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| Range::from_str(s.trim()))
        .flat_map(|r| r.invalid_ids_part2())
        .sum();

    format!("{}", result)
}

struct Range(u64, u64);

impl Range {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split("-");

        let first = parts.next().unwrap().parse().unwrap();
        let last = parts.next().unwrap().parse().unwrap();

        Self(first, last)
    }

    fn invalid_ids(&self) -> Vec<u64> {
        let &Self(first, last) = self;

        (first..=last)
            .map(|i| format!("{}", i))
            .filter(|s| s.len() % 2 == 0)
            .filter(|s| {
                let start = &s[..s.len() / 2];
                let end = &s[s.len() / 2..];

                return start == end;
            })
            .map(|s| s.parse::<u64>().unwrap())
            .collect()
    }

    fn invalid_ids_part2(&self) -> Vec<u64> {
        let &Self(first, last) = self;

        (first..=last)
            .map(|i| format!("{}", i))
            .filter(|s| {
                let bytes = s.as_bytes();

                for i in 1..=bytes.len() / 2 {
                    if bytes.len() % i != 0 {
                        continue;
                    }

                    let chunks = bytes
                        .chunks(i)
                        .map(|b| String::from_utf8_lossy(b))
                        .collect::<Vec<_>>();
                    if all_equal(&chunks) {
                        return true;
                    }
                }

                false
            })
            .map(|s| s.parse::<u64>().unwrap())
            .collect()
    }
}

fn all_equal<T: PartialEq + Debug>(i: &[T]) -> bool {
    let [head, tail @ ..] = i else {
        return false;
    };

    tail.iter().all(|x| x == head)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT), "1227775554");
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(INPUT), "4174379265");
    }
}
