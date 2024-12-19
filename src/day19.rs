use std::{cmp::Reverse, collections::HashMap, ops::ControlFlow};

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{example_tests, known_input_tests};

struct Input {
    atoms: Vec<Box<[u8]>>,
    designs: Vec<Box<[u8]>>,
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let atoms = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.bytes().collect())
        .collect();
    let _ = lines.next();
    let designs = lines.map(|s| s.bytes().collect()).collect();
    Input { atoms, designs }
}

const ALPHABET: [u8; 5] = [b'w', b'u', b'b', b'g', b'r'];

struct PrefixTree {
    alphabet: [u8; 5],
    nodes: Vec<PrefixTreeNode>,
    max_depth: usize,
}

struct PrefixTreeNode {
    children: [Option<usize>; 5],
    is_leaf: bool,
}

impl PrefixTree {
    fn new(alphabet: [u8; 5]) -> Self {
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

    fn insert(&mut self, s: &[u8]) {
        let mut node = 0;
        for &c in s {
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

    fn contains(&self, s: &[u8]) -> bool {
        let mut node = 0;
        for &c in s {
            let i = self.alphabet.iter().position(|&x| x == c).unwrap();
            if let Some(next) = self.nodes[node].children[i] {
                node = next;
            } else {
                return false;
            }
        }
        self.nodes[node].is_leaf
    }

    fn find<T>(&self, key: &[u8], mut func: impl FnMut(usize) -> ControlFlow<T>) -> Option<T> {
        let mut node = &self.nodes[0];
        for (depth, &c) in key.iter().enumerate() {
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
    let mut atoms = input.atoms.iter().map(|x| &x[..]).collect::<Vec<_>>();
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
                for &atom in &atoms {
                    if let Some(suffix) = s.strip_prefix(atom) {
                        stack.push(suffix);
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
        let mut tree = PrefixTree::new(ALPHABET);
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
                if tree
                    .find(s, |len| {
                        if len == s.len() {
                            ControlFlow::Break(())
                        } else {
                            stack.push(&s[len..]);
                            ControlFlow::Continue(())
                        }
                    })
                    .is_some()
                {
                    return true;
                }
            }
            false
        })
        .count()
}

fn count_strings<'a>(
    s: &'a [u8],
    cache: &mut HashMap<&'a [u8], usize>,
    tree: &PrefixTree,
) -> usize {
    let mut accumulator = 0;
    tree.find(s, |len| {
        let suffix = &s[len..];
        if suffix.is_empty() {
            accumulator += 1;
            return ControlFlow::Continue(());
        }
        let count = if let Some(count) = cache.get(suffix) {
            *count
        } else {
            let count = count_strings(suffix, cache, tree);
            cache.insert(suffix, count);
            count
        };
        accumulator += count;
        ControlFlow::Continue(())
    })
    .unwrap_or(accumulator)
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> usize {
    let tree = {
        let mut tree = PrefixTree::new(ALPHABET);
        for atom in &input.atoms {
            tree.insert(atom);
        }
        tree
    };

    let mut cache = HashMap::new();

    input
        .designs
        .iter()
        .map(move |design| {
            // #[cfg(debug_assertions)]
            // println!("{design}");
            count_strings(design, &mut cache, &tree)
        })
        .sum()
}

#[aoc(day19, part2, part2ciro)]
fn part2_ciro(input: &Input) -> usize {
    let tree = {
        let mut tree = PrefixTree::new(ALPHABET);
        for atom in &input.atoms {
            tree.insert(atom);
        }
        tree
    };

    input
        .designs
        .iter()
        .map(move |design| {
            let mut possibilities = vec![0; design.len() + 1];
            possibilities[0] = 1;
            for start in 0..design.len() {
                let s = &design[start..];
                tree.find::<()>(s, |len| {
                    possibilities[start + len] += possibilities[start];
                    ControlFlow::Continue(())
                });
            }
            possibilities[design.len()]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_tree() {
        let mut tree = PrefixTree::new([b'a', b'b', b'c', b'd', b'e']);
        tree.insert(b"ab");
        tree.insert(b"abc");
        tree.insert(b"aaab");
        tree.insert(b"b");
        tree.insert(b"cacca");
        assert!(tree.contains(b"ab"));
        assert!(tree.contains(b"abc"));
        assert!(tree.contains(b"aaab"));
        assert!(tree.contains(b"b"));
        assert!(tree.contains(b"cacca"));

        assert!(!tree.contains(b""));
        assert!(!tree.contains(b"a"));
        assert!(!tree.contains(b"ac"));
        assert!(!tree.contains(b"abca"));
        assert!(!tree.contains(b"c"));
    }

    #[test]
    fn prefix_tree_find() {
        let mut tree = PrefixTree::new([b'a', b'b', b'c', b'd', b'e']);
        tree.insert(b"aaaaaaaaa");

        let mut counter = 0;
        tree.find::<()>(b"aaaaaaaaa", |_| {
            counter += 1;
            ControlFlow::Continue(())
        });
        assert_eq!(counter, 1);

        tree.insert(b"a");
        tree.insert(b"aa");
        tree.insert(b"aaa");
        tree.insert(b"aaaa");
        tree.insert(b"aaaaa");
        tree.insert(b"aaaaaa");
        tree.insert(b"aaaaaaa");
        tree.insert(b"aaaaaaaa");
        let mut counter = 0;
        tree.find::<()>(b"aaaaaaaaa", |_| {
            counter += 1;
            ControlFlow::Continue(())
        });
        assert_eq!(counter, 9);
    }

    #[test]
    fn bad_cases() {
        // at one point, the trie-based solution marked these as a valid designs, but they are not:
        let designs: [&[u8]; 2] = [
            b"wrgwrbbrrwuwgrubrbrgrurwggrubwgbwgwruwwbugurwrubwgbwgbgrwrb",
            b"wrrwrbbrrwrwugwggggwuwbrbrwuwbruurgwwuuwrb",
        ];
        let input = parse(include_str!("../input/2024/day19.txt"));
        let tree = {
            let mut tree = PrefixTree::new(ALPHABET);
            for atom in &input.atoms {
                tree.insert(atom);
            }
            tree
        };
        assert!(!tree.contains(b"wrgw"));
        for design in designs {
            println!("{design:?}");
            let mut stack = vec![&design[..]];
            while let Some(s) = stack.pop() {
                if s.is_empty() {
                    assert!(false, "design should not be valid: {:?}", design);
                }
                tree.find::<()>(s, |len| {
                    let (a, b) = s.split_at(len);
                    println!("{a:?} {b:?}");
                    assert!(
                        input.atoms.iter().any(|atom| a == atom.as_ref()),
                        "{a:?} should not match"
                    );
                    stack.push(&s[len..]);
                    ControlFlow::Continue(())
                });
            }
        }
    }

    #[test]
    fn count_test() {
        let atoms: [&[u8]; 8] = [b"r", b"wr", b"b", b"g", b"bwu", b"rb", b"gb", b"br"];
        let tree = {
            let mut tree = PrefixTree::new(ALPHABET);
            for atom in atoms {
                tree.insert(atom);
            }
            tree
        };
        let mut cache = HashMap::new();
        assert_eq!(count_strings(b"u", &mut cache, &tree), 0);
        assert_eq!(count_strings(b"r", &mut cache, &tree), 1);
        assert_eq!(count_strings(b"wrr", &mut cache, &tree), 1);
        assert_eq!(count_strings(b"brwrr", &mut cache, &tree), 2);
        assert_eq!(count_strings(b"ubwu", &mut cache, &tree), 0);
        assert_eq!(count_strings(b"rrbgbr", &mut cache, &tree), 6);
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
    part2 => 16,
    part2_ciro => 16,
}

known_input_tests! {
    input: include_str!("../input/2024/day19.txt"),
    part1 => 265,
    part1_prefix_tree => 265,
    part2 => 752461716635602,
    part2_ciro => 752461716635602,
}
