const STRATEGY: &str = include_str!("02.in");

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn play_against(&self, opponent: Hand) -> u64 {
        self.score() + self.result(opponent).score()
    }

    fn result(&self, opponent: Hand) -> MatchResult {
        match (*self, opponent) {
            (a, b) if a == b => MatchResult::Draw,
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => MatchResult::Win,
            _ => MatchResult::Loss,
        }
    }

    #[inline]
    fn score(&self) -> u64 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Hand {
    type Error = ();

    fn try_from(x: char) -> Result<Self, Self::Error> {
        match x {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl MatchResult {
    fn score(&self) -> u64 {
        match self {
            MatchResult::Win => 6,
            MatchResult::Loss => 0,
            MatchResult::Draw => 3,
        }
    }
}

fn evaluate_strategy(strategy: &str) -> u64 {
    strategy
        .lines()
        .map(|x| {
            // Convert "A B" to ('A', 'B')
            let mut it = x.split(' ');
            (
                it.next()
                    .expect("Invalid strategy.")
                    .chars()
                    .next()
                    .unwrap(),
                it.next()
                    .expect("Invalid strategy.")
                    .chars()
                    .next()
                    .unwrap(),
            )
        })
        .map(|(a, b)| -> (Hand, Hand) {
            (
                a.try_into().expect("Invalid strategy."),
                b.try_into().expect("Invalid strategy."),
            )
        })
        .fold(0, |acc, (a, b)| acc + a.play_against(b))
}

fn main() {
    println!("{}", evaluate_strategy(STRATEGY));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_strat() {
        const STRATEGY: &str = "\
A Y
B X
C Z";
        assert_eq!(evaluate_strategy(STRATEGY), 15);
    }
}
