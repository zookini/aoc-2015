fn main() {
    let grid: Vec<Vec<bool>> = include_str!("18.input")
        .lines()
        .map(|line| line.chars().map(|light| light == '#').collect())
        .collect();

    for (part, grid) in (1..).zip([p1(grid.clone()), p2(grid)]) {
        println!("Part {}: {}", part, grid.iter().flatten().filter(|&&light| light).count());
    }
}

fn p1(grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    std::iter::successors(Some(grid), |grid| Some(step(&grid))).nth(100).unwrap()
}

fn p2(mut grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let (bottom, right) = (grid.len() - 1, grid[0].len() - 1);
    let mut i = 0;

    loop {
        for (x, y) in [(0, 0), (0, bottom), (right, bottom), (right, 0)] {
            grid[y][x] = true;
        }

        if i == 100 {
            return grid;
        }

        i += 1;

        grid = step(&grid);
    }
}

fn step(start: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut next = vec![vec![false; start[0].len()]; start.len()];

    for (x, y) in itertools::iproduct!(0..start[0].len(), 0..start.len()) {
        let lit = neighbours((x, y), start.len()).filter(|&(nx, ny)| start[ny][nx]).count();
        next[y][x] = lit == 3 || lit == 2 && start[y][x];
    }

    next
}

fn neighbours((x, y): (usize, usize), size: usize) -> impl Iterator<Item = (usize, usize)> {
    itertools::iproduct!(x.saturating_sub(1)..(x + 2), y.saturating_sub(1)..(y + 2))
        .filter(move |&(nx, ny)| (nx, ny) != (x, y) && nx < size && ny < size)
}
