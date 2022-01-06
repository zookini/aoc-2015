fn main() {
    let ingredients: Vec<Vec<isize>> = include_str!("15.input")
        .lines()
        .map(|s| s.split(|c| ", ".contains(c)).flat_map(|n| n.parse().ok()).collect())
        .collect();

    let [p1, p2] = highest(&ingredients, 100, &mut vec![0; ingredients[0].len()], [0, 0]);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn highest(ingredients: &[Vec<isize>], remaining: isize, properties: &mut Vec<isize>, [p1, p2]: [isize; 2]) -> [isize; 2] {
    if ingredients.is_empty() {
        let product = properties.iter().take(4).map(|n| n.max(&0)).product();
        [p1.max(product), if properties[properties.len() - 1] == 500 { p2.max(product) } else { p2 }]
    } else {
        let scale = if ingredients.len() == 1 { remaining } else { 1 };
        
        let max = (scale..=remaining)
            .map(|i| {
                properties.iter_mut().zip(&ingredients[0]).for_each(|(p, i)| *p += i * scale);
                highest(&ingredients[1..], remaining - i, properties, [p1, p2])
            })
            .fold([0, 0], |[max1, max2], [p1, p2]| [p1.max(max1), p2.max(max2)]);

        properties.iter_mut().zip(&ingredients[0]).for_each(|(p, i)| *p -= i * remaining);
        max
    }
}
