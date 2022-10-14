#![feature(assert_matches)]
#[cfg(test)]
use std::assert_matches::assert_matches;
use std::{collections::HashMap, fs::read_to_string, iter::zip};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_data = read_to_string(FILENAME)?;
    let parsed = parse_text(&file_data).collect::<Vec<LineResult>>();
    println!(
        "part 1 : {}",
        corruption_points(&mut parsed.iter().cloned())
    );
    let mut completion_scores = completion_scores(&mut parsed.iter().cloned());
    completion_scores.sort();
    println!(
        "part 2 : {}",
        completion_scores[completion_scores.len() / 2]
    );
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum LineResult {
    Valid,
    Corrupted(char),
    Completed(usize),
}

const OPENERS: [char; 4] = ['(', '[', '{', '<'];
const CLOSERS: [char; 4] = [')', ']', '}', '>'];

fn is_opener(c: &char) -> bool {
    OPENERS.contains(c)
}

fn is_closer(c: &char) -> bool {
    CLOSERS.contains(c)
}

fn char_to_corruption_score(char: char) -> i32 {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Cannot score this character {}", char),
    }
}

fn char_to_completion_score(char: char) -> usize {
    match char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Cannot score this character {}", char),
    }
}

struct State {
    stack: Vec<char>,
    brackets: HashMap<char, char>,
}

impl State {
    pub fn new() -> Self {
        Self {
            brackets: zip(OPENERS, CLOSERS).collect(),
            stack: vec![],
        }
    }
}

fn parse_chunk(chars: &mut impl Iterator<Item = char>, state: &mut State) -> LineResult {
    state.stack.clear();
    for char in chars {
        if is_closer(&char) {
            if let Some(expected) = state.stack.pop() {
                if expected == char {
                    if state.stack.is_empty() {
                        return LineResult::Valid;
                    } else {
                        continue;
                    }
                } else {
                    return LineResult::Corrupted(char);
                }
            } else {
                return LineResult::Corrupted(char);
            }
        } else if is_opener(&char) {
            state.stack.push(*state.brackets.get(&char).unwrap());
        } else {
            panic!("Invalid character {}", char);
        }
    }
    let mut score = 0;
    while let Some(char) = state.stack.pop() {
        score = score * 5 + char_to_completion_score(char);
    }
    LineResult::Completed(score)
}

fn parse_line(line: &str, state: &mut State) -> LineResult {
    let mut chars = line.chars().peekable();
    while chars.peek().is_some() {
        let result = parse_chunk(&mut chars, state);
        match result {
            LineResult::Valid => continue,
            _ => return result,
        }
    }
    LineResult::Valid
}

fn parse_text(text: &str) -> impl Iterator<Item = LineResult> + '_ {
    let mut state = State::new();
    text.split("\r\n")
        .map(move |line| parse_line(line, &mut state))
}

fn corruption_points(results: &mut impl Iterator<Item = LineResult>) -> i32 {
    results.fold(0, |total, result| {
        if let LineResult::Corrupted(char) = result {
            total + char_to_corruption_score(char)
        } else {
            total
        }
    })
}

fn completion_scores(results: &mut impl Iterator<Item = LineResult>) -> Vec<usize> {
    results
        .filter_map(|result| {
            if let LineResult::Completed(n) = result {
                Some(n)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "[({(<(())[]>[[{[]{<()<>>\r\n\
[(()[<>])]({[<{<<[]>>(\r\n\
{([(<{}[<>[]}>{[]{[(<()>\r\n\
(((({<>}<{<{<>}{[]{[]{}\r\n\
[[<[([]))<([[{}[[()]]]\r\n\
[{[{({}]{}}([{[{{{}}([]\r\n\
{<[[]]>}<{[{[{[]{()[[[]\r\n\
[<(<(<(<{}))><([]([]()\r\n\
<{([([[(<>()){}]>(<<{{\r\n\
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_parse_chunk() {
        let mut state = State::new();
        assert_matches!(
            parse_chunk(&mut "()".chars(), &mut state),
            LineResult::Valid
        );
        assert_matches!(
            parse_chunk(&mut "[]".chars(), &mut state),
            LineResult::Valid
        );
        assert_matches!(
            parse_chunk(&mut "([])".chars(), &mut state),
            LineResult::Valid
        );
        assert_matches!(
            parse_chunk(&mut "{()()()}".chars(), &mut state),
            LineResult::Valid
        );
        assert_matches!(
            parse_chunk(&mut "<([{}])>".chars(), &mut state),
            LineResult::Valid
        );
        assert_matches!(
            parse_chunk(&mut "[<>({}){}[([])<>]]".chars(), &mut state),
            LineResult::Valid
        );
        assert_matches!(
            parse_chunk(&mut "(((((((((())))))))))".chars(), &mut state),
            LineResult::Valid
        );

        assert_matches!(
            parse_chunk(&mut "(]".chars(), &mut state),
            LineResult::Corrupted(_)
        );
        assert_matches!(
            parse_chunk(&mut "{()()()>".chars(), &mut state),
            LineResult::Corrupted(_)
        );
        assert_matches!(
            parse_chunk(&mut "(((()))}".chars(), &mut state),
            LineResult::Corrupted(_)
        );
        assert_matches!(
            parse_chunk(&mut "<([]){()}[{}])".chars(), &mut state),
            LineResult::Corrupted(_)
        );
    }

    #[test]
    fn test_parse_line() {
        let mut state = State::new();
        const ERRORS: [(&str, char); 5] = [
            ("{([(<{}[<>[]}>{[]{[(<()>", '}'),
            ("[[<[([]))<([[{}[[()]]]", ')'),
            ("[{[{({}]{}}([{[{{{}}([]", ']'),
            ("[<(<(<(<{}))><([]([]()", ')'),
            ("<{([([[(<>()){}]>(<<{{", '>'),
        ];
        let corrupted = TEXT.split("\r\n").filter_map(|str| {
            if let LineResult::Corrupted(c) = parse_line(str, &mut state) {
                Some((str, c))
            } else {
                None
            }
        });
        zip(corrupted, ERRORS).for_each(|((str1, c1), (str2, c2))| {
            assert_eq!(str1, str2);
            assert_eq!(c1, c2);
        });
    }

    #[test]
    fn test_corruption_points() {
        let result = corruption_points(&mut parse_text(TEXT));
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_completion_score() {
        let mut state = State::new();
        const COMPLETED: [(&str, usize); 5] = [
            ("[({(<(())[]>[[{[]{<()<>>", 288957),
            ("[(()[<>])]({[<{<<[]>>(", 5566),
            ("(((({<>}<{<{<>}{[]{[]{}", 1480781),
            ("{<[[]]>}<{[{[{[]{()[[[]", 995444),
            ("<{([{{}}[<[[[<>{}]]]>[]]", 294),
        ];
        let completed = TEXT.split("\r\n").filter_map(|str| {
            if let LineResult::Completed(n) = parse_line(str, &mut state) {
                Some((str, n))
            } else {
                None
            }
        });
        zip(completed, COMPLETED).for_each(|((str1, n1), (str2, n2))| {
            assert_eq!(str1, str2);
            assert_eq!(n1, n2);
        })
    }

    #[test]
    fn test_middle_completion_score() {
        let mut completion_scores = completion_scores(&mut parse_text(TEXT));
        completion_scores.sort();
        assert_eq!(completion_scores[completion_scores.len() / 2], 288957);
    }
}
