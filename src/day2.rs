use std::str::FromStr;

pub struct IdRange(u64, u64);

struct IdRangeIter {
    twos_only: bool,
    end_num: u64,
    cur_num: u64,
}

impl IdRange {
    fn iter(&self, twos_only: bool) -> IdRangeIter {
        return IdRangeIter {
            twos_only,
            end_num: self.1,
            cur_num: self.0,
        };
    }
}

impl Iterator for IdRangeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_num <= self.end_num {
            let nxt_str = format!("{}", self.cur_num);
            let bytes = nxt_str.as_bytes();
            if bytes.len() == 1 {
                self.cur_num += 1;
                continue;
            }
            let start = if self.twos_only {
                nxt_str.len().div_ceil(2)
            } else {
                1
            };

            for i in start..(nxt_str.len().div_ceil(2)) + 1 {
                // If we can't divide go next
                if (nxt_str.len() % i) != 0 {
                    continue;
                }

                let mut chunks = bytes.chunks_exact(i);
                let fst = chunks.next().unwrap();
                // Preceeding zero
                if fst[0] == '0' as u8 {
                    continue;
                }

                if chunks.any(|x| x != fst) {
                    continue;
                }

                self.cur_num += 1;
                return Some(self.cur_num - 1);
            }
            self.cur_num += 1;
        }

        return None;
    }
}

impl FromStr for IdRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('-').unwrap();
        return Ok(IdRange(left.parse().unwrap(), right.parse().unwrap()));
    }
}

pub fn run_challenge(ranges: &Vec<IdRange>, two_only: bool) -> u64 {
    let mut total = 0;
    for range in ranges.iter() {
        total += range.iter(two_only).sum::<u64>();
    }
    return total;
}

pub fn run() {
    let ranges: Vec<IdRange> = include_str!("./day2-input.txt")
        .trim_end()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let total1 = run_challenge(&ranges, true);
    let total2 = run_challenge(&ranges, false);

    println!("Total1 {}", total1);
    println!("Total2 {}", total2);
}
