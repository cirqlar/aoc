use itertools::Itertools;
use pathfinding::prelude::bfs;
use rustc_hash::FxHashSet;
use std::str::from_utf8;

pub fn process(input: &[u8], grid_size: usize, count: usize) -> usize {
    let mut mem_space = vec![b'.'; (grid_size + 1).pow(2)];

    input
        .split(|ch| ch.is_ascii_whitespace() || ch == &b',')
        .filter(|n| !n.is_empty())
        .tuples()
        .take(count)
        .for_each(|(x, y)| {
            let x = from_utf8(x).expect("x").parse::<usize>().expect("x");
            let y = from_utf8(y).expect("y").parse::<usize>().expect("y");

            let index = y * (grid_size + 1) + x;

            mem_space[index] = b'#';
        });

    // for y in 0..=grid_size {
    //     for x in 0..=grid_size {
    //         let index = y * (grid_size + 1) + x;
    //         print!("{}", mem_space[index] as char);
    //     }
    //     println!();
    // }

    let res = bfs(
        &0,
        |n| {
            let mut possibles = Vec::with_capacity(4);

            let nx = *n % (grid_size + 1);
            let ny = *n / (grid_size + 1);

            if ny > 0 {
                let possible = *n - (grid_size + 1);
                if mem_space[possible] != b'#' {
                    possibles.push(possible);
                }
            }
            if ny < grid_size {
                let possible = *n + (grid_size + 1);
                if mem_space[possible] != b'#' {
                    possibles.push(possible);
                }
            }
            if nx > 0 {
                let possible = *n - 1;
                if mem_space[possible] != b'#' {
                    possibles.push(possible);
                }
            }
            if nx < grid_size {
                let possible = *n + 1;
                if mem_space[possible] != b'#' {
                    possibles.push(possible);
                }
            }

            // println!("From {}/{},{} the possibles are {:?}", n, nx, ny, possibles);

            possibles
        },
        |n| *n == mem_space.len() - 1,
    )
    .expect("A path");

    res.len() - 1
}

pub fn process_map(input: &[u8], grid_size: usize, count: usize) -> usize {
    let map = input
        .split(|ch| ch.is_ascii_whitespace() || ch == &b',')
        .filter(|n| !n.is_empty())
        .tuples()
        .take(count)
        .map(|(x, y)| {
            let x = from_utf8(x).expect("x").parse::<usize>().expect("x");
            let y = from_utf8(y).expect("y").parse::<usize>().expect("y");

            y * (grid_size + 1) + x
        })
        .collect::<FxHashSet<_>>();

    // for y in 0..=grid_size {
    //     for x in 0..=grid_size {
    //         let index = y * (grid_size + 1) + x;
    //         print!("{}", mem_space[index] as char);
    //     }
    //     println!();
    // }

    let res = bfs(
        &0,
        |n| {
            let mut possibles = Vec::with_capacity(4);

            let nx = *n % (grid_size + 1);
            let ny = *n / (grid_size + 1);

            if ny > 0 {
                let possible = *n - (grid_size + 1);
                if !map.contains(&possible) {
                    possibles.push(possible);
                }
            }
            if ny < grid_size {
                let possible = *n + (grid_size + 1);
                if !map.contains(&possible) {
                    possibles.push(possible);
                }
            }
            if nx > 0 {
                let possible = *n - 1;
                if !map.contains(&possible) {
                    possibles.push(possible);
                }
            }
            if nx < grid_size {
                let possible = *n + 1;
                if !map.contains(&possible) {
                    possibles.push(possible);
                }
            }

            // println!("From {}/{},{} the possibles are {:?}", n, nx, ny, possibles);

            possibles
        },
        |n| *n == (grid_size + 1).pow(2) - 1,
    )
    .expect("A path");

    res.len() - 1
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");
        let grid_size = 6;
        let count = 12;

        let answer = super::process(input, grid_size, count);

        assert_eq!(answer, 22);
    }
}
