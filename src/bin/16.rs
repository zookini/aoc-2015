type Traits<'a> = std::collections::HashMap<&'a str, usize>;

fn main() {
    let mfcsam = Traits::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
		("vizslas", 0),
		("goldfish", 5),
		("trees", 3),
		("cars", 2),
		("perfumes", 1),
    ]);

    let traits: Vec<_> = include_str!("16.input")
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|traits| traits
            .split(", ")
            .map(|t| t.split_once(": ").unwrap())
            .map(|(k, v)| (k, v.parse().unwrap()))
            .collect::<Traits>()
        )
        .collect();

    println!("Part 1: {}", traits.iter().position(|t| t.iter().all(|(k, &v)| mfcsam[k] == v)).unwrap() + 1);

    let p2 = traits
        .iter()
        .position(|t| t.iter().all(|(&k, &v)| match k {
            "cats" | "trees" => v > mfcsam[k],
            "pomerians" | "goldfish" => v < mfcsam[k],
            _ => v == mfcsam[k]
        }))
        .unwrap() + 1;

    println!("Part 2: {}", p2);
}
