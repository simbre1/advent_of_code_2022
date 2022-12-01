#[derive(Debug)]
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
    println!("day01");

    let input = std::fs::read_to_string("input/day01/input.txt").expect("peut");
    println!("{}", input);

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
}

fn part1(elves: &Vec<Elf>) {
    match elves.iter().max_by_key(|e| (*e).total()) {
        None => println!("No elves found."),
        Some(e) => println!("name {}, colories: {}", e.name, e.total())
    }
}