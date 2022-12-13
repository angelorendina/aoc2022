use std::cmp::Ordering;

#[derive(PartialEq, Eq, Ord)]
enum Value {
    Integer(u64),
    List(Vec<Box<Value>>),
}

impl Value {
    fn parse(value: &str) -> Self {
        if let Some(value) = value.strip_prefix('[') {
            let value = value.strip_suffix(']').unwrap();
            let mut items = Vec::<&str>::new();
            if !value.is_empty() {
                let mut depth = 0usize;
                let mut start = 0usize;
                for (end, c) in value.chars().enumerate() {
                    match c {
                        '[' => {
                            depth += 1;
                        }
                        ']' => {
                            depth -= 1;
                        }
                        ',' => {
                            if depth == 0 {
                                items.push(&value[start..end]);
                                start = end + 1;
                            }
                        }
                        _ => {}
                    }
                }
                items.push(&value[start..]);
            }

            Self::List(items.into_iter().map(Self::parse).map(Box::new).collect())
        } else {
            Self::Integer(value.parse::<u64>().unwrap())
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Integer(l), Value::Integer(r)) => Some(l.cmp(r)),
            (Value::Integer(l), Value::List(_)) => {
                Self::partial_cmp(&Value::List(vec![Box::new(Value::Integer(*l))]), other)
            }
            (Value::List(_), Value::Integer(r)) => {
                Self::partial_cmp(self, &Value::List(vec![Box::new(Value::Integer(*r))]))
            }
            (Value::List(l), Value::List(r)) => {
                let mut l = l.into_iter();
                let mut r = r.into_iter();
                Some(loop {
                    let left = l.next();
                    let right = r.next();
                    match (left, right) {
                        (None, None) => break Ordering::Equal,
                        (None, Some(_)) => break Ordering::Less,
                        (Some(_), None) => break Ordering::Greater,
                        (Some(l), Some(r)) => match Self::partial_cmp(l.as_ref(), r.as_ref()) {
                            Some(Ordering::Less) => break Ordering::Less,
                            Some(Ordering::Equal) => {}
                            Some(Ordering::Greater) => break Ordering::Greater,
                            None => unreachable!(),
                        },
                    }
                })
            }
        }
    }
}

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut lines = values.lines();
    let mut index = 1;
    let mut s = 0;
    loop {
        let Some(line) = lines.next() else { break };
        let left = Value::parse(line);
        let right = Value::parse(lines.next().unwrap());
        if left <= right {
            s += index;
        }
        index += 1;
        lines.next();
    }

    s
}

pub fn star_two() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut values = values
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(Value::parse(line))
            }
        })
        .collect::<Vec<_>>();
    values.push(Value::List(vec![Box::new(Value::List(vec![Box::new(
        Value::Integer(2),
    )]))]));
    values.push(Value::List(vec![Box::new(Value::List(vec![Box::new(
        Value::Integer(6),
    )]))]));

    values.sort();

    let start = Value::List(vec![Box::new(Value::List(vec![Box::new(Value::Integer(
        2,
    ))]))]);
    let end = Value::List(vec![Box::new(Value::List(vec![Box::new(Value::Integer(
        6,
    ))]))]);

    let mut start_index = 0;
    let mut end_index = 0;
    for (index, v) in values.into_iter().enumerate() {
        if v == start {
            start_index = index + 1;
        }
        if v == end {
            end_index = index + 1;
        }
    }

    start_index * end_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 13);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 140);
    }
}
