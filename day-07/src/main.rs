use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn bags_containing(bag_color: String, rules: &str) -> Vec<String> {
    let bag_colors: Vec<String> = rules
        .split('\n')
        .filter(|s| s.contains(&format!(" {}", bag_color)))
        .map(|s| s.split(' ').take(2).collect::<Vec<&str>>().join(" "))
        .collect();

    let parent_bag_colors: Vec<String> = bag_colors
        .iter()
        .flat_map(|c| bags_containing(c.to_string(), rules))
        .collect();

    bag_colors
        .iter()
        .chain(parent_bag_colors.iter())
        .map(|s| s.to_string())
        .collect()
}

fn unique_bags_containing(bag_color: String, rules: &str) -> Vec<String> {
    // We're only looking for bags that are contained, so adding a leading
    // space eliminates rules that start with the bag color
    // Then pull the first two words which are the color of the containing bag
    let mut bags_containing_color = bags_containing(bag_color, rules);
    bags_containing_color.sort_unstable();
    bags_containing_color.dedup();
    bags_containing_color
}

fn bags_contained(bag_color: String, rules: &str, initial: usize) -> usize {
    lazy_static! {
        static ref BAG_COLOR_RE: Regex =
            Regex::new(r"(?P<num>\d+) (?P<color>[a-z]+ [a-z]+)").unwrap();
    }
    initial
        + rules
            .split('\n')
            .filter(|s| s.contains(&format!("{} bags contain", bag_color)))
            .map(|c| {
                BAG_COLOR_RE
                    .captures_iter(c)
                    .map(|cap| {
                        let num = cap
                            .name("num")
                            .map(|n| n.as_str().parse::<usize>().unwrap())
                            .unwrap();
                        let color = cap.name("color").unwrap().as_str();
                        num * bags_contained(String::from(color), rules, 1)
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
}

fn main() {
    let input = fs::read_to_string("./day-07/input.txt").unwrap();

    println!(
        "Part 1: {}",
        unique_bags_containing(String::from("shiny gold"), &input).len()
    );

    println!(
        "Part 2: {}",
        bags_contained(String::from("shiny gold"), &input, 0)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        let rules = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(
            unique_bags_containing(String::from("shiny gold"), rules).len(),
            4
        )
    }

    #[test]
    fn test_sample_2() {
        let rules = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(bags_contained(String::from("shiny gold"), rules, 0), 126);
    }
}
