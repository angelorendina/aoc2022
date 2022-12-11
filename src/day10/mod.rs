pub fn star_one() -> i64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    enum Command {
        Noop,
        Addx { value: i64, elapsed: bool },
    }

    let mut signal = 0;
    let mut x = 1i64;
    let mut cycle = 1i64;
    let mut command = None;

    let mut lines = values.lines();
    loop {
        if cycle % 40 == 20 {
            signal += cycle * x;
        }
        if command.is_none() {
            let Some(line) = lines.next() else { break; };
            let mut tokens = line.split_whitespace();
            let token = tokens.next().expect("command should not be empty");
            match token {
                "addx" => {
                    let value = tokens
                        .next()
                        .expect("addx should have an operand")
                        .parse::<i64>()
                        .expect("operand should be a number");
                    command = Some(Command::Addx {
                        value,
                        elapsed: false,
                    });
                }
                "noop" => {
                    command = Some(Command::Noop);
                }
                _ => unreachable!("unrecognised command"),
            }
        }
        match command.take() {
            Some(cmd) => match cmd {
                Command::Noop => {}
                Command::Addx { value, elapsed } => {
                    if elapsed {
                        x += value;
                    } else {
                        command = Some(Command::Addx {
                            value,
                            elapsed: true,
                        });
                    }
                }
            },
            _ => unreachable!("command should have been loaded"),
        }
        cycle += 1;
    }

    signal
}

pub fn star_two() -> String {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    enum Command {
        Noop,
        Addx { value: i64, elapsed: bool },
    }

    let mut output = "\n".to_string();
    let mut x = 1i64;
    let mut cycle = 1i64;
    let mut command = None;

    let mut lines = values.lines();
    loop {
        if command.is_none() {
            let Some(line) = lines.next() else { break; };
            let mut tokens = line.split_whitespace();
            let token = tokens.next().expect("command should not be empty");
            match token {
                "addx" => {
                    let value = tokens
                        .next()
                        .expect("addx should have an operand")
                        .parse::<i64>()
                        .expect("operand should be a number");
                    command = Some(Command::Addx {
                        value,
                        elapsed: false,
                    });
                }
                "noop" => {
                    command = Some(Command::Noop);
                }
                _ => unreachable!("unrecognised command"),
            }
        }

        if i64::abs_diff(x, (cycle - 1) % 40) < 2 {
            output.push('#');
        } else {
            output.push('.');
        }
        if cycle % 40 == 0 {
            output.push('\n');
        }

        match command.take() {
            Some(cmd) => match cmd {
                Command::Noop => {}
                Command::Addx { value, elapsed } => {
                    if elapsed {
                        x += value;
                    } else {
                        command = Some(Command::Addx {
                            value,
                            elapsed: true,
                        });
                    }
                }
            },
            _ => unreachable!("command should have been loaded"),
        }
        cycle += 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 13140);
    }
}
