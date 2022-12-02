#[derive(Debug, Clone, Copy, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn from(c: char) -> RPS {
        match c {
            'A' | 'X' => RPS::Rock,
            'B' | 'Y' => RPS::Paper,
            'C' | 'Z' => RPS::Scissors,
            _ => panic!("Unknown RPS char: {}", c)
        }
    }

    fn score(&self) -> i32 {
        (*self as i32) + 1
    }
}

fn score_part1(e: &(RPS, RPS)) -> i32 {
    let sa = e.0.score();
    let sb = e.1.score();

    if sa == sb {
        sb + 3
    } else if (sa % 3) + 1 == sb {
        sb + 6
    } else {
        sb
    }
}

fn score_part2(e: &(RPS, RPS)) -> i32 {
    let sa = e.0.score();
    match e.1 {
        RPS::Rock => ((sa + 1) % 3) + 1,
        RPS::Paper => sa + 3,
        RPS::Scissors => (sa % 3) + 7
    }
}

pub fn solve() {
    println!("*** Day 02 ***");

    let input = std::fs::read_to_string("input/day02/input.txt").expect("peut");

    let game = parse_input(&input);
    println!("Rounds: {}", game.len());
    part1(&game);
    part2(&game);
}

fn parse_input(input: &str) -> Vec<(RPS, RPS)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let tline = line.trim();
        if tline.len() > 2 {
            result.push(
                (RPS::from(tline.chars().nth(0).unwrap()), 
                RPS::from(tline.chars().nth(2).unwrap())));
        }
    }
    result
}

fn part1(game: &Vec<(RPS,RPS)>) {
    println!("* Part 1 *");
    let score = game.iter()
        .map(score_part1)
        .sum::<i32>();
    println!("Score: {}", score);
}

fn part2(game: &Vec<(RPS,RPS)>) {
    println!("* Part 2 *");
    let score = game.iter()
        .map(score_part2)
        .sum::<i32>();
    println!("Score: {}", score);
}