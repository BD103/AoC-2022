use std::fmt;

const CRATES: &str = include_str!("05/crates.in");
const STEPS: &str = include_str!("05/steps.in");

#[derive(Debug, Clone)]
struct Stacks([Vec<char>; 9]);

impl Stacks {
    #[allow(dead_code)]
    fn new() -> Self {
        Stacks([(); 9].map(|_| Vec::new()))
    }

    fn from_input(crates: &str) -> Self {
        // I know it's a Vec of Vecs, but this is only being called once per-program
        let crates = crates
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut res = [(); 9].map(|_| Vec::new());

        for y in (0..crates.len()).rev() {
            for x in 0..crates[y].len() {
                // Should be no other crates above this one, so skip forward
                if crates[y][x] == ' ' {
                    continue;
                }

                res[x].push(crates[y][x]);
            }
        }

        Stacks(res)
    }

    /// Move with the CrateMover 9000
    fn move_amount_9000(&mut self, from: usize, to: usize, amount: usize) {
        for _ in 0..amount {
            let c = self.0[from].pop().expect("Tried to pop from empty crate stack.");
            self.0[to].push(c);
        }
    }

    /// Move with CrateMover 9001
    fn move_amount_9001(&mut self, from: usize, to: usize, amount: usize) {
        let absolute_amount = self.0[from].len() - amount;
        let drain = self.0[from].drain(absolute_amount..).collect::<Vec<_>>();

        for i in drain {
            self.0[to].push(i);
        }
    }

    fn message(&self) -> String {
        let mut res = String::with_capacity(9);

        for i in self.0.iter() {
            res.push(match i.last() {
                Some(&c) => c,
                None => ' ',
            });
        }

        res
    }
}

impl fmt::Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write;

        for i in self.0.iter() {
            for c in i.iter().copied() {
                f.write_char(c)?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn parse_steps(steps: &str) -> impl Iterator<Item = (u8, u8, u8)> + '_ {
    steps.lines()
        // Strip beginning "move "
        .map(|x| &x[5..])
        .map(|x| x.split_once(" from ").unwrap())
        .map(|(a, b)| (a, b.split_once(" to ").unwrap()))
        .map(|(a, (b, c))| (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()))
}

fn main() {
    let mut stacks_9000 = Stacks::from_input(CRATES);
    let steps_9000 = parse_steps(STEPS);

    let mut stacks_9001 = stacks_9000.clone();
    let steps_9001 = parse_steps(STEPS);

    steps_9000.for_each(|(amount, from, to)| {
        // Adjust from and to to start from 0
        stacks_9000.move_amount_9000((from - 1) as usize, (to - 1) as usize, amount as usize);
    });

    println!("CrateMover 9000:\n\n{}", stacks_9000);
    println!("First message: {}", stacks_9000.message());

    steps_9001.for_each(|(amount, from, to)| {
        stacks_9001.move_amount_9001((from - 1) as usize, (to - 1) as usize, amount as usize);
    });

    println!("CrateMover 9001:\n\n{}", stacks_9001);
    println!("Second message: {}", stacks_9001.message());
}
