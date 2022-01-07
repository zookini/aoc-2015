use itertools::Itertools;
use regex::{Captures, Regex};

fn main() {
    let (replacements, molecule) = include_str!("19.input")
        .split_once("\r\n\r\n")
        .map(|(replacements, molecule)| (
            replacements.lines().map(|line| line.split_once(" => ").unwrap()),
            molecule.as_bytes()
        ))
        .unwrap();

    let rules = replacements.clone().map(|(k, v)| (k.as_bytes(), v.as_bytes())).into_group_map();

    let p1 = (0..molecule.len())
        .filter_map(|i| (i + 1..=molecule.len().min(i + 2))
            .find_map(|j| rules.get(&molecule[i..j]).map(|expansions| (i, j, expansions.iter())))
        )
        .flat_map(|(i, j, expansions)| expansions
            .map(move |&expansion| molecule[..i].iter().chain(expansion).chain(&molecule[j..]).collect::<Vec<_>>())
        );

    println!("Part 1: {}", p1.unique().count());

    let rules: std::collections::HashMap<String, String> = replacements
        .map(|(k, v)| (v.chars().rev().collect(), k.chars().rev().collect()))
        .collect();

    let subs = Regex::new(&rules.keys().join("|")).unwrap();
    let molecule: String = molecule.iter().rev().map(|&b| b as char).collect();
    
    let p2 = std::iter::successors(Some(molecule), |m| Some(subs.replace(&m, |r: &Captures| &rules[&r[0]]).into()))
        .enumerate()
        .find_map(|(i, molecule)| (molecule == "e").then(|| i));

    println!("Part 2: {}", p2.unwrap());
}
