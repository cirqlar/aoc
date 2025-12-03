use rayon::{iter::ParallelIterator, slice::ParallelSlice};

pub fn process(input: &[u8]) -> usize {
    input
        .par_split(|b| b.is_ascii_whitespace())
        .map(|ba| {
            if ba.is_empty() {
                return 0;
            }

            let bat_size = 2;

            let mut last_top = 0;
            let mut total = 0;

            for pos in (0..bat_size).rev() {
                let (n_pos, n) = ba[last_top..ba.len() - pos]
                    .iter()
                    .enumerate()
                    .max_by(|(pos1, num1), (pos2, num2)| match num1.cmp(num2) {
                        std::cmp::Ordering::Equal => pos2.cmp(pos1),
                        x => x,
                    })
                    .expect("A max first value");

                last_top += n_pos + 1;
                total += ((*n - 48) as usize) * 10_usize.pow(u32::try_from(pos).unwrap());
            }

            total
        })
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 357);
    }
}
