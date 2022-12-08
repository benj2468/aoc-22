use std::collections::HashMap;
use std::collections::HashSet;

use super::utils;
use super::Day;

//     fn interpret_line(mut self, line: &str) -> Option<String> {
//         if line.starts_with("$ cd") {
//             let dir = line.split_at(5).1.to_string();
//             Some(dir)
//         } else if !line.starts_with("$ ls") {
//             let parts: Vec<&str> = line.split(' ').collect();
//             let size = *parts.get(0).unwrap();
//             let name = parts.get(1).unwrap().to_string();
//             let parent = &Box::new(self);
//             let child = if size == "dir" {
//                 File::child(&parent)
//             } else {
//                 let size = size.parse().expect("Size should be a number");
//                 File::with_size(&parent, size)
//             };
//             self.children.insert(name, child);

//             None
//         } else if line.starts_with("$ ls") {
//             return None;
//         } else {
//             unimplemented!("Unparsable line")
//         }
//     }
// }

const MAX_SIZE: u32 = 100000;
const FS_SPACE: u32 = 70000000;
const NEEDED_SPACE: u32 = 30000000;

pub struct Day7 {}
use std::cmp::Reverse;

impl Day<u32> for Day7 {
    fn run() -> Result<u32, String> {
        let input = utils::fetch_input(7)?;

        let mut path: Vec<u32> = vec![0];
        let mut sum = 0;
        let mut heap = std::collections::BinaryHeap::new();

        for line in input.lines() {
            if line.starts_with("$ cd") {
                let dir = line.split_at(5).1;
                match dir {
                    ".." => {
                        let s = path.pop().unwrap();
                        heap.push(Reverse(s));
                        if s <= MAX_SIZE {
                            sum += s;
                        }
                        let last = path.last_mut().unwrap();
                        *last += s;
                    }
                    "/" => {
                        path.clear();
                        path.push(0);
                    }
                    _ => path.push(0),
                }
            } else if line.starts_with("dir ") || line.starts_with("$ ls") {
                continue;
            } else {
                let parts: Vec<&str> = line.split(' ').collect();
                let size: u32 = parts.get(0).unwrap().parse().unwrap();
                let last = path.last_mut().unwrap();
                *last += size;
            }
        }

        while path.len() > 1 {
            let s = path.pop().unwrap();
            heap.push(Reverse(s));
            if s <= MAX_SIZE {
                sum += s;
            }
            let last = path.last_mut().unwrap();
            *last += s;
        }
        let true_needed = NEEDED_SPACE - (FS_SPACE - path.get(0).unwrap());

        loop {
            let smallest = heap.pop().unwrap().0;
            if smallest >= true_needed {
                return Ok(smallest);
            }
        }
    }
}
