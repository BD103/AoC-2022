const INPUT: &str = include_str!("01.in");

fn find_sums(input: &str) -> Vec<u64> {
    let mut sums = Vec::new();
    let mut buffer = Vec::with_capacity(4);

    for line in input.lines() {
        if line == "" {
            // Add sum of buffer to response, empty buffer
            sums.push(buffer.iter().sum());
            buffer.clear();
        } else {
            // Parse line as u64 and add to buffer
            buffer.push(line.parse::<u64>().unwrap());
        }
    }

    // Add final buffer
    if !buffer.is_empty() {
        sums.push(buffer.iter().sum());
        // Don't bother clearing buffer, as it's about to be dropped
    }

    sums
}

fn top_3_largest(numbers: &[u64]) -> [u64; 3] {
    let mut res = [0; 3];

    for n in numbers {
        if *n >= res[0] {
            res[2] = res[1];
            res[1] = res[0];
            res[0] = *n;
        } else if *n >= res[1] {
            res[2] = res[1];
            res[1] = *n;
        } else if *n > res[2] {
            res[2] = *n;
        }
    }

    res
}

fn main() {
    let sums = find_sums(INPUT);

    let max = sums.iter().max().unwrap();
    println!("Max: {}", max);

    let largest_3 = top_3_largest(&sums);
    println!("Top 3 Largest: {:?}", largest_3);
    println!("Top 3 Largest Sum: {}", largest_3.iter().sum::<u64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums() {
        const TEST_INPUT: &str = "\
2
3

6

1
2

8\n";

        assert_eq!(&find_sums(TEST_INPUT), &[5, 6, 3, 8]);
    }

    #[test]
    fn largest() {
        assert_eq!(top_3_largest(&[4, 2, 3, 1]), [4, 3, 2],);

        assert_eq!(top_3_largest(&[1, 45, 2]), [45, 2, 1],);
    }
}
