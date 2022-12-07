use std::{cell::RefCell, cmp::min, rc::Rc, str::FromStr};

use utils::parse_text;

fn main() {
    let text = parse_text();

    let root = Rc::new(RefCell::new(Directory {
        name: String::from("/"),
        parent: None,
        files: Vec::new(),
        dirs: Vec::new(),
    }));

    create_structure(root.clone(), &text);

    let dir_sum = sum_tree(root.clone());
    let smallest_delete = find_smallest_delete(
        30000000 + sum_size(root.clone()) - 70000000,
        usize::MAX,
        root,
    );
    println!("The sum of directories is {}", dir_sum);
    println!("The smallest delete is {}", smallest_delete);
}

fn create_structure(root: Rc<RefCell<Directory>>, text: &str) {
    let mut current_dir = root.clone();

    for line in text.lines() {
        let line = line.trim();
        if let Ok(cmd) = line.parse::<Command>() {
            match cmd {
                Command::ChangeUp => {
                    let parent = current_dir.borrow().parent.as_ref().unwrap().clone();
                    current_dir = parent;
                }
                Command::ChangeDown(x) => {
                    current_dir = {
                        let c = &current_dir.borrow().dirs;
                        let changed_dir = c.iter().find(|&y| y.borrow().name == x).unwrap();
                        changed_dir.clone()
                    }
                }
                Command::ChangeRoot => current_dir = root.clone(),
                Command::List => (),
            }
        }
        if let Ok(file) = line.parse::<File>() {
            current_dir.borrow_mut().files.push(file);
        }
        if let Ok(mut dir) = line.parse::<Directory>() {
            dir.parent = Some(Rc::clone(&current_dir));
            current_dir
                .borrow_mut()
                .dirs
                .push(Rc::new(RefCell::new(dir)));
        }
    }
}

fn sum_size(root: Rc<RefCell<Directory>>) -> usize {
    let mut own_size = root.borrow().files.iter().map(|f| f.size).sum();
    if root.borrow().dirs.is_empty() {
        own_size
    } else {
        for dir in &root.borrow().dirs {
            own_size += sum_size(dir.clone());
        }
        own_size
    }
}

fn sum_tree(root: Rc<RefCell<Directory>>) -> usize {
    let own_size = sum_size(root.clone());
    let mut result = 0;
    for dir in &root.borrow().dirs {
        result += sum_tree(dir.clone());
    }
    if own_size > 100000 {
        result
    } else {
        result + own_size
    }
}

fn find_smallest_delete(
    min_requried_size: usize,
    current_min: usize,
    root: Rc<RefCell<Directory>>,
) -> usize {
    let own_size = sum_size(root.clone());

    if own_size < min_requried_size {
        return current_min;
    }

    if root.borrow().dirs.is_empty() {
        min(own_size, current_min)
    } else {
        let mut new_min = usize::MAX;
        for dir in &root.borrow().dirs {
            let child_min =
                find_smallest_delete(min_requried_size, min(current_min, own_size), dir.clone());
            new_min = min(new_min, child_min);
        }
        new_min
    }
}

#[derive(Debug)]
enum Command {
    ChangeUp,
    ChangeDown(String),
    ChangeRoot,
    List,
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err: Result<Self, Self::Err> = Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "This line is just not a Command",
        )));
        if s.starts_with("$ ") {
            let args: Vec<&str> = s
                .strip_prefix("$ ")
                .expect("Should be possible after if")
                .split_whitespace()
                .collect();
            let cmd = match args[0] {
                "ls" => Command::List,
                "cd" => match args[1] {
                    "/" => Command::ChangeRoot,
                    ".." => Command::ChangeUp,
                    x if !x.is_empty() => Command::ChangeDown(String::from(x)),
                    _ => return err,
                },
                _ => return err,
            };
            return Ok(cmd);
        }
        err
    }
}

#[derive(Debug)]
struct File {
    size: usize,
    _name: String,
}

impl FromStr for File {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err: Result<Self, Self::Err> = Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "This line is just not a Command",
        )));
        if !s.starts_with("$ ") {
            let args: Vec<&str> = s.split_whitespace().collect();
            let cmd = match args[0] {
                x if x.parse::<usize>().is_ok() => File {
                    size: x.parse::<usize>().unwrap(),
                    _name: String::from(args[1]),
                },
                _ => return err,
            };
            return Ok(cmd);
        }
        err
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    files: Vec<File>,
    dirs: Vec<Rc<RefCell<Directory>>>,
}

impl FromStr for Directory {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err: Result<Self, Self::Err> = Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "This line is just not a Command",
        )));
        if !s.starts_with("$ ") {
            let args: Vec<&str> = s.split_whitespace().collect();
            let cmd = match args[0] {
                "dir" => Directory {
                    name: String::from(args[1]),
                    parent: None,
                    files: Vec::new(),
                    dirs: Vec::new(),
                },
                _ => return err,
            };
            return Ok(cmd);
        }
        err
    }
}
