use std::collections::HashMap;

use reqwest::header::COOKIE;

pub fn fetch_input(day: u32) -> Result<String, String> {
    match std::env::args()
        .nth(2)
        .unwrap_or_else(|| "-s".to_string())
        .as_str()
    {
        "-s" => fetch_sample(day),
        "-r" => fetch_real(day),
        _ => unimplemented!("Only support -s, or -r as a second argument"),
    }
}

fn fetch_sample(day: u32) -> Result<String, String> {
    let formatted_path = format!("src/days/day{}/sample.txt", day);
    let path = std::path::Path::new(&formatted_path);
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

fn fetch_real(day: u32) -> Result<String, String> {
    let token = std::env::var("AOC_TOKEN").map_err(|e| e.to_string())?;
    let formatted_token = format!("session={}", token);

    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let client = reqwest::blocking::Client::new();

    client
        .get(url)
        .header(COOKIE, formatted_token)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
}

#[derive(Debug, Hash, Default, PartialEq, Eq, Copy, Clone)]
pub(crate) struct Point {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl Point {
    pub(crate) fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub(crate) fn absolute_distance(&self, other: &Self) -> usize {
        (std::cmp::max(self.x.abs_diff(other.x), 1) * std::cmp::max(self.y.abs_diff(other.y), 1))
    }
}

#[derive(Debug, Default)]
struct Tree<T> {
    arena: Vec<Node<T>>,
    root: Node<T>,
}

#[derive(Debug, Default)]
struct Node<T> {
    data: Option<T>,
    parent: Option<usize>,
    children: Vec<usize>,
}

enum TreeError {
    InvalidNodeId,
}

impl<T: Default> Tree<T> {
    fn add_child_to(&mut self, node_id: usize, child: T) -> Result<usize, TreeError> {
        let child_id = self.arena.len();
        let child_node = Node {
            data: Some(child),
            parent: Some(node_id),
            ..Default::default()
        };

        self.arena.push(child_node);

        let node = self
            .arena
            .get_mut(node_id)
            .ok_or(TreeError::InvalidNodeId)?;

        node.children.push(child_id);

        Ok(child_id)
    }

    fn get_node(&self, node_id: usize) -> Result<&Node<T>, TreeError> {
        self.arena.get(node_id).ok_or(TreeError::InvalidNodeId)
    }
}
