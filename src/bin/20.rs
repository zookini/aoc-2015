const INPUT: u32 = 33100000;

fn main() {
    println!("Part 1: {}", tally(INPUT, 10));
    println!("Part 2: {}", tally(50, 11));
}

fn tally(stops: u32, deliveries: u32) -> u32 {
    let mut houses = vec![0; (INPUT / deliveries / 2) as usize];

    (1..).find(|&elf| {
        for house in houses[elf..].iter_mut().step_by(elf).take(stops as usize) {
            *house += elf as u32 * deliveries;
        }

        houses[elf] >= INPUT
    })
    .unwrap() as u32
}
