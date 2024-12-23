use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};
use std::collections::{HashMap, HashSet};

#[aoc_puzzle(day = 23)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let connections = puzzle
            .input_as_str()
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .fold(HashMap::<&str, HashSet<&str>>::new(), |mut map, set| {
                map.entry(set.0).or_default().insert(set.1);
                map.entry(set.1).or_default().insert(set.0);
                map
            });
        let mut clusters = HashSet::new();
        for (c1, c1_connections) in &connections { 
            for c2 in c1_connections.iter() {
                if let Some(c2_connections) = connections.get(c2) {
                    for c3 in c2_connections {
                        if c1 == c3 {
                            continue;
                        }
                        if c1_connections.contains(c3) {
                            let mut set = vec![c1, c2, *c3];
                            set.sort_unstable();
                            clusters.insert(set);
                        }
                    }
                }
            }
        }
        clusters
            .iter()
            .filter(|c| c.iter().any(|s| s.starts_with('t')))
            .count()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let connections = puzzle
            .input_as_str()
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .fold(HashMap::<&str, HashSet<&str>>::new(), |mut map, set| {
                map.entry(set.0).or_default().insert(set.1);
                map.entry(set.1).or_default().insert(set.0);
                map
            });
        let mut clusters = HashSet::new();
        for (c1, c1_connections) in &connections { 
            for c2 in c1_connections.iter() {
                if let Some(c2_connections) = connections.get(c2) {
                    let mut network = HashSet::from([*c1, *c2]);
                    let connected_to_both = c1_connections.intersection(c2_connections);
                    for c3 in connected_to_both {
                        let c3_connections = connections.get(c3).unwrap();
                        if c3_connections.intersection(&network).count() == network.len() {
                            network.insert(c3);
                        }
                    }
                    let mut network = network.into_iter().collect::<Vec<_>>();
                    network.sort_unstable();
                    clusters.insert(network);
                }
            }
        }
        clusters.iter().max_by_key(|l| l.len()).unwrap().join(",").into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 7.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, "co,de,ka,ta".into());
    }
}
