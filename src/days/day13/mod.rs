use std::{cmp::Ordering, process::exit, str::FromStr};

use itertools::Itertools;

use super::{utils, Day};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Node {
    Parent(Vec<Node>),
    Leaf(u32),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Self::Leaf(a) = self && let Self::Leaf(b) = other {
            a.cmp(b)
        } else {
            let l1 = match self {
                Node::Leaf(l) => vec![Node::Leaf(*l)],
                Node::Parent(p) => p.clone()
            };
            let l2 = match other {
                Node::Leaf(l) => vec![Node::Leaf(*l)],
                Node::Parent(p) => p.clone()
            };

            let mut i = 0;
            while i < l1.len() {
                if i == l2.len() {
                    return std::cmp::Ordering::Greater
                } else {
                    let res = l1.get(i).unwrap().cmp(l2.get(i).unwrap());
                    if res == std::cmp::Ordering::Equal {
                        i += 1;
                    } else {
                        return res;
                    }
                }
            };
            if i == l2.len() {
                return std::cmp::Ordering::Equal
            }
            std::cmp::Ordering::Less
        }
    }
}

impl From<String> for Node {
    fn from(s: String) -> Self {
        if s.starts_with('[') {
            let mut start = 1;
            let mut i = 1;
            let mut balance = 0;

            let mut children = vec![];

            while i < s.len() {
                if s.chars().collect_vec().get(i).unwrap() == &'[' {
                    balance += 1;
                } else if s.chars().collect_vec().get(i).unwrap() == &']' {
                    balance -= 1;
                }
                i += 1;

                if balance == 0
                    && (s.chars().collect_vec().get(i).unwrap() == &','
                        || s.chars().collect_vec().get(i).unwrap() == &']')
                {
                    let child = Node::from(s.as_str()[start..i].to_string());
                    children.push(child);
                    start = i + 1;
                    i += 1;
                }
            }

            Self::Parent(children)
        } else {
            Node::Leaf(s.as_str().parse().unwrap())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Day13 {
    a: Node,
    b: Node,
}

impl Day13 {
    fn valid(&self) -> bool {
        self.a <= self.b
    }
}

impl FromStr for Day13 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut foo = s.split('\n');
        let a = Node::from(foo.next().unwrap().to_string());
        let b = Node::from(foo.next().unwrap().to_string());
        Ok(Self { a, b })
    }
}

impl Day<u32> for Day13 {
    fn run() -> Result<u32, String> {
        let nodes = utils::fetch_input(13)?
            .lines()
            .filter(|s| !s.is_empty())
            .map(|text| Node::from(text.to_string()))
            .sorted();

        let mut res = None;

        let divider1 = Node::from("[[2]]".to_string());
        let divider2 = Node::from("[[6]]".to_string());

        for (i, node) in nodes.enumerate() {
            if divider1 < node && res.is_none() {
                res = Some(i + 1);
            }
            if divider2 < node {
                res = Some(res.unwrap() * (i + 2));
                break;
            }
        }

        Ok(res.unwrap() as u32)
    }
}
