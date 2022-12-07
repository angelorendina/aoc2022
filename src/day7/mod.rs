use std::collections::BTreeMap;
use std::collections::VecDeque;

enum Entity {
    File { size: u64 },
    Folder { content: BTreeMap<String, Entity> },
}

impl Entity {
    fn touch(size: u64) -> Self {
        Entity::File { size }
    }

    fn mkdir() -> Self {
        Entity::Folder {
            content: BTreeMap::new(),
        }
    }

    fn size(&self) -> u64 {
        match self {
            Entity::File { size } => *size,
            Entity::Folder { content } => content.values().map(Self::size).sum::<u64>(),
        }
    }
}

#[derive(Clone)]
struct Path {
    tokens: Vec<String>,
}

impl Path {
    fn new() -> Self {
        Self { tokens: vec![] }
    }

    fn push(&mut self, token: String) {
        self.tokens.push(token);
    }

    fn pop(&mut self) {
        self.tokens.pop();
    }
}

struct Filesystem {
    root: Entity,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            root: Entity::mkdir(),
        }
    }

    fn get(&self, path: &Path) -> &Entity {
        let mut entity = &self.root;
        for token in path.tokens.iter() {
            let Entity::Folder { content: folder } = entity else { unreachable!() };
            entity = folder.get(token).expect("No such entity.");
        }
        entity
    }

    fn get_mut(&mut self, path: &Path) -> &mut Entity {
        let mut entity = &mut self.root;
        for token in path.tokens.iter() {
            let Entity::Folder { content: folder } = entity else { unreachable!() };
            entity = folder.get_mut(token).expect("No such entity.");
        }
        entity
    }

    fn parse(values: &str) -> Self {
        let mut filesystem = Filesystem::new();
        let mut path = Path::new();

        for line in values.lines() {
            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("$") => match tokens.next() {
                    Some("cd") => match tokens.next() {
                        Some("/") => {
                            path = Path::new();
                        }
                        Some("..") => {
                            path.pop();
                        }
                        Some(name) => {
                            path.push(name.to_string());
                        }
                        _ => unreachable!(),
                    },
                    Some("ls") => {}
                    _ => unreachable!(),
                },
                Some("dir") => {
                    let name = tokens.next().unwrap();
                    let Entity::Folder { content: cwd } = filesystem.get_mut(&path) else { unreachable!() };
                    cwd.entry(name.to_string()).or_insert_with(Entity::mkdir);
                }
                Some(size) => {
                    let name = tokens.next().unwrap();
                    let size = size.parse::<u64>().unwrap();
                    let Entity::Folder { content: cwd } = filesystem.get_mut(&path) else { unreachable!() };
                    cwd.entry(name.to_string())
                        .or_insert_with(|| Entity::touch(size));
                }
                _ => unreachable!(),
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
    let mut queue = VecDeque::from([filesystem.get(&Path::new())]);
    while let Some(entity) = queue.pop_front() {
        let Entity::Folder { content } = entity else { continue };
        let size = entity.size();
        if size <= 100_000 {
            s += size;
        }
        for child in content.values() {
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

    let used_space = filesystem.get(&Path::new()).size();
    let free_space = 70_000_000 - used_space;
    let required_space = 30_000_000 - free_space;

    let mut folder_by_size = BTreeMap::new();

    let mut queue = VecDeque::from([filesystem.get(&Path::new())]);
    while let Some(entity) = queue.pop_front() {
        let Entity::Folder { content } = entity else { continue };
        let size = entity.size();
        folder_by_size.insert(size, entity);
        for child in content.values() {
            queue.push_back(child);
        }
    }

    let (size, _) = folder_by_size.range(required_space..).next().unwrap();
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
