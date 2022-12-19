use std::vec;

pub fn process_part1(input: &str) -> String {
    input.to_uppercase()
}

pub fn process_part2(input: &str) -> String {
    input.to_uppercase()
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
            ["$", "cd", ".."] => drop(path.pop()),
            ["$", "cd", name] => path.push(name),
            ["$", _] => (),
            ["dir", name] => {
                root = root.insert_item(
                    &path,
                    &Inode::Directory(Directory {
                        name: name.to_string(),
                        content: Vec::new(),
                    }),
                )
            }
            [size, name] => {
                root = root.insert_item(
                    &path,
                    &Inode::File(File {
                        name: name.to_string(),
                        size: size.parse().unwrap(),
                    }),
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

impl Clone for Inode {
    fn clone(&self) -> Self {
        match self {
            Self::File(file) => Self::File(file.clone()),
            Self::Directory(dir) => Self::Directory(dir.clone()),
        }
    }
}

impl Inode {
    fn insert_item(&self, path: &[&str], item: &Inode) -> Inode {
        match self {
            Self::File(file) => Self::File(file.clone()),
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

    fn is_dir_with_name(&self, name: &str) -> bool {
        match self {
            Self::File(_) => false,
            Self::Directory(dir) => dir.name.eq(name),
        }
    }

    fn map<T>(&self, func: fn(&Inode) -> T) -> Vec<T> {
        match self {
            Inode::File(_) => vec![func(&self)],
            Inode::Directory(_) => todo!(),
        }
    }
}

#[derive(Clone)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn is_same(&self, other: &File) -> bool {
        self.name.eq(&other.name) && self.size == other.size
    }
}

struct Directory {
    name: String,
    content: Vec<Inode>,
}

impl Clone for Directory {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            content: self.content.to_vec(),
        }
    }
}

impl Directory {
    fn insert_item(&self, path: &[&str], item: &Inode) -> Inode {
        match path.first() {
            Some(first) => {
                let content: Vec<Inode> = self
                    .content
                    .iter()
                    .map(|inode| {
                        if inode.is_dir_with_name(first) {
                            inode.insert_item(&path[1..], item)
                        } else {
                            inode.clone()
                        }
                    })
                    .collect();
                Inode::Directory(Self {
                    name: self.name.clone(),
                    content: content,
                })
            }
            None => {
                let mut content: Vec<Inode> = self.content.to_vec();
                content.push(item.clone());
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "input";
        let result = process_part1(input);
        assert_eq!("INPUT", result);
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
    fn test_is_same_fs() {
        assert!(create_example_fs1().is_same(&create_example_fs1()));
        assert!(!create_example_fs2().is_same(&create_example_fs3()));
    }

    #[test]
    fn test_insert_item() {
        let fs2 = create_example_fs2();
        let item = Inode::File(File {
            name: String::from("i"),
            size: 5,
        });
        let fs3 = fs2.insert_item(&vec!["c", "f"], &item);
        assert!(fs3.is_same(&create_example_fs3()));
    }

    fn create_example_fs1() -> Inode {
        Inode::Directory(Directory {
            name: String::from("/"),
            content: vec![
                Inode::Directory(Directory {
                    name: String::from("a"),
                    content: vec![
                        Inode::Directory(Directory {
                            name: String::from("e"),
                            content: vec![Inode::File(File {
                                name: String::from("i"),
                                size: 584,
                            })],
                        }),
                        Inode::File(File {
                            name: String::from("f"),
                            size: 29116,
                        }),
                        Inode::File(File {
                            name: String::from("g"),
                            size: 2557,
                        }),
                        Inode::File(File {
                            name: String::from("h.lst"),
                            size: 62596,
                        }),
                    ],
                }),
                Inode::File(File {
                    name: String::from("b.txt"),
                    size: 14848514,
                }),
                Inode::File(File {
                    name: String::from("c.dat"),
                    size: 8504156,
                }),
                Inode::Directory(Directory {
                    name: String::from("d"),
                    content: vec![
                        Inode::File(File {
                            name: String::from("j"),
                            size: 4060174,
                        }),
                        Inode::File(File {
                            name: String::from("d.log"),
                            size: 8033020,
                        }),
                        Inode::File(File {
                            name: String::from("d.ext"),
                            size: 5626152,
                        }),
                        Inode::File(File {
                            name: String::from("k"),
                            size: 7214296,
                        }),
                    ],
                }),
            ],
        })
    }

    fn create_example_fs2() -> Inode {
        Inode::Directory(Directory {
            name: String::from("a"),
            content: vec![
                Inode::File(File {
                    name: String::from("b"),
                    size: 5,
                }),
                Inode::Directory(Directory {
                    name: String::from("c"),
                    content: vec![
                        Inode::File(File {
                            name: String::from("e"),
                            size: 5,
                        }),
                        Inode::Directory(Directory {
                            name: String::from("f"),
                            content: vec![
                                Inode::File(File {
                                    name: String::from("g"),
                                    size: 5,
                                }),
                                Inode::File(File {
                                    name: String::from("h"),
                                    size: 5,
                                }),
                            ],
                        }),
                    ],
                }),
                Inode::File(File {
                    name: String::from("d"),
                    size: 5,
                }),
            ],
        })
    }

    fn create_example_fs3() -> Inode {
        Inode::Directory(Directory {
            name: String::from("a"),
            content: vec![
                Inode::File(File {
                    name: String::from("b"),
                    size: 5,
                }),
                Inode::Directory(Directory {
                    name: String::from("c"),
                    content: vec![
                        Inode::File(File {
                            name: String::from("e"),
                            size: 5,
                        }),
                        Inode::Directory(Directory {
                            name: String::from("f"),
                            content: vec![
                                Inode::File(File {
                                    name: String::from("g"),
                                    size: 5,
                                }),
                                Inode::File(File {
                                    name: String::from("h"),
                                    size: 5,
                                }),
                                Inode::File(File {
                                    name: String::from("i"),
                                    size: 5,
                                }),
                            ],
                        }),
                    ],
                }),
                Inode::File(File {
                    name: String::from("d"),
                    size: 5,
                }),
            ],
        })
    }
}
