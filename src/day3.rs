use std::mem::transmute;
use std::mem::MaybeUninit;

const NUM_BANKS: usize = 200;
const LINE_LENGTH: usize = 100;
type Bank = [u8; LINE_LENGTH];

fn asc<const DIGITS: usize>() -> [usize; DIGITS] {
    let mut out: [MaybeUninit<usize>; DIGITS] = [MaybeUninit::uninit(); DIGITS];
    (0..DIGITS).for_each(|i| {
        out[i].write(i);
    });
    return unsafe { out.transpose().assume_init() };
}

fn cascade(idxs: &mut [usize], fin: usize) {
    for i in 0..idxs.len() - 1 {
        idxs[i] = idxs[i + 1];
    }
    idxs[idxs.len() - 1] = fin;
}

fn calculate_jolts<const DIGITS: usize>(cur_bank: &Bank) -> usize {
    let mut best_digit_idxs = asc::<DIGITS>();
    'outer: for i in DIGITS..LINE_LENGTH {
        for bst in 0..DIGITS {
            if bst == DIGITS - 1 {
                if cur_bank[i] > cur_bank[best_digit_idxs[bst]] {
                    best_digit_idxs[bst] = i;
                    continue 'outer;
                }
            } else {
                if cur_bank[best_digit_idxs[bst + 1]] > cur_bank[best_digit_idxs[bst]] {
                    cascade(&mut best_digit_idxs[bst..], i);
                    continue 'outer;
                }
            }
        }
    }

    return collapse(&cur_bank, &best_digit_idxs);
}

fn collapse<const DIGITS: usize>(bank: &Bank, digits: &[usize; DIGITS]) -> usize {
    let mut sum = 0;
    for idx in 0..DIGITS {
        sum += 10usize.pow((DIGITS - idx - 1) as u32) * (bank[digits[idx]] as usize);
    }
    return sum;
}

pub fn run_challenge<const DIGITS: usize>(data: &[Bank; NUM_BANKS]) -> usize {
    let mut jolts = 0;
    for cur_bank in data {
        jolts += calculate_jolts::<DIGITS>(cur_bank);
    }

    return jolts;
}

pub fn run() {
    let banks = get_input();

    println!("Jolts (2 digit): {}", run_challenge::<2>(&banks));
    println!("Jolts (12 digit): {}", run_challenge::<12>(&banks));
}

pub fn get_input() -> [Bank; NUM_BANKS] {
    let mut banks: [[MaybeUninit<u8>; LINE_LENGTH]; NUM_BANKS] =
        [MaybeUninit::uninit().transpose(); NUM_BANKS];
    let mut lines = include_str!("./day3-input.txt").lines();
    for i in 0..banks.len() {
        populate(&mut banks[i], lines.next().unwrap());
    }

    return unsafe { transmute(banks) };
}

fn populate(batteries: &mut [MaybeUninit<u8>; LINE_LENGTH], line: &str) {
    let line_chars = line.as_bytes();
    for i in 0..LINE_LENGTH {
        batteries[i].write(unsafe { line_chars[i].unchecked_sub(48) });
    }
}
