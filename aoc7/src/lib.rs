use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

trait FsEntrySize<'a> {
    fn get_size(&self) -> usize;
}

/// Represents a dir.
/// The lifetime generic 'a specifies that the dyn FsEntrySize trait objects
/// must live for lifetime of this object (helps with add_child).
#[derive(Default)] // root dir
pub struct Dir<'a> {
    parent: Option<Rc<RefCell<Dir<'a>>>>,
    dir_name: String,
    child_dirs: HashMap<String, Rc<RefCell<Dir<'a>>>>,
    files: HashMap<String, usize>,
}

impl<'a> Dir<'a> {
    fn with_name<T: AsRef<str>>(dir_name: T, parent: Rc<RefCell<Self>>) -> Self {
        let dir = Dir {
            parent: Some(parent),
            dir_name: String::from(dir_name.as_ref()),
            ..Default::default()
        };
        dir
    }

    fn get_name(&self) -> &str {
        self.dir_name.as_str()
    }

    fn add_dir(&mut self, dir: Dir<'a>) {
        self.child_dirs
            .entry(String::from(dir.get_name()))
            .or_insert(Rc::new(RefCell::new(dir)));
    }

    fn add_file(&mut self, file: File) {
        self.files.entry(file.name).or_insert(file.size);
    }
}

// ----- Functions on Rc<RefCell<Dir>> -----
fn apply_op(dir: Rc<RefCell<Dir>>, op: FsOperation) -> Rc<RefCell<Dir>> {
    match op {
        FsOperation::cd(to) => cd(dir, to).expect("failed to apply cd command op"),
        FsOperation::ls(entries) => {
            for entry in entries {
                match entry {
                    FsEntry::Dir(dir_name) => dir
                        .borrow_mut()
                        .add_dir(Dir::with_name(dir_name, dir.clone())),
                    FsEntry::File(file) => dir.borrow_mut().add_file(file),
                }
            }
            dir
        }
    }
}

fn cd(dir: Rc<RefCell<Dir>>, to: String) -> Result<Rc<RefCell<Dir>>, ()> {
    match to.as_str() {
        ".." => dir.borrow_mut().parent.as_ref().ok_or(()).cloned(),
        "/" => Ok(get_root(dir)),
        to => dir
            .borrow_mut()
            .child_dirs
            .get(to)
            .ok_or(())
            .map(|x| x.clone()),
    }
}

fn get_root(mut dir: Rc<RefCell<Dir>>) -> Rc<RefCell<Dir>> {
    // do a cd .. until we get an error
    while let Ok(parent) = cd(dir.clone(), String::from("..")) {
        dir = parent.clone();
    }
    dir
}
// -----------------------------------------

impl<'a> FsEntrySize<'a> for Dir<'a> {
    fn get_size(&self) -> usize {
        self.child_dirs
            .values()
            .map(|child| child.borrow_mut().get_size())
            .sum::<usize>()
            + self.files.values().sum::<usize>()
    }
}

#[derive(Debug)]
pub struct File {
    name: String,
    size: usize,
}

impl<'a> FsEntrySize<'a> for File {
    fn get_size(&self) -> usize {
        self.size
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum FsOperation {
    // TODO find a way to make this &str ?
    cd(String),
    ls(Vec<FsEntry>),
}

/// Represents a filesystem entry to be inserted.
/// The parser returns a vec of these within `FsOperation::ls`.
#[derive(Debug)]
pub enum FsEntry {
    File(File),
    Dir(String),
}

mod parser {
    use crate::File;

    use super::{FsEntry, FsOperation};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, digit1},
        combinator::{all_consuming, into, map_res, rest},
        sequence::{preceded, separated_pair},
        Parser,
    };

    impl From<(&str, &str)> for FsEntry {
        fn from((size_str, file_name): (&str, &str)) -> Self {
            use std::str::FromStr;
            FsEntry::File(File {
                name: String::from(file_name),
                size: usize::from_str(size_str).expect("failed to parse file size string to file"),
            })
        }
    }

    impl From<&str> for FsEntry {
        fn from(dir_name: &str) -> Self {
            FsEntry::Dir(String::from(dir_name))
        }
    }

