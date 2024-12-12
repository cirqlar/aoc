pub mod part1;
pub mod part2;

#[cfg(target_os = "windows")]
pub const NEWLINE_OFFSET: usize = 2;
#[cfg(not(target_os = "windows"))]
pub const NEWLINE_OFFSET: usize = 1;

pub fn num_digits(mut num: usize) -> u32 {
    let mut n = 1;

    if num >= 100000000 {
        n += 8;
        num /= 100000000;
    }
    if num >= 10000 {
        n += 4;
        num /= 10000;
    }
    if num >= 100 {
        n += 2;
        num /= 100;
    }
    if num >= 10 {
        n += 1;
    }

    n
}

pub fn num_digits_v2(num: u64) -> u32 {
    let mut n = 1;
    if num > 10 {
        n += 1;
    }
    if num > 100 {
        n += 1;
    }
    if num > 1000 {
        n += 1;
    }
    if num > 10000 {
        n += 1;
    }
    if num > 100000 {
        n += 1;
    }

    n
}