use std::ops::RangeInclusive;

const INPUT: &str = include_str!("04.in");

fn range_contains<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    (a.contains(b.start()) && a.contains(b.end())) || (b.contains(a.start()) && b.contains(a.end()))
}

fn range_overlaps<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
}

fn parse_assignments<'a>(
    input: &'a str,
) -> impl Iterator<Item = (RangeInclusive<u8>, RangeInclusive<u8>)> + 'a {
    input.lines().map(|x| {
        // Due to formatting guarantees, this will always return
        // an iterator like [u8; 4].
        let mut it = x.split(&[',', '-']);

        // Convert it into RangeInclusive
        (
            it.next().unwrap().parse::<u8>().unwrap()..=it.next().unwrap().parse::<u8>().unwrap(),
            it.next().unwrap().parse::<u8>().unwrap()..=it.next().unwrap().parse::<u8>().unwrap(),
        )
    })
}

fn main() {
    let mut contains = parse_assignments(INPUT).filter(|(a, b)| range_contains(a, b));
    let mut overlap = parse_assignments(INPUT).filter(|(a, b)| range_overlaps(a, b));

    println!("Count containing: {}", contains.count());
    println!("Count overlapping: {}", overlap.count());
}
