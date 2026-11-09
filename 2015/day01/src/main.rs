const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let mut floor = 0;

    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }

    return floor;
}

fn part2(input: &str) -> usize {
    let mut floor = 0;

    for (i, c) in input.char_indices() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            return i + 1;
        }
    }

    unreachable!()
}
