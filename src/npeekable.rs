pub struct NPeekable<I, const N: usize>
where
    I: Iterator,
{
    iter: I,
    peekbuf: arraydeque::ArrayDeque<I::Item, N>,
}

impl<I, const N: usize> NPeekable<I, N>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            peekbuf: arraydeque::ArrayDeque::new(),
        }
    }
    /// Returns `None` if the type is out of the bounds of the peeking buffer.
    /// Returns `Some(None)` if there is no element, because the iterator is empty.
    pub fn peek(&mut self, n: usize) -> Option<Option<&I::Item>> {
        if n >= N {
            None
        } else {
            while n >= self.peekbuf.len() {
                match self.iter.next() {
                    None => break,
                    Some(e) => match self.peekbuf.push_back(e) {
                        Ok(()) => (),
                        Err(_) => unreachable!(),
                    },
                }
            }
            Some(self.peekbuf.get(n))
        }
    }
}

impl<I, const N: usize> Iterator for NPeekable<I, N>
where
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.peekbuf.pop_front() {
            Some(e) => Some(e),
            None => self.iter.next(),
        }
    }
}
