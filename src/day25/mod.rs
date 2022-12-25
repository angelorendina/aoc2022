use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Digit {
    Equal,
    Dash,
    Zero,
    One,
    Two,
}

impl Digit {
    fn decimal(&self) -> i64 {
        match self {
            Digit::Equal => -2,
            Digit::Dash => -1,
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Digit::Equal => "=",
            Digit::Dash => "-",
            Digit::Zero => "0",
            Digit::One => "1",
            Digit::Two => "2",
        }
    }
}

#[derive(Clone)]
struct Snafu(Vec<Digit>);

impl Snafu {
    fn decimal(&self) -> i64 {
        let mut dec = 0;
        let mut fives = 1;
        for d in self.0.iter().rev() {
            dec += fives * d.decimal();
            fives *= 5;
        }
        dec
    }
}

impl From<i64> for Snafu {
    fn from(mut value: i64) -> Self {
        let mut digits = BTreeMap::<usize, Digit>::new();
        while value != 0 {
            let mut fives = 1;
            let mut n = 0;
            while fives <= 2 * value.abs() {
                fives *= 5;
                n += 1;
            }
            n -= 1;
            fives /= 5;
            let subpart = (fives - 1) / 2;
            let digit = if value <= subpart - 2 * fives {
                Digit::Equal
            } else if value <= subpart - fives {
                Digit::Dash
            } else if value <= subpart {
                Digit::Equal
            } else if value <= subpart + fives {
                Digit::One
            } else if value <= subpart + 2 * fives {
                Digit::Two
            } else {
                unreachable!()
            };
            value -= digit.decimal() * fives;
            digits.insert(n, digit);
        }
        digits.entry(0).or_insert(Digit::Zero);
        let (&m, _) = digits.last_key_value().unwrap();
        Snafu(
            (0..=m)
                .map(|n| *digits.get(&n).unwrap_or(&Digit::Zero))
                .collect(),
        )
    }
}

pub fn star_one() -> String {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut sum = 0;
    for line in values.lines() {
        let mut digits = Vec::<Digit>::new();
        for c in line.chars() {
            digits.push(match c {
                '=' => Digit::Equal,
                '-' => Digit::Dash,
                '0' => Digit::Zero,
                '1' => Digit::One,
                '2' => Digit::Two,
                _ => unreachable!(),
            });
        }
        sum += Snafu(digits).decimal();
    }
    let s = Snafu::from(sum);
    s.0.iter()
        .rev()
        .map(Digit::as_str)
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), "2=-1=0");
    }
}
