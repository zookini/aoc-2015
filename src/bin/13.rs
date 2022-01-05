use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let happiness: HashMap<_, isize> = Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+)")
        .unwrap()
        .captures_iter(include_str!("13.input"))
        .map(|r| (r.get(1).unwrap().as_str(), r.get(4).unwrap().as_str(), &r[2] == "gain", r[3].parse::<isize>().unwrap()))
        .map(|(a, b, positive, units)| ((a, b), if positive { units } else { -units }))
        .collect();

    let mut people: HashSet<&str> = happiness.keys().map(|p| p.0).collect();

    let happiest = |people: &HashSet<&str>| people
        .iter()
        .permutations(people.len())
        .map(|mut perm| {
            perm.push(perm[0]);
            
            perm.windows(2)
                .map(|w| happiness.get(&(*w[0], *w[1])).unwrap_or(&0) + happiness.get(&(*w[1], *w[0])).unwrap_or(&0))
                .sum::<isize>()
        })
        .max()
        .unwrap();

    println!("Part 1: {}", happiest(&people));

    people.insert("me");
    
    println!("Part 2: {}", happiest(&people));
}
