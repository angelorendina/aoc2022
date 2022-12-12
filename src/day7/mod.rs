use std::collections::BTreeMap;
use std::collections::VecDeque;

struct Folder {
    files: BTreeMap<String, u64>,
    folders: BTreeMap<String, Self>,
}

impl Folder {
    fn new() -> Self {
        Self {
            files: BTreeMap::new(),
            folders: BTreeMap::new(),
        }
    }

    fn touch(&mut self, name: String, size: u64) {
        self.files.insert(name, size);
    }

    fn mkdir(&mut self, name: String) {
        self.folders.insert(name, Self::new());
    }

    fn size(&self) -> u64 {
        self.folders.values().map(Self::size).sum::<u64>() + self.files.values().sum::<u64>()
    }
}

struct Filesystem {
    root: Folder,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            root: Folder::new(),
        }
    }

    fn parse(values: &str) -> Self {
        let mut filesystem = Filesystem::new();

        // stack of references to the parents of `cwd`: used to navigate up;
        // uses pointers, as borrowing rules make it impossible to have
        // separate &mut references to something AND its content
        // (since they'd be pointing to overlapping memory);
        // we tackle this by threading read/write access through
        // only one pointer at the time (the last of the list)
        struct Path(Vec<*mut Folder>);

        impl Path {
            fn go_up(&mut self) {
                if self.0.len() > 1 {
                    self.0.pop();
                }
            }

            fn go_root(&mut self) {
                self.0.truncate(1);
            }

            fn cwd(&mut self) -> &mut Folder {
                unsafe {
                    self.0
                        .last_mut()
                        .expect("path should not be empty")
                        .as_mut()
                        .expect("pointer should be dereferenceable")
                }
            }

            fn enter(&mut self, name: &str) {
                let cwd = self.cwd();
                let dir = cwd.folders.get_mut(name).expect("folder must exist") as *mut Folder;
                self.0.push(dir);
            }
        }

        let mut path = Path(vec![&mut filesystem.root as *mut Folder]);

        for line in values.lines() {
            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("$") => match tokens.next() {
                    Some("cd") => match tokens.next() {
                        Some("/") => {
                            path.go_root();
                        }
                        Some("..") => {
                            path.go_up();
                        }
                        Some(name) => {
                            path.enter(name);
                        }
                        _ => unreachable!("unrecognised cd operand"),
                    },
                    Some("ls") => {}
                    _ => unreachable!("unrecognised command"),
                },
                Some("dir") => {
                    let name = tokens.next().expect("file must have a name");
                    path.cwd().mkdir(name.to_string());
                }
                Some(size) => {
                    let name = tokens.next().expect("file must have a name");
                    let size = size.parse::<u64>().expect("size must be a u64");
                    path.cwd().touch(name.to_string(), size);
                }
                _ => unreachable!("unrecognised"),
            }
        }

        filesystem
    }
}

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let filesystem = Filesystem::parse(values);

    let mut s = 0;
    let mut queue = VecDeque::from([&filesystem.root]);
    while let Some(folder) = queue.pop_front() {
        let size = folder.size();
        if size <= 100_000 {
            s += size;
        }
        for child in folder.folders.values() {
            queue.push_back(child);
        }
    }
    s
}

pub fn star_two() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let filesystem = Filesystem::parse(values);

    let used_space = filesystem.root.size();
    let free_space = 70_000_000 - used_space;
    let required_space = 30_000_000 - free_space;

    let mut folder_by_size = BTreeMap::new();

    let mut queue = VecDeque::from([&filesystem.root]);
    while let Some(folder) = queue.pop_front() {
        let size = folder.size();
        folder_by_size.insert(size, folder);
        for child in folder.folders.values() {
            queue.push_back(child);
        }
    }

    let (size, _) = folder_by_size
        .range(required_space..)
        .next()
        .expect("there should be a big enough folder");
    *size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 95437);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 24933642);
    }
}
