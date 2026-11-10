const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT).unwrap());
}

fn part1(input: &str) -> i32 {
    input
        .chars()
        .fold(0, |acc, x| if x == '(' { acc + 1 } else { acc - 1 })
}

fn part2(input: &str) -> Option<usize> {
    let mut floor = 0;
    input
        .chars()
        .position(|c| {
            floor += if c == '(' { 1 } else { -1 };
            floor == -1
        })
        .map(|i| i + 1)
}
