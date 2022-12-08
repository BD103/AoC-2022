pub struct Chunk3<I> {
    it: I,
}

impl<I> Iterator for Chunk3<I>
where
    I: Iterator,
{
    type Item = [<I as Iterator>::Item; 3];

    fn next(&mut self) -> Option<Self::Item> {
        const ERROR: &str = "Given amount of items in iterator are not divisible by 3.";
        Some([
            self.it.next()?,
            self.it.next().expect(ERROR),
            self.it.next().expect(ERROR),
        ])
    }
}

pub fn chunk_three(s: &str) -> impl Iterator<Item = [&str; 3]> {
    // Assert divisible by 3
    // If this is not checked, then the iterator might panic
    debug_assert_eq!(s.lines().count() % 3, 0);

    Chunk3 { it: s.lines() }
}
