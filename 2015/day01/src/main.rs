const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT).unwrap());
}

fn part1(input: &str) -> i32 {
    floors(input).last().unwrap()
}

fn part2(input: &str) -> Option<usize> {
    floors(input).position(|floor| floor == -1).map(|i| i + 1)
}

fn floors(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.chars().scan(0, |floor, c| {
        *floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        Some(*floor)
    })
}
