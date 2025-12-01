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
        let added = current + amount;
        let mut zero_count = added.unsigned_abs() / 100;
        if added <= 0 && current > 0 {
            zero_count += 1;
        }

        current = (added).rem_euclid(100);
        found += zero_count;
    }

    found
}

#[cfg(test)]
mod test {
    #[test]
    fn test_2() {
        let input = include_bytes!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 6);
    }
}
