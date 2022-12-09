use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Directory {
    parent: Option<String>,
    name: String,
    directories: Vec<String>,
    files: Vec<String>,
}

impl Directory {
    fn new<T: ToString>(name: T, parent: Option<String>) -> Directory {
        Directory {
            parent,
            name: name.to_string(),
            directories: vec![],
            files: vec![],
        }
    }
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new<T: ToString>(name: T, size: usize) -> File {
        File {
            name: name.to_string(),
            size,
        }
    }
}

fn total_size(
    dir: String,
    dir_index: &HashMap<String, Directory>,
    file_index: &HashMap<String, File>,
    self_size: &HashMap<String, usize>,
) -> usize {
    let total_size = dir_index
        .get(&dir)
        .unwrap()
        .directories
        .iter()
        .map(|dir| total_size(dir.clone(), dir_index, file_index, self_size))
        .sum::<usize>()
        + self_size.get(&dir.clone()).unwrap();

    total_size
}

fn main() {
    let input = include_str!("input.txt");

    let mut dir_index: HashMap<String, Directory> = HashMap::new();
    let mut file_index: HashMap<String, File> = HashMap::new();
    let root = Directory::new("/", None);
    dir_index.insert(root.name.clone(), root);

    let mut current: String = dir_index.get("/").unwrap().name.clone();

    for line in input.lines().skip(1) {
        if line.starts_with("$") {
            if line.contains("cd") {
                // dbg!(&current, &line);
                let next = &line[5..line.len()];
                if next == ".." {
                    let current_dir = dir_index.get(&current).unwrap();
                    current = current_dir.parent.clone().unwrap();
                } else {
                    current = current.clone() + next + "/";
                }
            }
        } else {
            if line.starts_with("dir") {
                let name = current.clone() + &line[4..line.len()] + "/";

                let new = Directory::new(name, Some(current.clone()));
                dir_index.insert(new.name.clone(), new.clone());
                dir_index
                    .get_mut(&current)
                    .unwrap()
                    .directories
                    .push(new.name.clone());
            } else {
                let (size, name) = line.split_once(" ").unwrap();
                let name = current.clone() + name + "/";
                let file = File::new(name, size.parse::<usize>().unwrap());
                file_index.insert(file.name.clone(), file.clone());
                dir_index
                    .get_mut(&current)
                    .unwrap()
                    .files
                    .push(file.name.clone());
            }
        }
    }

    let mut self_sizes: HashMap<String, usize> = HashMap::new();
    for dir in dir_index.values() {
        let self_size: usize = dir
            .files
            .iter()
            .map(|file| file_index.get(file).unwrap().size)
            .sum();

        self_sizes.insert(dir.name.clone(), self_size);
    }

    let directories = dir_index
        .values()
        .map(|dir| dir.name.clone())
        .collect::<Vec<String>>();

    let small_directories = directories
        .iter()
        .map(|dir| {
            (
                dir.clone(),
                total_size(dir.clone(), &dir_index, &file_index, &self_sizes),
            )
        })
        .filter(|(dir, total_size)| total_size <= &100000)
        .map(|(_, total_size)| total_size)
        // .collect::<Vec<(String, usize)>>();
    .sum::<usize>();

    dbg!(small_directories);

    // TODO: calculate nested sizes
}
