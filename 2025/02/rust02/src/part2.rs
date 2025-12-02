use std::collections::HashSet;

fn get_factors(num: usize) -> Vec<usize> {
    match num {
        1 => vec![],
        2 => vec![1],
        3 => vec![1],
        4 => vec![1, 2],
        5 => vec![1],
        6 => vec![1, 2, 3],
        7 => vec![1],
        8 => vec![1, 2, 4],
        9 => vec![1, 3],
        10 => vec![1, 2, 5],
        11 => vec![1],
        x => panic!("Unhandled number {x}"),
    }
}

pub fn process(input: &str) -> usize {
    let mut global_set = HashSet::new();

    input.split(',').for_each(|s| {
        let dash_pos = s.find('-').expect("Has a dash");

        let max_size = s.len() - dash_pos - 1;

        (dash_pos..=max_size).for_each(|len| {
            let factors = get_factors(len);

            if factors.is_empty() {
                return;
            }

            let len_u32: u32 = len.try_into().unwrap();
            let bottom = if len == dash_pos {
                s[0..dash_pos]
                    .parse::<usize>()
                    .expect("Parsable first half of first num")
            } else {
                10_usize.pow(len_u32 - 1)
            };
            let top = if len == max_size {
                s[(dash_pos + 1)..]
                    .parse::<usize>()
                    .expect("Parsable first half of first num")
            } else {
                10_usize.pow(len_u32) - 1
            };

            for factor in factors {
                let factor_u32 = u32::try_from(factor).unwrap();

                let number_of: u32 = (len / factor).try_into().unwrap();

                let mut default_min = bottom / 10_usize.pow((number_of - 1) * factor_u32);
                for n in (0..number_of - 1).rev() {
                    let current = ((bottom - (bottom % 10_usize.pow(n * factor_u32)))
                        - (bottom - (bottom % 10_usize.pow((n + 1) * factor_u32))))
                        / 10_usize.pow(n * factor_u32);

                    if current < default_min {
                        break;
                    }

                    if current > default_min {
                        default_min += 1;
                        break;
                    }
                }
                if default_min > 10_usize.pow(factor_u32) - 1 {
                    continue;
                }

                let mut default_max = top / 10_usize.pow((number_of - 1) * factor_u32);
                for n in (0..number_of - 1).rev() {
                    let current = ((top - (top % 10_usize.pow(n * factor_u32)))
                        - (top - (top % 10_usize.pow((n + 1) * factor_u32))))
                        / 10_usize.pow(n * factor_u32);

                    if current > default_max {
                        break;
                    }

                    if current < default_max {
                        default_max -= 1;
                        break;
                    }
                }
                if default_max < 10_usize.pow(factor_u32 - 1) {
                    continue;
                }

                (default_min..=default_max).for_each(|n| {
                    global_set.insert(
                        (0..number_of)
                            .map(|t| n * 10_usize.pow(t * factor_u32))
                            .sum(),
                    );
                });
            }
        });
    });

    global_set.iter().sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_str!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 4174379265);
    }
}
