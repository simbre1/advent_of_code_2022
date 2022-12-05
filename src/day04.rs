
pub fn solve() {
    println!("*** Day 04 ***");

    let s = std::fs::read_to_string("input/day04/input.txt").expect("peut");
    let input = parse_input(&s);
    part1(&input);
    part2(&input);
}

fn parse_input(input: &str) -> Vec<Vec<(u32,u32)>> {
    let result: Vec<Vec<(u32,u32)>> = input.lines()
        .map(|line| line.trim().split(",").map(to_range).collect())
        .collect();
    return result;
}

fn to_range(s: &str) -> (u32,u32) {
    let mut ab = s.split("-");
    return (
        ab.next().unwrap().parse::<u32>().unwrap(), 
        ab.next().unwrap().parse::<u32>().unwrap()
    )
}

fn part1(input: &Vec<Vec<(u32,u32)>>) {
    println!("* Part 1 *");
    let count = input.iter().filter(|e| part1_contains(e)).count();
    println!("Count: {}", count);
}

fn part1_contains(e: &Vec<(u32,u32)>) -> bool {
    let from = e.iter().map(|r| r.0).min().unwrap();
    let to = e.iter().map(|r| r.1).max().unwrap();
    e.contains(&(from,to))
}

fn part2(input: &Vec<Vec<(u32,u32)>>) {
    println!("* Part 2 *");
    let count = input.iter().filter(|e| part2_overlap(e)).count();
    println!("Count: {}", count);
}

fn part2_overlap(e: &Vec<(u32,u32)>) -> bool {
    !(e[0].1 < e[1].0 || e[1].1 < e[0].0)
}