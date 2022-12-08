use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

const INPUT: &str = include_str!("07.in");

#[derive(Debug)]
#[repr(transparent)]
struct FileTree(BTreeMap<String, u32>);

impl FileTree {
    fn from_cmds(cmds: &str) -> Self {
        let mut res = BTreeMap::new();
        let mut cwd = PathBuf::from("/");
        let mut cmds = cmds.lines().peekable();

        while let Some(cmd) = cmds.next() {
            if cmd.starts_with("$ cd") {
                let cd_dest = cmd.strip_prefix("$ cd ").unwrap();

                match cd_dest {
                    "/" => cwd = PathBuf::from("/"),
                    ".." => { cwd.pop(); },
                    x => cwd.push(x),
                }
            } else if cmd == "$ ls" {
                while let Some(file) = cmds.next_if(|&x| !x.starts_with('$')) {
                    if !file.starts_with("dir ") {
                        let (size, name) = file.split_once(' ').unwrap();

                        cwd.push(name);

                        res.insert(
                            cwd.to_string_lossy().into_owned(),
                            size.parse::<u32>().unwrap(),
                        );

                        cwd.pop();
                    }
                }
            }

            // println!("{}", cmd);
        }

        FileTree(res)
    }
}

fn main() {
    let tree = FileTree::from_cmds(INPUT);

    println!("{:#?}", tree);
}
