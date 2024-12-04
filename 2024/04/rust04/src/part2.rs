use std::io::BufRead;

pub fn process(input: &[u8]) -> i32 {
    let width = input.lines().next().unwrap().unwrap().len();
    let height = input.lines().count();

    let input = input
        .iter()
        .filter_map(|ch| {
            if ch.is_ascii_whitespace() {
                None
            } else {
                Some(*ch)
            }
        })
        .collect::<Vec<_>>();

    input
        .iter()
        .enumerate()
        .filter(|(ind, ch)| {
            if **ch != b'A' {
                return false;
            }

            let x_coord = ind % width;
            let y_coord = ind / width;

            x_coord > 0
                && x_coord < width - 1
                && y_coord > 0
                && y_coord < height - 1
                && ((input[ind - width - 1] == b'M' && input[ind + width + 1] == b'S')
                    || (input[ind - width - 1] == b'S' && input[ind + width + 1] == b'M'))
                && ((input[ind - width + 1] == b'M' && input[ind + width - 1] == b'S')
                    || (input[ind - width + 1] == b'S' && input[ind + width - 1] == b'M'))
        })
        .count()
        .try_into()
        .unwrap()
}