use std::collections::BTreeMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Polynomial(Vec<i64>);

impl Polynomial {
    fn deg(&self) -> usize {
        self.0.len() - 1
    }

    fn gcd(&self) -> i64 {
        self.0.iter().fold(0u64, |g, &x| {
            if x == 0 {
                g
            } else {
                gcd::binary_u64(g, x.abs() as u64)
            }
        }) as i64
    }
}

impl From<i64> for Polynomial {
    fn from(value: i64) -> Self {
        Self(vec![value])
    }
}

impl std::ops::Add<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Self::Output {
        let l = usize::max(self.deg(), rhs.deg());
        let lhs = self.0.iter().chain(std::iter::repeat(&0));
        let rhs = rhs.0.iter().chain(std::iter::repeat(&0));
        let mut sum = Vec::with_capacity(l + 1);
        sum.resize(l + 1, 0);
        let entries = sum.iter_mut();
        for ((&x, &y), z) in lhs.zip(rhs).zip(entries) {
            *z = x + y;
        }
        sum = sum.into_iter().rev().skip_while(|&x| x == 0).collect();
        sum.reverse();
        if sum.is_empty() {
            sum.push(0)
        }
        Polynomial(sum)
    }
}

impl std::ops::Sub<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: &Polynomial) -> Self::Output {
        let l = usize::max(self.deg(), rhs.deg());
        let lhs = self.0.iter().chain(std::iter::repeat(&0));
        let rhs = rhs.0.iter().chain(std::iter::repeat(&0));
        let mut diff = Vec::with_capacity(l + 1);
        diff.resize(l + 1, 0);
        let entries = diff.iter_mut();
        for ((&x, &y), z) in lhs.zip(rhs).zip(entries) {
            *z = x - y;
        }
        diff = diff.into_iter().rev().skip_while(|&x| x == 0).collect();
        diff.reverse();
        if diff.is_empty() {
            diff.push(0)
        }
        Polynomial(diff)
    }
}

impl std::ops::Mul<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Self::Output {
        let l = self.deg() + rhs.deg();
        let mut mul = Vec::with_capacity(l + 1);
        mul.resize(l + 1, 0);
        for (d1, a1) in self.0.iter().enumerate() {
            for (d2, a2) in rhs.0.iter().enumerate() {
                *mul.get_mut(d1 + d2).unwrap() += a1 * a2;
            }
        }
        Polynomial(mul)
    }
}

#[derive(Debug)]
struct Rational(Polynomial, Polynomial);

impl Rational {
    fn simplify(self) -> Self {
        let mut num = self.0;
        let mut den = self.1;
        let g = num.gcd();
        let h = den.gcd();
        if g == 0 {
            if h != 0 {
                for x in den.0.iter_mut() {
                    *x /= h;
                }
            }
        } else {
            if h == 0 {
                for x in num.0.iter_mut() {
                    *x /= g;
                }
            } else {
                let d = gcd::binary_u64(g as u64, h as u64) as i64;
                for x in den.0.iter_mut() {
                    *x /= d;
                }
                for x in num.0.iter_mut() {
                    *x /= d;
                }
            }
        }
        Self(num, den)
    }
}

impl From<i64> for Rational {
    fn from(value: i64) -> Self {
        Self(Polynomial::from(value), Polynomial::from(1))
    }
}

impl std::ops::Add<&Rational> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        Rational(&(&self.0 * &rhs.1) + &(&self.1 * &rhs.0), &self.1 * &rhs.1).simplify()
    }
}

impl std::ops::Sub<&Rational> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        Rational(&(&self.0 * &rhs.1) - &(&self.1 * &rhs.0), &self.1 * &rhs.1).simplify()
    }
}

impl std::ops::Mul<&Rational> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        Rational(&self.0 * &rhs.0, &self.1 * &rhs.1).simplify()
    }
}

impl std::ops::Div<&Rational> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        Rational(&self.0 * &rhs.1, &self.1 * &rhs.0).simplify()
    }
}

