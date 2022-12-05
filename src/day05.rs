use std::str::FromStr;

#[derive(Clone,Copy,Debug,PartialEq)]
struct Move {
    amount: u32,
    from: usize,
    to: usize
}

impl std::str::FromStr for Move {
    type Err = std::num::ParseIntError;

    //move 1 from 2 to 3
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.trim().split(" ").collect();
        return Result::Ok(Move {
            amount: words[1].parse::<u32>()?,
            from: words[3].parse::<usize>()?,
            to: words[5].parse::<usize>()?
        });
    }
}

pub fn solve() {
    println!("*** Day 05 ***");

    let s = std::fs::read_to_string("input/day05/input.txt").expect("peut");
    let (stacks, moves) = parse_input(&s);

    println!("stacks: {}, moves: {}", stacks.len(), moves.len());
    part1(&stacks, &moves);
    part2(&stacks, &moves);
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut len = 0;
    for line in input.lines() {
        if !line.contains("[") {
            break;
        }
        len = std::cmp::max(line.len(), len);
    }
    let n = (len + 1)/4;
    let mut stacks: Vec<Vec<char>> = (0..n).map(|_| Vec::new()).collect();
    for line in input.lines() {
        if !line.contains("[") {
            break;
        }
        for i in 0..n {
            let j = (i * 4) + 1;
            if j >= line.len() {
                break;
            }
            match line.chars().nth(j) {
                Some(c) => match c { 'A'..='Z' => stacks[i].push(dbg!(c)), _ => {}},
                _ => {}
            };
        }        
    }

    let moves = input.lines()
        .filter(|line| line.contains("move"))
        .map(&str::trim)
        .map(Move::from_str)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();

    (stacks, moves)
}

fn part1(stacks: &Vec<Vec<char>>, moves: &Vec<Move>) {
    println!("* Part 1 *");
    let count = 0;
    println!("Count: {}", count);
}

fn part2(stacks: &Vec<Vec<char>>, moves: &Vec<Move>) {
    println!("* Part 2 *");
    let count = 0;
    println!("Count: {}", count);
}
