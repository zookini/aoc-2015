use itertools::Itertools;

const WEAPONS: [[i16; 3]; 5] = [[8, 4, 0], [10, 5, 0], [25, 6, 0], [40, 7, 0], [74, 8, 0]];
const ARMOR: [[i16; 3]; 6] = [[0, 0, 0], [13, 0, 1], [31, 0, 2], [53, 0, 3], [75, 0, 4], [102, 0, 5]];
const RINGS: [[i16; 3]; 7] = [[0, 0, 0], [25, 1, 0], [50, 2, 0], [100, 3, 0], [20, 0, 1], [40, 0, 2], [80, 0, 3]];

fn main() {    
    let rings = RINGS
        .iter()
        .tuple_combinations()
        .map(|(a, b)| [a[0] + b[0], a[1] + b[1], a[2] + b[2]])
        .chain([[0, 0, 0]]);

    let (win, lose): (Vec<_>, Vec<_>) = itertools::iproduct!(WEAPONS, ARMOR, rings)
        .map(|(w, a, r)| [w[0] + a[0] + r[0], w[1] + a[1] + r[1], w[2] + a[2] + r[2]])
        .partition(|&[_, damage, armor]| fight([100, damage, armor], [103, 9, 2]));
    
    println!("Part 1: {}", win.iter().map(|[cost, ..]| cost).min().unwrap());
    println!("Part 2: {}", lose.iter().map(|[cost, ..]| cost).max().unwrap());
}

fn fight(mut player: [i16; 3], mut boss: [i16; 3]) -> bool {
    while player[0] > 0 && boss[0] > 0 {
        boss[0] -= 1.max(player[1] - boss[2]);
        player[0] -= 1.max(boss[1] - player[2]);
    }

    boss[0] <= 0
}
