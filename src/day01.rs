#[derive(Clone, Debug)]
struct Elf {
    name: String,
    calories: Vec<i32>
}

impl Elf {
    fn total(&self) -> i32 {
        self.calories.iter().sum()
    }
}

pub fn solve() {
    println!("*** Day 01 ***");

    let input = std::fs::read_to_string("input/day01/input.txt").expect("peut");

    let mut calories: Vec<i32> = Vec::new();
    let mut elves: Vec<Elf> = Vec::new();
    for line in input.lines() {
        if line.is_empty() && !calories.is_empty() {
            elves.push(Elf{
                name: elves.len().to_string(),
                calories: calories.clone()
            });
            calories.clear();
        } else {
            calories.push(line.parse::<i32>().unwrap());
        }
    }
    if !calories.is_empty() {
        elves.push(Elf{
            name: elves.len().to_string(),
            calories: calories.clone()
        });
    }
    println!("Elves {}", elves.len());
    part1(&elves);
    part2(&elves, 3);
}

fn part1(elves: &Vec<Elf>) {
    println!("* Part 1 *");
    match elves.iter().max_by_key(|e| (*e).total()) {
        None => println!("No elves found."),
        Some(e) => println!("Name {}, calories: {}.", e.name, e.total())
    }
}

fn part2(elves: &Vec<Elf>, num: usize) {
    println!("* Part 2 *");
    let mut sorted_elves = elves.clone();
    sorted_elves.sort_by_cached_key(|e| (*e).total());
    if num < sorted_elves.len() {
        let total_num = sorted_elves[sorted_elves.len() - num..]
            .iter()
            .map(|e| (*e).total())
            .sum::<i32>();
        println!("Total {}: {}", num, total_num);
    }
}