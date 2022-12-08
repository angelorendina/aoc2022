use std::collections::BTreeMap;

struct GridMap<T> {
    data: BTreeMap<usize, BTreeMap<usize, T>>,
    rows: usize,
    columns: usize,
}

impl<T> GridMap<T> {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            rows: 0,
            columns: 0,
        }
    }

    fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.data.get(&row).and_then(|row| row.get(&column))
    }

    fn set(&mut self, row: usize, column: usize, value: T) {
        self.data.entry(row).or_default().insert(column, value);
        self.rows = self.rows.max(row + 1);
        self.columns = self.columns.max(column + 1);
    }

    fn row(&self, row: usize) -> Option<impl DoubleEndedIterator<Item = (usize, Option<&T>)>> {
        if row >= self.rows {
            None
        } else {
            Some((0..self.columns).map(move |column| (column, self.get(row, column))))
        }
    }

    fn column(
        &self,
        column: usize,
    ) -> Option<impl DoubleEndedIterator<Item = (usize, Option<&T>)>> {
        if column >= self.columns {
            None
        } else {
            Some((0..self.rows).map(move |row| (row, self.get(row, column))))
        }
    }
}

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = GridMap::<u8>::new();

    for (row, line) in values.lines().enumerate() {
        for (column, b) in line.bytes().enumerate() {
            if b < 48 {
                break;
            }
            map.set(row, column, b as u8 - 48);
        }
    }

    let mut visible = GridMap::<bool>::new();

    for row in 0..map.rows {
        if let Some(ltr) = map.row(row) {
            ltr.fold(None, |mut max_height, data| {
                let column = data.0;
                if let Some(&height) = data.1 {
                    match max_height {
                        Some(max) => {
                            if height > max {
                                max_height = Some(height);
                                visible.set(row, column, true);
                            }
                        }
                        None => {
                            max_height = Some(height);
                            visible.set(row, column, true);
                        }
                    }
                }
                max_height
            });
        }
        if let Some(ltr) = map.row(row) {
            ltr.rev().fold(None, |mut max_height, data| {
                let column = data.0;
                if let Some(&height) = data.1 {
                    match max_height {
                        Some(max) => {
                            if height > max {
                                max_height = Some(height);
                                visible.set(row, column, true);
                            }
                        }
                        None => {
                            max_height = Some(height);
                            visible.set(row, column, true);
                        }
                    }
                }
                max_height
            });
        }
    }

    for column in 0..map.columns {
        if let Some(ttb) = map.column(column) {
            ttb.fold(None, |mut max_height, data| {
                let row = data.0;
                if let Some(&height) = data.1 {
                    match max_height {
                        Some(max) => {
                            if height > max {
                                max_height = Some(height);
                                visible.set(row, column, true);
                            }
                        }
                        None => {
                            max_height = Some(height);
                            visible.set(row, column, true);
                        }
                    }
                }
                max_height
            });
        }
        if let Some(ttb) = map.column(column) {
            ttb.rev().fold(None, |mut max_height, data| {
                let row = data.0;
                if let Some(&height) = data.1 {
                    match max_height {
                        Some(max) => {
                            if height > max {
                                max_height = Some(height);
                                visible.set(row, column, true);
                            }
                        }
                        None => {
                            max_height = Some(height);
                            visible.set(row, column, true);
                        }
                    }
                }
                max_height
            });
        }
    }

    let mut s = 0;
    for row in 0..visible.rows {
        for column in 0..visible.columns {
            if let Some(&true) = visible.get(row, column) {
                s += 1;
            }
        }
    }
    s
}

pub fn star_two() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = GridMap::<u8>::new();

    for (row, line) in values.lines().enumerate() {
        for (column, b) in line.bytes().enumerate() {
            if b < 48 {
                break;
            }
            map.set(row, column, b as u8 - 48);
        }
    }

    let mut scores = GridMap::new();

    for row in 1..(map.rows - 1) {
        for column in 1..(map.columns - 1) {
            scores.set(row, column, {
                let &house_height = map.get(row, column).unwrap();

                let ltr = map.row(row).unwrap().skip_while(|(c, _)| *c <= column);
                let mut ltr_score = 0u64;
                for (_, h) in ltr {
                    let Some(&h) = h else { continue };
                    ltr_score += 1;
                    if h >= house_height {
                        break;
                    }
                }

                let rtl = map
                    .row(row)
                    .unwrap()
                    .rev()
                    .skip_while(|(c, _)| *c >= column);
                let mut rtl_score = 0u64;
                for (_, h) in rtl {
                    let Some(&h) = h else { continue };
                    rtl_score += 1;
                    if h >= house_height {
                        break;
                    }
                }

                let ttb = map.column(column).unwrap().skip_while(|(r, _)| *r <= row);
                let mut ttb_score = 0u64;
                for (_, h) in ttb {
                    let Some(&h) = h else { continue };
                    ttb_score += 1;
                    if h >= house_height {
                        break;
                    }
                }

                let btt = map
                    .column(column)
                    .unwrap()
                    .rev()
                    .skip_while(|(r, _)| *r >= row);
                let mut btt_score = 0u64;
                for (_, h) in btt {
                    let Some(&h) = h else { continue };
                    btt_score += 1;
                    if h >= house_height {
                        break;
                    }
                }

                ltr_score * rtl_score * ttb_score * btt_score
            });
        }
    }

    let mut s = 0u64;
    for row in 1..scores.rows {
        for column in 1..scores.columns {
            if let Some(&score) = scores.get(row, column) {
                if score > s {
                    s = score;
                }
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 21);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 8);
    }
}
