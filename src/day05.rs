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
    let stack_lines: Vec<&str> = input.lines()
        .take_while(|line| line.contains("["))
        .inspect(|line| len = std::cmp::max(len, line.len()))
        .collect();

    let n = (len + 1)/4;
    let mut stacks: Vec<Vec<char>> = (0..n)
        .map(|_| Vec::new())
        .collect();
    for line in stack_lines {
        for i in 0..n {
            let j = (i * 4) + 1;
            if j < line.len() {
                match line.chars().nth(j) {
                    Some(c) => match c {
                        'A'..='Z' => stacks[i].insert(0, c),
                        _ => {}
                    },
                    _ => {}
                }
            }
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
    let mut result = stacks.clone();

    for m in moves {
        for _ in 0..m.amount {
            match result[m.from-1].pop() {
                Some(c) => result[m.to-1].push(c),
                _ => {}
            }
        }
    }

    let msg = stacks_to_string(&result);

    println!("* Part 1 *");
    println!("Msg: {}", msg);
}

fn part2(stacks: &Vec<Vec<char>>, moves: &Vec<Move>) {
    let mut result = stacks.clone();

    for m in moves {
        let len = result[m.from-1].len() as u32;
        for i in (len - m.amount)..len {
            let c = result[m.from-1][i as usize];
            result[m.to-1].push(c);
        }
        for _ in 0..m.amount {
            result[m.from-1].pop();
        }
    }

    let msg = stacks_to_string(&result);

    println!("* Part 2 *");
    println!("Msg: {}", msg);
}

fn stacks_to_string(stacks: &Vec<Vec<char>>) -> String {
    stacks.iter()
        .map(|s| s.last())
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect::<String>()
}