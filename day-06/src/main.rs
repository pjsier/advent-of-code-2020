use std::fs;

fn unique_answers(answers: &str) -> Vec<char> {
    let mut all_answers: Vec<char> = answers
        .split('\n')
        .flat_map(|s| s.chars())
        .collect::<Vec<char>>();
    all_answers.sort_unstable();
    all_answers.dedup();
    all_answers
}

fn answers_for_group_every(answers: &str) -> usize {
    let num_answers = answers.split('\n').count();
    let unique_answer_chars = unique_answers(answers);
    unique_answer_chars
        .iter()
        .filter(|u_a| answers.split('\n').filter(|a| a.contains(**u_a)).count() == num_answers)
        .count()
}

fn main() {
    let input = fs::read_to_string("./day-06/input.txt").unwrap();

    println!(
        "Part 1: {}",
        input
            .split("\n\n")
            .map(|g| unique_answers(g).len())
            .sum::<usize>()
    );

    println!(
        "Part 2: {}",
        input
            .split("\n\n")
            .map(answers_for_group_every)
            .sum::<usize>()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answers_for_group_any() {
        let answers = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let answers_any: usize = answers.split("\n\n").map(|g| unique_answers(g).len()).sum();
        assert_eq!(unique_answers("abbc"), vec!['a', 'b', 'c']);
        assert_eq!(answers_any, 11);
    }

    #[test]
    fn test_answers_for_group_every() {
        let answers = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let answer_group_0 = "a
b
c";
        let answers_every: usize = answers
            .split("\n\n")
            .map(|g| answers_for_group_every(g))
            .sum();
        assert_eq!(answers_for_group_every(answer_group_0), 0);
        assert_eq!(answers_every, 6);
    }
}
