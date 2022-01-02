use itertools::Itertools;

fn main() {
    let dimensions = include_str!("02.input")
        .lines()
        .map(|line| line.split('x').map(|n| n.parse::<usize>().unwrap()).collect_tuple().unwrap());

    let p1: usize = dimensions
        .clone()
        .map(|(l, w, h)| [l * w, w * h, h * l])
        .map(|areas| areas.iter().map(|n| 2 * n).sum::<usize>() + areas.iter().min().unwrap())
        .sum();

    println!("Part 1: {}", p1);

    let p2: usize = dimensions
        .map(|(l, w, h)| [l, w, h].iter().sorted().take(2).sum::<usize>() * 2 + (l * w * h))
        .sum();

    println!("Part 2: {}", p2);
}