enum Op<'a> {
    Literal(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

pub fn star_one() -> i64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut monkeys = BTreeMap::<&str, Op>::new();
    for line in values.lines() {
        let (name, line) = line.split_once(": ").unwrap();
        let op = match line.parse::<i64>() {
            Ok(literal) => Op::Literal(literal),
            Err(_) => {
                let mut tokens = line.split_whitespace();
                let op1 = tokens.next().unwrap();
                let sym = tokens.next().unwrap();
                let op2 = tokens.next().unwrap();
                match sym {
                    "+" => Op::Add(op1, op2),
                    "-" => Op::Sub(op1, op2),
                    "*" => Op::Mul(op1, op2),
                    "/" => Op::Div(op1, op2),
                    _ => unreachable!(),
                }
            }
        };
        monkeys.insert(name, op);
    }

    let mut queue = VecDeque::new();
    queue.push_back("root");
    while let Some(m) = queue.pop_front() {
        let op = monkeys.get(m).unwrap();
        let (m1, m2) = match op {
            Op::Add(m1, m2) | Op::Sub(m1, m2) | Op::Mul(m1, m2) | Op::Div(m1, m2) => (m1, m2),
            _ => continue,
        };
        if let Some(&Op::Literal(a)) = monkeys.get(m1) {
            if let Some(&Op::Literal(b)) = monkeys.get(m2) {
                let op = monkeys.get_mut(m).unwrap();
                match op {
                    Op::Add(_, _) => *op = Op::Literal(a + b),
                    Op::Sub(_, _) => *op = Op::Literal(a - b),
                    Op::Mul(_, _) => *op = Op::Literal(a * b),
                    Op::Div(_, _) => *op = Op::Literal(a / b),
                    _ => {
                        unreachable!()
                    }
                };
                continue;
            }
        }
        queue.push_back(m);
        queue.push_front(m2);
        queue.push_front(m1);
    }

    let Op::Literal(v) = monkeys.get("root").unwrap() else { panic!() };
    *v
}

pub fn star_two() -> i64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut monkeys = BTreeMap::<&str, Op>::new();
    for line in values.lines() {
        let (name, line) = line.split_once(": ").unwrap();
        let op = match line.parse::<i64>() {
            Ok(literal) => Op::Literal(literal),
            Err(_) => {
                let mut tokens = line.split_whitespace();
                let op1 = tokens.next().unwrap();
                let sym = tokens.next().unwrap();
                let op2 = tokens.next().unwrap();
                match sym {
                    "+" => Op::Add(op1, op2),
                    "-" => Op::Sub(op1, op2),
                    "*" => Op::Mul(op1, op2),
                    "/" => Op::Div(op1, op2),
                    _ => unreachable!(),
                }
            }
        };
        monkeys.insert(name, op);
    }

    let mut expressions = BTreeMap::<&str, Rational>::new();
    expressions.insert(
        "humn",
        Rational(Polynomial(vec![0, 1]), Polynomial(vec![1])),
    );

    let mut queue = VecDeque::new();
    match monkeys.get("root").unwrap() {
        Op::Add(m1, m2) | Op::Sub(m1, m2) | Op::Mul(m1, m2) | Op::Div(m1, m2) => {
            queue.push_back(*m1);
            queue.push_back(*m2);
        }
        _ => unreachable!(),
    };

    while let Some(m) = queue.pop_front() {
        if expressions.contains_key(m) {
            continue;
        }

        let op = monkeys.get(m).unwrap();
        let (m1, m2) = match op {
            Op::Add(m1, m2) => (m1, m2),
            Op::Sub(m1, m2) => (m1, m2),
            Op::Mul(m1, m2) => (m1, m2),
            Op::Div(m1, m2) => (m1, m2),
            Op::Literal(value) => {
                expressions.insert(m, Rational::from(*value));
                continue;
            }
        };
        if let Some(a) = expressions.get(m1) {
            if let Some(b) = expressions.get(m2) {
                let f = match op {
                    Op::Add(_, _) => a + b,
                    Op::Sub(_, _) => a - b,
                    Op::Mul(_, _) => a * b,
                    Op::Div(_, _) => a / b,
                    _ => {
                        unreachable!()
                    }
                };
                expressions.insert(m, f);
                continue;
            }
        }
        queue.push_back(m);
        queue.push_front(m2);
        queue.push_front(m1);
    }

    let (m1, m2) = match monkeys.get("root").unwrap() {
        Op::Add(m1, m2) | Op::Sub(m1, m2) | Op::Mul(m1, m2) | Op::Div(m1, m2) => (m1, m2),
        _ => unreachable!(),
    };
    let m1 = expressions.get(m1).unwrap();
    let m2 = expressions.get(m2).unwrap();

    let p = &m1.0 * &m2.1;
    let q = &m2.0 * &m1.1;
    let f = &p - &q;

    (-f.0[0]) / (f.0[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 152);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 301);
    }
}
