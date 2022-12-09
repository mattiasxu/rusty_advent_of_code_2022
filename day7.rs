use std::{collections::HashMap, path::PathBuf};
fn main() {
    let input = include_str!("day7.prod");

    let mut current_path = PathBuf::new();
    let mut directories: HashMap<PathBuf, usize> = HashMap::new();

    for s in input.lines() {
        if s.contains("$ cd") {
            let to = s.split(" ").last().unwrap();
            if to == ".." {
                // cd ..
                current_path.pop();
            } else {
                // cd to new dir, make hashmap entry
                current_path.push(s.split(" ").last().unwrap());
                directories.insert(current_path.clone(), 0);
            }
        } else if s.contains("$ ls") || s.contains("dir ") {
            continue;
        } else {
            // Add file size to current dir, and all parent dirs
            let size: usize = s.split(" ").next().unwrap().parse().unwrap();
            *directories.get_mut(&current_path).unwrap() += size;

            let mut walk_back = current_path.parent();
            while let Some(parent) = walk_back {
                *directories.get_mut(parent).unwrap() += size;
                walk_back = parent.parent();
            }
        }
    }

    // Part 1
    let mut ans = 0;
    for value in directories.values() {
        if *value <= 100000 {
            ans += *value;
        }
    }
    println!("{}", ans);

    // Part 2
    let needed_space =
        30000000 - (70000000 - directories.get(&PathBuf::from("/")).unwrap().clone());

    ans = usize::MAX;
    for value in directories.values() {
        if *value >= needed_space {
            ans = ans.min(*value);
        }
    }
    println!("{}", ans);
}
