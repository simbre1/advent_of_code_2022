pub fn solve() {
    println!("*** Day 05 ***");

    let s = std::fs::read_to_string("input/day06/input.txt").expect("peut");
 
    part1(&s);
    part2(&s);
}

fn part1(input: &str) {
    println!("* Part 1 *");
    let i = find_mark(4, input);
    println!("Index: {}", i+1);
}

fn part2(input: &str) {
    println!("* Part 2 *");
    let i = find_mark(14, input);
    println!("Index: {}", i+1);
}

fn find_mark(n: usize, input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = n-1;
    while i < len {
        let s = skip(&chars[i+1-n..i+1]);
        if s > 0 {
            i += s;
        } else if i < n {
            i += 1;
        } else {
            return i;
        }
    }
    len
}

fn skip(chars: &[char]) -> usize {
    for i in (1..chars.len()).rev() {
        for j in (0..i).rev() {
            if chars[i] == chars[j] {
                return j+1;
            }
        }
    }
    0
}