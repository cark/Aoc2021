//#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::rc::Rc;

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_data = read_to_string(FILENAME)?;
    let prob = Prob::from_string(&file_data);
    println!("Part 1 : {}", prob.recursive_score(10));
    println!("Part 2 : {}", prob.recursive_score(40));
    Ok(())
}

type ScoreMap = HashMap<char, usize>;
type Rules = HashMap<(char, char), char>;

struct Prob {
    template: Vec<char>,
    rules: Rules,
}

impl Prob {
    pub fn from_string(text: &str) -> Self {
        let mut lines = text.trim().lines();
        let template = lines.next().unwrap().trim().chars().collect::<Vec<char>>();
        lines.next();
        let mut rules = HashMap::default();
        for line in lines {
            let mut parts = line.trim().split(" -> ");
            let mut pair_chars = parts.next().unwrap().chars();
            let pair = (pair_chars.next().unwrap(), pair_chars.next().unwrap());
            let insertion_char = parts.next().unwrap().chars().next().unwrap();
            rules.insert(pair, insertion_char);
        }
        Prob { template, rules }
    }

    pub fn recursive_score(&self, recursion_count: usize) -> usize {
        fn merge_scores(into: &mut ScoreMap, other: &ScoreMap) {
            for (char, score) in other.iter() {
                into.insert(*char, into.get(char).unwrap_or(&0) + score);
            }
        }
        fn inc_score(into: &mut ScoreMap, char: char) {
            into.insert(char, into.get(&char).unwrap_or(&0) + 1);
        }

        type Cache = HashMap<(usize, (char, char)), Rc<ScoreMap>>;

        fn recurse(
            cache: &mut Cache,
            rules: &Rules,
            depth_left: usize,
            pair: (char, char),
        ) -> Rc<ScoreMap> {
            let mut scores = ScoreMap::default();
            if depth_left > 0 {
                if let Some(insert_char) = rules.get(&pair) {
                    inc_score(&mut scores, *insert_char);
                    merge_scores(
                        &mut scores,
                        &call_cached(cache, rules, depth_left - 1, (pair.0, *insert_char)),
                    );
                    merge_scores(
                        &mut scores,
                        &call_cached(cache, rules, depth_left - 1, (*insert_char, pair.1)),
                    );
                } else {
                    panic!("No insert char");
                }
            }
            Rc::new(scores)
        }

        fn call_cached(
            cache: &mut Cache,
            rules: &Rules,
            depth_left: usize,
            pair: (char, char),
        ) -> Rc<ScoreMap> {
            let cache_key = (depth_left, pair);
            if let Some(scores) = cache.get(&cache_key) {
                scores.clone()
            } else {
                let score = recurse(cache, rules, depth_left, pair);
                cache.insert(cache_key, score);
                cache.get(&cache_key).unwrap().clone()
            }
        }

        
        let mut vd = VecDeque::with_capacity(3);
        let mut scores = ScoreMap::default();
        let mut cache = Cache::default();
        for char in &self.template {
            inc_score(&mut scores, *char);
            vd.push_back(*char);
            if vd.len() > 2 {
                vd.pop_front();
            }
            if vd.len() == 2 {
                merge_scores(
                    &mut scores,
                    &call_cached(&mut cache, &self.rules, recursion_count, (vd[0], vd[1])),
                );
            }
        }
        let (min, max) = scores
            .values()
            .fold((usize::MAX, usize::MIN), |(min, max), &score| {
                (min.min(score), max.max(score))
            });
        max - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prob_from_string() {
        let prob = Prob::from_string(SAMPLE);
        assert_eq!(prob.template, ['N', 'N', 'C', 'B']);
        assert_eq!(prob.rules.len(), 16);
        assert_eq!(*prob.rules.get(&('N', 'C')).unwrap(), 'B');
    }

    #[test]
    fn test_recursive() {
        let prob = Prob::from_string(SAMPLE);
        assert_eq!(prob.recursive_score(10), 1588);
        assert_eq!(prob.recursive_score(40), 2188189693529);
    }

    const SAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
}
