use std::collections::VecDeque;

fn main() {
    let mut iter = include_str!("day5.prod").split("\n\n");

    let grid_str = iter.next().unwrap();
    let moves_str = iter.next().unwrap();

    let mut stacks = make_stacks(grid_str);
    println!("{:?}", stacks);

    let commands = moves_str.lines().map(|line| {
        line.split(" ")
            .into_iter()
            .filter(|s| s.chars().next().unwrap().is_digit(10))
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>()
    });

    for command in commands {
        move_part1(
            &mut stacks,
            command[0].try_into().unwrap(),
            command[1].try_into().unwrap(),
            command[2].try_into().unwrap(),
        )
    }

    println!("{}", get_answer(&mut stacks));
}

fn make_stacks(grid_str: &str) -> Vec<VecDeque<char>> {
    let width = (grid_str.lines().next().unwrap().len() + 1) / 4;

    let mut stacks = vec![VecDeque::new(); width];

    let mut str_iter = grid_str.lines().rev();
    str_iter.next();
    for line in str_iter {
        for (i, c) in line.chars().enumerate().filter(|(i, _)| i % 4 == 1) {
            if c != ' ' {
                stacks[i / 4].push_back(c);
            }
        }
    }
    stacks
}

fn move_part1(stacks: &mut Vec<VecDeque<char>>, times: usize, from: usize, to: usize) {
    for _ in 0..times {
        let c: char = stacks[from - 1].pop_back().unwrap();
        stacks[to - 1].push_back(c);
    }
}

fn move_part2(stacks: &mut Vec<VecDeque<char>>, times: usize, from: usize, to: usize) {
    let mut temp: VecDeque<char> = VecDeque::new();
    for _ in 0..times {
        temp.push_back(stacks[from - 1].pop_back().unwrap_or_default());
    }
    for _ in 0..times {
        stacks[to - 1].push_back(temp.pop_back().unwrap_or_default());
    }
}

fn get_answer(stacks: &mut Vec<VecDeque<char>>) -> String {
    let mut answer = String::from("");
    for stack in stacks {
        answer.push(stack.pop_back().unwrap_or_default());
    }
    answer
}
