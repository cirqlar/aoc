pub fn process(input: &str) -> usize {
    input
        .split(',')
        // .filter_map(|s| (!s.trim().is_empty()).then_some(s.trim()))
        .map(|s| {
            let dash_pos = s.find('-').expect("Has a dash");
            let dash_pos_u32: u32 = dash_pos.try_into().unwrap();

            let max_size = s.len() - dash_pos - 1;
            let max_size_u32: u32 = max_size.try_into().unwrap();

            let start_num = if dash_pos.is_multiple_of(2) {
                let first_num = s[0..dash_pos]
                    .parse::<usize>()
                    .expect("Parsable first half of first num");

                let first_part = first_num / 10_usize.pow(dash_pos_u32 / 2);
                let second_part = first_num % 10_usize.pow(dash_pos_u32 / 2);

                if second_part > first_part {
                    first_part + 1
                } else {
                    first_part
                }
            } else {
                let start = dash_pos + 1;
                let min_u: u32 = start.try_into().unwrap();
                10_usize.pow((min_u / 2) - 1)
            };

            let end_num = if max_size.is_multiple_of(2) {
                let second_num = s[(dash_pos + 1)..]
                    .parse::<usize>()
                    .expect("Parsable first half of first num");

                let first_part = second_num / 10_usize.pow(max_size_u32 / 2);
                let second_part = second_num % 10_usize.pow(max_size_u32 / 2);

                if second_part < first_part {
                    first_part - 1
                } else {
                    first_part
                }
            } else {
                let end = max_size - 1;
                let max_u: u32 = end.try_into().unwrap();
                10_usize.pow(max_u / 2) - 1
            };

            let start = if dash_pos_u32.is_multiple_of(2) {
                dash_pos_u32
            } else {
                dash_pos_u32 + 1
            };

            // println!("For {s} we have {start_num} and {end_num} with {start}");
            if start_num <= end_num {
                (start_num..=end_num).fold(0, |acc, x| acc + (x * 10_usize.pow(start / 2)) + x)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_str!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 1227775554);
    }
}
