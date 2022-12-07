use std::fmt::Debug;
use std::iter::Skip;
use std::str::Split;

struct Parameter<'a>(&'a str);
struct Response<'a>(&'a str);

/// Tokenize the available commands. Holds the response.
enum Command<'a> {
    ListDirectory(Response<'a>),
    ChangeDirectory(Parameter<'a>),
}

/// Implements an iterator for &str that returns a Command.
#[derive(Debug, Clone)]
struct CommandIterator<'a> {
    iterator: Skip<Split<'a, &'a str>>,
}

impl<'a> CommandIterator<'a> {
    fn new(input: &'a str) -> CommandIterator<'a> {
        CommandIterator {
            iterator: input.split("$ ").skip(1),
        }
    }
}

impl<'a> Iterator for CommandIterator<'a> {
    type Item = Command<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut command_and_response = self.iterator.next()?.splitn(2, "\n");
        let command = command_and_response.next()?;

        match command[..2].as_ref() {
            "cd" => Some(Command::ChangeDirectory(Parameter(command[3..].trim()))),
            "ls" => Some(Command::ListDirectory(Response(
                command_and_response.next()?,
            ))),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Directory {
    contains: Vec<Entry>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            contains: Vec::new(),
        }
    }

    fn add_entry(&mut self, entry: Entry) {
        self.contains.push(entry);
    }

    fn size(&self) -> u64 {
        self.contains.iter().map(|entry| entry.size()).sum()
    }

    fn get_entries(&self) -> Vec<&Directory> {
        let mut temp: Vec<&Directory> = self
            .contains
            .iter()
            .filter_map(|entry| match entry {
                Entry::Directory(directory) => Some(directory),
                _ => None,
            })
            .flat_map(|dir| dir.get_entries())
            .collect();

        temp.push(self);
        temp
    }
}

#[derive(Debug)]
struct File {
    size: u64,
}

impl File {
    fn new(size: u64) -> File {
        File { size }
    }

    fn size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug)]
enum Entry {
    Directory(Directory),
    File(File),
}

impl Entry {
    fn size(&self) -> u64 {
        match self {
            Entry::Directory(directory) => directory.size(),
            Entry::File(file) => file.size(),
        }
    }
}

impl TryFrom<&str> for Entry {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split_whitespace();
        let size_or_dir = iter.next().ok_or("No size or dir found")?;

        match size_or_dir.parse::<u64>() {
            Ok(size) => Ok(Entry::File(File::new(size))),
            Err(_) => Ok(Entry::Directory(Directory::new())),
        }
    }
}

impl<'a> TryFrom<&'a str> for Directory {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut directories: Vec<Directory> = vec![Directory::new()];

        for command in CommandIterator::new(value) {
            match command {
                Command::ChangeDirectory(Parameter(path)) => match path {
                    "/" => continue,
                    ".." => {
                        let current_dir = directories.pop().unwrap();
                        directories
                            .last_mut()
                            .unwrap()
                            .add_entry(Entry::Directory(current_dir));
                    }
                    _ => {
                        directories.push(Directory::new());
                    }
                },
                Command::ListDirectory(Response(response)) => {
                    let entries: Vec<Entry> = response
                        .lines()
                        .map(|line| Entry::try_from(line).unwrap())
                        .collect();

                    for entry in entries.into_iter() {
                        match entry {
                            Entry::Directory(_) => {} // ignore directories, because they are useless if we do not cd into them.
                            Entry::File(file) => {
                                directories.last_mut().unwrap().add_entry(Entry::File(file))
                            }
                        };
                    }
                }
            }
        }

        while directories.len() > 1 {
            if let Some(child_dir) = directories.pop() {
                directories
                    .last_mut()
                    .unwrap()
                    .add_entry(Entry::Directory(child_dir));
            }
        }

        assert_eq!(directories.len(), 1);
        directories.pop().ok_or("No directory found")
    }
}

fn day7_part1(text: &str) -> u64 {
    let computer = Directory::try_from(text).unwrap();
    computer
        .get_entries()
        .iter()
        .filter_map(|&dir| (dir.size() <= 100_000).then(|| dir.size()))
        .sum()
}

fn day7_part2(text: &str) -> u64 {
    let computer = Directory::try_from(text).unwrap();
    let needed_size = 30_000_000 - (70_000_000 - computer.size());

    computer
        .get_entries()
        .iter()
        .filter_map(|&dir| (dir.size() >= needed_size).then(|| dir.size()))
        .min()
        .unwrap()
}

pub fn day7(text: &str) {
    println!("day7 part 1: {}", day7_part1(text));
    println!("day7 part 2: {}", day7_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day7_part1, day7_part2};

    const TEST_INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
0 h.lst
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn fixed_tests1() {
        assert_eq!(day7_part1(TEST_INPUT), 95437);
        /// Taken grateful from https://www.reddit.com/r/adventofcode/comments/zf791a/comment/izackde/?utm_source=share&utm_medium=web2x&context=3
        assert_eq!(
            day7_part1(
                r#"$ cd /
$ ls
dir a
$ cd a
$ ls
dir a
2 a.txt
$ cd a
$ ls
99999 a.txt"#
            ),
            99999
        );
    }

    #[test]
    fn fixed_tests2() {
        assert_eq!(day7_part2(TEST_INPUT), 24933642);
    }
}
