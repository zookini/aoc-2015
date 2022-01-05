use itertools::Itertools;

fn main() {
    let routes: std::collections::HashMap<[&str; 2], usize> = regex::Regex::new(r"(\w+) to (\w+) = (\d+)")
        .unwrap()
        .captures_iter(include_str!("09.input"))
        .map(|r| ([r.get(1).unwrap().as_str(), r.get(2).unwrap().as_str()], r[3].parse().unwrap()))
        .collect();

    let places: std::collections::HashSet<_> = routes.keys().flatten().collect();    

    if let itertools::MinMaxResult::MinMax(shortest, longest) = places
        .iter()
        .permutations(places.len())
        .map(|permutation| permutation
            .windows(2)
            .map(|w| routes.get(&[**w[0], **w[1]]).or_else(|| routes.get(&[**w[1], **w[0]])).unwrap())
            .sum::<usize>()
        )
        .minmax()
    {
        println!("Part 1: {:?}", shortest);
        println!("Part 2: {:?}", longest);
    }
}
