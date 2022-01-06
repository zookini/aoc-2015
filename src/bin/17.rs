use itertools::Itertools;

fn main() {
    let containers: Vec<Vec<usize>> = include_str!("17.input")
        .lines()
        .map(|line| line.parse().unwrap())
        .powerset()
        .filter(|v| v.iter().sum::<usize>() == 150)
        .collect();

    println!("Part 1: {}", containers.len());

    let min = containers.iter().map(|v| v.len()).min().unwrap();

    println!("Part 2: {}", containers.iter().filter(|v| v.len() == min).count());
}
