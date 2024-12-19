use std::{cmp::Reverse, ops::ControlFlow};

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{example_tests, known_input_tests};

struct Input {
    atoms: Vec<String>,
    designs: Vec<String>,
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let atoms = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let _ = lines.next();
    let designs = lines.map(|s| s.to_string()).collect();
    Input { atoms, designs }
}

struct PrefixTree {
    alphabet: Vec<char>,
    nodes: Vec<PrefixTreeNode>,
    max_depth: usize,
}

struct PrefixTreeNode {
    children: [Option<usize>; 5],
    is_leaf: bool,
}

impl PrefixTree {
    fn new(alphabet: Vec<char>) -> Self {
        let nodes = vec![PrefixTreeNode {
            children: [None; 5],
            is_leaf: false,
        }];
        Self {
            alphabet,
            nodes,
            max_depth: 0,
        }
    }

    fn insert(&mut self, s: &str) {
        let mut node = 0;
        for c in s.chars() {
            let i = self.alphabet.iter().position(|&x| x == c).unwrap();
            if let Some(next) = self.nodes[node].children[i] {
                node = next;
            } else {
                let next = self.nodes.len();
                self.nodes[node].children[i] = Some(next);
                self.nodes.push(PrefixTreeNode {
                    children: [None; 5],
                    is_leaf: false,
                });
                node = next;
            }
        }
        self.nodes[node].is_leaf = true;
        self.max_depth = self.max_depth.max(s.len());
    }

    fn contains(&self, s: &str) -> bool {
        let mut node = 0;
        for c in s.chars() {
            let i = self.alphabet.iter().position(|&x| x == c).unwrap();
            if let Some(next) = self.nodes[node].children[i] {
                node = next;
            } else {
                return false;
            }
        }
        self.nodes[node].is_leaf
    }

    fn find<T>(&self, key: &str, mut func: impl FnMut(usize) -> ControlFlow<T>) -> Option<T> {
        let mut node = &self.nodes[0];
        for (depth, c) in key.chars().enumerate() {
            let i = self.alphabet.iter().position(|&x| x == c).unwrap();
            if let Some(next) = node.children[i] {
                node = &self.nodes[next];
                if node.is_leaf {
                    debug_assert!(self.contains(&key[..=depth]));
                    match func(depth + 1) {
                        ControlFlow::Continue(_) => {}
                        ControlFlow::Break(value) => return Some(value),
                    }
                }
            } else {
                return None;
            }
        }
        None
    }
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    let mut atoms = input.atoms.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    atoms.sort_unstable_by_key(|x| Reverse(x.len()));
    input
        .designs
        .iter()
        .filter(|design| {
            let mut stack = vec![&design[..]];
            while let Some(s) = stack.pop() {
                if s.is_empty() {
                    return true;
                }
                for atom in &atoms {
                    if s.starts_with(atom) {
                        stack.push(&s[atom.len()..]);
                    }
                }
            }
            false
        })
        .count()
}

#[aoc(day19, part1, part1_prefix_tree)]
fn part1_prefix_tree(input: &Input) -> usize {
    let tree = {
        let mut tree = PrefixTree::new(['w', 'u', 'b', 'r', 'g'].into());
        for atom in &input.atoms {
            tree.insert(atom);
        }
        tree
    };
    input
        .designs
        .iter()
        .filter(|design| {
            let mut stack = vec![&design[..]];
            while let Some(s) = stack.pop() {
                if let Some(_) = tree.find(s, |len| {
                    if len == s.len() {
                        ControlFlow::Break(())
                    } else {
                        stack.push(&s[len..]);
                        ControlFlow::Continue(())
                    }
                }) {
                    return true;
                }
            }
            false
        })
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_tree() {
        let mut tree = PrefixTree::new(vec!['a', 'b', 'c']);
        tree.insert("ab");
        tree.insert("abc");
        tree.insert("aaab");
        tree.insert("b");
        tree.insert("cacca");
        assert!(tree.contains("ab"));
        assert!(tree.contains("abc"));
        assert!(tree.contains("aaab"));
        assert!(tree.contains("b"));
        assert!(tree.contains("cacca"));

        assert!(!tree.contains(""));
        assert!(!tree.contains("a"));
        assert!(!tree.contains("ac"));
        assert!(!tree.contains("abca"));
        assert!(!tree.contains("c"));
    }

    #[test]
    fn prefix_tree_find() {
        let mut tree = PrefixTree::new(vec!['a', 'b', 'c']);
        tree.insert("aaaaaaaaa");

        let mut counter = 0;
        tree.find::<()>("aaaaaaaaa", |_| {
            counter += 1;
            ControlFlow::Continue(())
        });
        assert_eq!(counter, 1);

        tree.insert("a");
        tree.insert("aa");
        tree.insert("aaa");
        tree.insert("aaaa");
        tree.insert("aaaaa");
        tree.insert("aaaaaa");
        tree.insert("aaaaaaa");
        tree.insert("aaaaaaaa");
        let mut counter = 0;
        tree.find::<()>("aaaaaaaaa", |_| {
            counter += 1;
            ControlFlow::Continue(())
        });
        assert_eq!(counter, 9);
    }

    #[test]
    fn bad_cases() {
        // at one point, the trie-based solution marked these as a valid designs, but they are not:
        let designs = [
            "wrgwrbbrrwuwgrubrbrgrurwggrubwgbwgwruwwbugurwrubwgbwgbgrwrb",
            "wrrwrbbrrwrwugwggggwuwbrbrwuwbruurgwwuuwrb",
        ];
        let input = parse(include_str!("../input/2024/day19.txt"));
        let tree = {
            let mut tree = PrefixTree::new(['w', 'u', 'b', 'r', 'g'].into());
            for atom in &input.atoms {
                tree.insert(atom);
            }
            tree
        };
        assert!(!tree.contains("wrgw"));
        for design in designs {
            println!("{design}");
            let mut stack = vec![&design[..]];
            while let Some(s) = stack.pop() {
                if s.is_empty() {
                    assert!(false, "design should not be valid: {}", design);
                }
                tree.find::<()>(s, |len| {
                    let (a, b) = s.split_at(len);
                    println!("{a} {b}");
                    assert!(
                        input.atoms.iter().any(|atom| a == atom),
                        "{a} should not match"
                    );
                    stack.push(&s[len..]);
                    ControlFlow::Continue(())
                });
            }
        }
    }
}

example_tests! {
    "
    r, wr, b, g, bwu, rb, gb, br

    brwrr
    bggr
    gbbr
    rrbgbr
    ubwu
    bwurrg
    brgr
    bbrgwb
    ",

    part1 => 6,
    part1_prefix_tree => 6,
}

known_input_tests! {
    input: include_str!("../input/2024/day19.txt"),
    part1 => 265,
    part1_prefix_tree => 265,
}
