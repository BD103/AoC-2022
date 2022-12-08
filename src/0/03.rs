#[path = "03/chunk.rs"]
mod chunk;

use chunk::chunk_three;

const INPUT: &str = include_str!("03.in");

fn split_text(s: &str) -> (&str, &str) {
    let middle = s.len() / 2;

    (&s[..middle], &s[middle..])
}

fn priority(x: char) -> u8 {
    match x {
        // Convert a-z to 1-26
        'a'..='z' => x as u8 - 96,
        // Convert A-Z to 27-52
        'A'..='Z' => x as u8 - 38,
        _ => unreachable!("Invalid char for priority {:?}", x),
    }
}

fn find_similar(left: &str, right: &str) -> char {
    // Iterate over all characters in left string
    for c in left.chars() {
        if right.contains(c) {
            return c;
        }
    }

    panic!(
        "Given strings that do not share a char. {:?} and {:?}",
        left, right
    );
}

// Tried doing recursion, but this is simpler
fn find_similar_3(a: &str, b: &str, c: &str) -> char {
    for ac in a.chars() {
        for bc in b.chars() {
            if ac == bc && c.contains(bc) {
                return ac;
            }
        }
    }

    panic!(
        "Given strings that do not share a char. {:?}, {:?}, and {:?}.",
        a, b, c
    );
}

fn sum_rucksack_priority(input: &str) -> u64 {
    input
        .lines()
        .map(split_text)
        .map(|(left, right)| find_similar(left, right))
        .map(priority)
        .map(|x| x as u64)
        .sum()
}

fn sum_rucksack_groups(input: &str) -> u64 {
    chunk::chunk_three(input)
        .map(|x| find_similar_3(x[0], x[1], x[2]))
        .map(priority)
        .map(|x| x as u64)
        .sum()
}

fn main() {
    println!("Individual Priority Sum: {}", sum_rucksack_priority(INPUT));
    println!("Groups Priority Sum: {}", sum_rucksack_groups(INPUT));
}

#[cfg(test)]
mod tests {
    #[test]
    fn split_text() {
        use super::split_text;

        assert_eq!(split_text("hi"), ("h", "i"));
        // Right side will always be equal or +1 greater than left
        assert_eq!(split_text("h i"), ("h", " i"));

        // Example from AoC
        assert_eq!(
            split_text("vJrwpWtwJgWrhcsFMMfFFhFp"),
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
        );
    }

    #[test]
    fn priority() {
        ('a'..='z')
            .chain('A'..='Z')
            .enumerate()
            .map(|(i, x)| ((i + 1) as u8, x))
            .for_each(|(i, x)| assert_eq!(super::priority(x), i));
    }

    #[test]
    fn find_similar() {
        use super::find_similar;

        assert_eq!(find_similar("abc", "dc"), 'c');
        assert_eq!(find_similar("vJrwpWtwJgWr", "hcsFMMfFFhFp"), 'p')
    }

    #[test]
    fn sum_rucksack_priority() {
        use super::sum_rucksack_priority;

        const INPUT_1: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(sum_rucksack_priority(INPUT_1), 157);
    }
}
