use std::collections::HashSet;

fn main() {
    let input = util::read_input("day4/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let rolls = find_rolls(input);

    let accessible = rolls.iter().filter(|p| accessible(p, &rolls)).count();

    format!("{}", accessible)
}

fn part2(input: &str) -> String {
    let mut total_removed = 0;
    let mut rolls = find_rolls(input);

    loop {
        let can_be_removed: Vec<_> = rolls
            .iter()
            .filter_map(|p| {
                if accessible(p, &rolls) {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect();
        if can_be_removed.is_empty() {
            break;
        }

        total_removed += can_be_removed.len();

        for r in can_be_removed {
            rolls.remove(&r);
        }
    }

    format!("{}", total_removed)
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Pos(i32, i32);

fn find_rolls(input: &str) -> HashSet<Pos> {
    let mut rolls: HashSet<Pos> = HashSet::new();
    util::to_lines(input).iter().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c == '@' {
                rolls.insert(Pos(x as i32, y as i32));
            }
        });
    });
    rolls
}

fn accessible(Pos(x, y): &Pos, rolls: &HashSet<Pos>) -> bool {
    let mut adjacent = 0;
    for x2 in x - 1..=x + 1 {
        for y2 in y - 1..=y + 1 {
            if x2 == *x && y2 == *y {
                continue;
            }

            if rolls.contains(&Pos(x2, y2)) {
                adjacent += 1;
            }
        }
    }

    adjacent < 4
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.
    ";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(INPUT), "43")
    }
}