    fn ls_result_parser<'a>() -> impl Parser<&'a str, FsEntry, ()> {
        move |line: &'a str| {
            // Parser consumes the whole line to create an FsEntry
            let (tail, insert) = all_consuming::<_, _, (), _>(alt((
                into::<_, _, FsEntry, (), (), _>(preceded(tag("dir "), alphanumeric1)),
                into::<_, _, FsEntry, (), (), _>(separated_pair(digit1, tag(" "), rest)),
            )))
            .parse(line)
            .map_err(|_| nom::Err::Error(()))?;

            Ok((tail, insert))
        }
    }

    fn fs_oper_parser<'a>() -> impl Parser<&'a str, FsOperation, ()> {
        move |line: &'a str| {
            // Parser consumes the whole line, taking the $, then picking between an ls and a cd and mapping to the proper FsOperation enum variant
            let (tail, op) = all_consuming(preceded(
                tag::<_, _, ()>("$ "),
                alt((
                    map_res(tag("ls"), |_| Ok::<_, ()>(FsOperation::ls(vec![]))),
                    map_res(preceded(tag("cd "), rest), |s: &str| {
                        Ok::<_, ()>(FsOperation::cd(String::from(s)))
                    }),
                )),
            ))
            .parse(line)
            .map_err(|_| nom::Err::Error(()))?;

            Ok((tail, op))
        }
    }

    pub fn parse<T>(lines: T) -> Vec<FsOperation>
    where
        T: Iterator<Item = String>,
    {
        let mut lines = lines.peekable();

        let mut ops = vec![];
        while let Some(line) = lines.next() {
            let (_, op) = all_consuming(fs_oper_parser())
                .parse(line.as_str())
                .expect("could not parse FsOperation from line starting with $");
            let op = match op {
                FsOperation::cd(to) => FsOperation::cd(to),
                // Fills in ls's vec with result lines below
                FsOperation::ls(_) => {
                    // Loop, peek next line and try to parse FsInsert
                    let mut fs_entries = vec![];
                    loop {
                        if let Some(line) = lines.peek() {
                            if let Ok((_, entry)) =
                                all_consuming(ls_result_parser()).parse(line.as_str())
                            {
                                fs_entries.push(entry);
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }

                        lines.next();
                    }

                    // We should've parsed at least one ls entry
                    assert!(!fs_entries.is_empty());
                    FsOperation::ls(fs_entries)
                }
            };
            ops.push(op);
        }

        ops
    }
}

pub fn run1<'r, T>(lines: T)
where
    T: Iterator<Item = String>,
{
    let ops = parser::parse(lines);

    let mut dir = Rc::new(RefCell::new(Dir::default()));
    for op in ops {
        dir = apply_op(dir, op);
    }

    // Go to root
    let root = get_root(dir);

    let threshold = 100_000;
    #[allow(unused_variables)]
    let mut total_size = 0;
    let mut dirs = vec![root];
    while let Some(dir) = dirs.pop() {
        // Add child dirs to search
        let mut children = dir
            .borrow_mut()
            .child_dirs
            .values()
            .map(|x| x.clone())
            .collect::<Vec<Rc<RefCell<Dir>>>>();
        dirs.append(&mut children);

        // Add to total size if under threshold
        let size = dir.borrow_mut().get_size();
        if size <= threshold {
            total_size += size;
        }
    }

    println!("Summed size of dirs with size less than {threshold} is {total_size}");
}

pub fn run2<T>(lines: T)
where
    T: Iterator<Item = String>,
{
    let ops = parser::parse(lines);

    let mut dir = Rc::new(RefCell::new(Dir::default()));
    for op in ops {
        dir = apply_op(dir, op);
    }

    // Go to root
    let root = get_root(dir);

    let total_disk = 70_000_000;
    let space_needed = 30_000_000;
    let space_used = root.borrow().get_size();
    let mut dirs = vec![root];
    let mut smallest_satisfactory = usize::MAX;
    while let Some(dir) = dirs.pop() {
        // Add child dirs to search
        let mut children = dir
            .borrow_mut()
            .child_dirs
            .values()
            .map(|x| x.clone())
            .collect::<Vec<Rc<RefCell<Dir>>>>();
        dirs.append(&mut children);

        // Set new smallest satisfactory if appropriate
        let this_size = dir.borrow().get_size();
        if this_size < smallest_satisfactory
            && (total_disk + this_size)
                .checked_sub(space_used + space_needed)
                .is_some()
        {
            smallest_satisfactory = this_size;
        }
    }

    println!("Smallest dir to delete in order to satisfy {space_needed} space needed on a drive with capacity {total_disk} (current usage: {space_used}) is {smallest_satisfactory}");
}
