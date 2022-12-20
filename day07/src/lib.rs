use std::{rc::Rc, vec};

pub fn process_part1(input: &str) -> usize {
    let fs = create_fs_from_commands(input.lines().collect());
    process_part1_help(&fs)
}

pub fn process_part2(input: &str) -> usize {
    let fs = create_fs_from_commands(input.lines().collect());
    process_part2_help(&fs)
}

fn process_part1_help(fs: &Inode) -> usize {
    fs.flatten()
        .iter()
        .filter_map(|item| {
            if item.is_dir() {
                match item.size() {
                    s if s <= 100000 => Some(s),
                    _ => None,
                }
            } else {
                None
            }
        })
        .sum()
}

fn process_part2_help(fs: &Inode) -> usize {
    let required_space = 30000000 - (70000000 - fs.size());
    fs.flatten()
        .iter()
        .filter_map(|item| {
            if item.is_dir() {
                match item.size() {
                    s if s >= required_space => Some(s),
                    _ => None,
                }
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn create_fs_from_commands(commands: Vec<&str>) -> Inode {
    let mut root = Inode::Directory(Directory {
        name: String::from("/"),
        content: Vec::new(),
    });
    let mut path: Vec<&str> = Vec::new();
    for cmd in commands {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match parts.as_slice() {
            ["$", "cd", "/"] => (),
            ["$", "cd", ".."] => {
                path.pop();
            }
            ["$", "cd", name] => path.push(name),
            ["$", _] => (),
            ["dir", name] => {
                root = root.insert_item(
                    &path,
                    Rc::new(Inode::Directory(Directory::new(
                        name.to_string(),
                        Vec::new(),
                    ))),
                )
            }
            [size, name] => {
                root = root.insert_item(
                    &path,
                    Rc::new(Inode::File(File::new(
                        name.to_string(),
                        size.parse().unwrap(),
                    ))),
                )
            }
            _ => (),
        }
    }

    return root;
}

enum Inode {
    File(File),
    Directory(Directory),
}

impl Inode {
    fn insert_item(&self, path: &[&str], item: Rc<Inode>) -> Inode {
        match self {
            Self::File(file) => Self::File(File::new(file.name.to_string(), file.size)),
            Self::Directory(dir) => dir.insert_item(path, item),
        }
    }

    fn is_same(&self, other: &Inode) -> bool {
        match (self, other) {
            (Self::File(this), Self::File(other)) => this.is_same(other),
            (Self::Directory(this), Self::Directory(other)) => this.is_same(other),
            _ => false,
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            Self::File(_) => false,
            Self::Directory(_) => true,
        }
    }

    fn name(&self) -> &str {
        match self {
            Inode::File(file) => &file.name,
            Inode::Directory(dir) => &dir.name,
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::File(file) => file.size(),
            Self::Directory(dir) => dir.size(),
        }
    }

    fn flatten(&self) -> Vec<Rc<Inode>> {
        match self {
            Self::File(_) => vec![Rc::new(Self::File(File::new(
                self.name().to_string(),
                self.size(),
            )))],
            Self::Directory(dir) => vec![Rc::new(Self::Directory(Directory::new(
                self.name().to_string(),
                dir.content.to_vec(),
            )))]
            .into_iter()
            .chain(
                dir.content
                    .iter()
                    .map(|item| item.flatten())
                    .flat_map(|item| item),
            )
            .collect(),
        }
    }
}

struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> File {
        File { name, size }
    }

    fn is_same(&self, other: &File) -> bool {
        self.name.eq(&other.name) && self.size == other.size
    }

    fn size(&self) -> usize {
        self.size
    }
}

struct Directory {
    name: String,
    content: Vec<Rc<Inode>>,
}

impl Directory {
    fn new(name: String, content: Vec<Rc<Inode>>) -> Directory {
        Directory { name, content }
    }

    fn insert_item(&self, path: &[&str], item: Rc<Inode>) -> Inode {
        match path.first() {
            Some(first) => {
                let content: Vec<Rc<Inode>> = self
                    .content
                    .iter()
                    .map(|inode| {
                        if inode.is_dir() && inode.name().eq(*first) {
                            Rc::new(inode.insert_item(&path[1..], Rc::clone(&item)))
                        } else {
                            Rc::clone(inode)
                        }
                    })
                    .collect();
                Inode::Directory(Self {
                    name: self.name.clone(),
                    content: content,
                })
            }
            None => {
                let mut content: Vec<_> = self.content.to_vec();
                content.push(Rc::clone(&item));
                Inode::Directory(Self {
                    name: self.name.clone(),
                    content: content,
                })
            }
        }
    }

    fn is_same(&self, other: &Directory) -> bool {
        self.name.eq(&other.name)
            && self.content.len() == other.content.len()
            && self
                .content
                .iter()
                .zip(other.content.iter())
                .all(|(item1, item2)| item1.is_same(item2))
    }

    fn size(&self) -> usize {
        self.content.iter().map(|item| item.size()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1_help() {
        let fs = create_example_fs1();
        assert_eq!(95437, process_part1_help(&fs));
    }

    #[test]
    fn test_process_part2_help() {
        let fs = create_example_fs1();
        assert_eq!(24933642, process_part2_help(&fs));
    }

    #[test]
    fn test_create_fs_from_commands() {
        let commands = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        let fs1 = create_fs_from_commands(commands);
        let fs2 = create_example_fs1();
        assert!(fs1.is_same(&fs2));
    }

    #[test]
    fn test_size() {
        let fs = create_example_fs1();
        assert_eq!(48381165, fs.size())
    }

    #[test]
    fn test_is_same_fs() {
        assert!(create_example_fs1().is_same(&create_example_fs1()));
        assert!(!create_example_fs2().is_same(&create_example_fs3()));
    }

    #[test]
    fn test_insert_item() {
        let fs2 = create_example_fs2();
        let item = Rc::new(Inode::File(File {
            name: String::from("i"),
            size: 5,
        }));
        let fs3 = fs2.insert_item(&vec!["c", "f"], item);
        assert!(fs3.is_same(&create_example_fs3()));
    }

    fn create_example_fs1() -> Inode {
        Inode::Directory(Directory {
            name: String::from("/"),
            content: vec![
                Rc::new(Inode::Directory(Directory {
                    name: String::from("a"),
                    content: vec![
                        Rc::new(Inode::Directory(Directory {
                            name: String::from("e"),
                            content: vec![Rc::new(Inode::File(File {
                                name: String::from("i"),
                                size: 584,
                            }))],
                        })),
                        Rc::new(Inode::File(File {
                            name: String::from("f"),
                            size: 29116,
                        })),
                        Rc::new(Inode::File(File {
                            name: String::from("g"),
                            size: 2557,
                        })),
                        Rc::new(Inode::File(File {
                            name: String::from("h.lst"),
                            size: 62596,
                        })),
                    ],
                })),
                Rc::new(Inode::File(File {
                    name: String::from("b.txt"),
                    size: 14848514,
                })),
                Rc::new(Inode::File(File {
                    name: String::from("c.dat"),
                    size: 8504156,
                })),
                Rc::new(Inode::Directory(Directory {
                    name: String::from("d"),
                    content: vec![
                        Rc::new(Inode::File(File {
                            name: String::from("j"),
                            size: 4060174,
                        })),
                        Rc::new(Inode::File(File {
                            name: String::from("d.log"),
                            size: 8033020,
                        })),
                        Rc::new(Inode::File(File {
                            name: String::from("d.ext"),
                            size: 5626152,
                        })),
                        Rc::new(Inode::File(File {
                            name: String::from("k"),
                            size: 7214296,
                        })),
                    ],
                })),
            ],
        })
    }

    fn create_example_fs2() -> Inode {
        Inode::Directory(Directory {
            name: String::from("a"),
            content: vec![
                Rc::new(Inode::File(File {
                    name: String::from("b"),
                    size: 5,
                })),
                Rc::new(Inode::Directory(Directory {
                    name: String::from("c"),
                    content: vec![
                        Rc::new(Inode::File(File {
                            name: String::from("e"),
                            size: 5,
                        })),
                        Rc::new(Inode::Directory(Directory {
                            name: String::from("f"),
                            content: vec![
                                Rc::new(Inode::File(File {
                                    name: String::from("g"),
                                    size: 5,
                                })),
                                Rc::new(Inode::File(File {
                                    name: String::from("h"),
                                    size: 5,
                                })),
                            ],
                        })),
                    ],
                })),
                Rc::new(Inode::File(File {
                    name: String::from("d"),
                    size: 5,
                })),
            ],
        })
    }

    fn create_example_fs3() -> Inode {
        Inode::Directory(Directory {
            name: String::from("a"),
            content: vec![
                Rc::new(Inode::File(File {
                    name: String::from("b"),
                    size: 5,
                })),
                Rc::new(Inode::Directory(Directory {
                    name: String::from("c"),
                    content: vec![
                        Rc::new(Inode::File(File {
                            name: String::from("e"),
                            size: 5,
                        })),
                        Rc::new(Inode::Directory(Directory {
                            name: String::from("f"),
                            content: vec![
                                Rc::new(Inode::File(File {
                                    name: String::from("g"),
                                    size: 5,
                                })),
                                Rc::new(Inode::File(File {
                                    name: String::from("h"),
                                    size: 5,
                                })),
                                Rc::new(Inode::File(File {
                                    name: String::from("i"),
                                    size: 5,
                                })),
                            ],
                        })),
                    ],
                })),
                Rc::new(Inode::File(File {
                    name: String::from("d"),
                    size: 5,
                })),
            ],
        })
    }
}
