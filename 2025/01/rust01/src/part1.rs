pub fn process(input: &[u8]) -> usize {
    let it = input
        .split(u8::is_ascii_whitespace)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let sign = if line[0] == b'L' { -1 } else { 1 };
            let amount: isize = line[1..]
                .iter()
                .rev()
                .enumerate()
                .map(|(pos, n)| ((n - 48) as isize) * 10_isize.pow(pos.try_into().unwrap()))
                .sum();
            amount * sign
        });

    let mut current = 50;
    let mut found = 0;

    for amount in it {
        current = (current + amount).rem_euclid(100);

        if current == 0 {
            found += 1;
        }
    }

    found
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 3);
    }
}
