use std::ops::AddAssign;
use std::str::FromStr;

enum Turn {
    Left(i32),
    Right(i32),
}

impl Turn {
    fn val(&self) -> i32 {
        return match self {
            Turn::Left(i) => -*i,
            Turn::Right(i) => *i,
        };
    }
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first = chars.nth(0).expect("??");
        let rest = chars.collect::<String>().parse::<i32>().unwrap();
        return Ok(match first {
            'R' => Turn::Right(rest),
            'L' => Turn::Left(rest),
            c => panic!("{}", c),
        });
    }
}

// Always in [0, 99]
struct DialArrow(i32);

impl AddAssign<&Turn> for DialArrow {
    fn add_assign(&mut self, rhs: &Turn) {
        self.0 += rhs.val();
        self.0 = self.0.rem_euclid(100);
    }
}

pub fn run() {
    let mut arrow = DialArrow(50);
    let input: Vec<Turn> = include_str!("./day1-input.txt")
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let mut zeros = 0;
    let mut method0x434c49434b = 0;
    for nxt in input.iter() {
        method0x434c49434b += match nxt {
            Turn::Left(_) => {
                (arrow.0 - 1).div_euclid(100) - (arrow.0 + nxt.val() - 1).div_euclid(100)
            },
            Turn::Right(_) => (arrow.0 + nxt.val()).div_euclid(100) - arrow.0.div_euclid(100),
        };

        arrow += nxt;
        if arrow.0 == 0 {
            zeros += 1;
        }
    }

    println!("Zeros: {}", zeros);
    println!("Zero Clicks: {}", method0x434c49434b);
}
