
pub fn solve() {
    println!("*** Day 03 ***");

    let input = std::fs::read_to_string("input/day03/input.txt").expect("peut");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    println!("* Part 1 *");
    let sum = input.lines()
        .map(&str::trim)
        .map(part1_rucksack)
        .sum::<u32>();
    println!("Score: {}", sum);
}

fn part1_rucksack(line: &str) -> u32 {
    let len = line.len();
    if line.len() < 2 {
        return 0;
    }
    let split = len/2;
    for c in line[..split].chars() {
        if line[split..].contains(c) {
            return char_score(c);
        }
    }
    0
}

fn char_score(c: char) -> u32 {
    let s = c as u32;
    match c {
        'a'..='z' => s - 96,
        'A'..='Z' => s - 38,
        _ => panic!("Wrong char: {}", c)
    }
}

fn part2(input: &str) {
    println!("* Part 2 *");
    let mut sum = 0u32;
    let mut i = 0 as usize;

    let lines: Vec<&str> = input.lines().map(&str::trim).collect();
    while i < lines.len()-1 {
        let j = std::cmp::min(i+3, lines.len());
        sum += part2_rucksacks(&lines[i..j]);
        i += 3;
    }
    println!("Score: {}", sum);
}

fn part2_rucksacks(input: &[&str]) -> u32 {
    for c in input[0].chars() {
        if input[1..].iter().all(|i| i.contains(c)) {
            return char_score(c);
        }
    }
    0
}
