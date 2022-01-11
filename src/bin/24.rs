use itertools::Itertools;

fn main() {
    let packages: Vec<u64> = include_str!("24.input").lines().map(|n| n.parse().unwrap()).collect();
    let weight = packages.iter().sum::<u64>();

    let qe = |sections: u64| (2..)
        .find_map(|i| packages
            .iter()
            .combinations(i)
            .filter(|cs| cs.iter().copied().sum::<u64>() == weight / sections)
            .map(|cs| cs.into_iter().product::<u64>())
            .min()
        )
        .unwrap();

    println!("Part 1: {}", qe(3));
    println!("Part 2: {}", qe(4));
}
