use std::collections::HashSet;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input: u32 = include_str!("day3.prod")
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(comp1, comp2)| {
            let set1: HashSet<char> = comp1.chars().collect();
            let set2: HashSet<char> = comp2.chars().collect();

            set1.intersection(&set2).next().unwrap().clone()
        })
        .map(|c| char_to_int(&c))
        .sum();
    println!("{}", input);
}

fn part_two() {
    let mut lines = include_str!("day3.prod").lines().peekable();
    let mut answer = 0;
    while lines.peek().is_some() {
        let mut chunk: Vec<&str> = lines.by_ref().take(3).collect();
        let set1: HashSet<char> = chunk.pop().unwrap().chars().collect();
        let set2: HashSet<char> = chunk.pop().unwrap().chars().collect();
        let set3: HashSet<char> = chunk.pop().unwrap().chars().collect();

        let common = set1
            .iter()
            .filter(|c| set2.contains(c) && set3.contains(c))
            .map(|c| char_to_int(c))
            .next()
            .unwrap();

        answer += common;
    }
    println!("{}", answer);
}

fn char_to_int(c: &char) -> u32 {
    if c.is_lowercase() {
        c.to_digit(36).unwrap() - 9
    } else {
        c.to_digit(36).unwrap() - 9 + 26
    }
}
