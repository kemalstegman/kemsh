use itertools::PeekNth;

pub trait NLookahead<const LOOKAHEAD: usize>: Iterator {
    /// It is undefined behavior if `n` is larger than `LOOKAHEAD - 1`.
    fn peek_nth(&mut self, n: usize) -> Option<&Self::Item>;
    fn peek(&mut self) -> Option<&Self::Item> {
        self.peek_nth(0)
    }
    fn next_if(&mut self, f: impl FnOnce(&Self::Item) -> bool) -> Option<Self::Item> {
        if f(self.peek()?) { self.next() } else { None }
    }
}

impl<const LOOKAHEAD: usize, I: NLookahead<LOOKAHEAD>> NLookahead<LOOKAHEAD> for &mut I {
    fn peek_nth(&mut self, n: usize) -> Option<&Self::Item> {
        (**self).peek_nth(n)
    }
}

// You may ask, "why not just use PeekNth?". It is a heap allocated
// vector, but there are times where you know the maximum amount
// of lookahead you need, and thus can be stack allocated. I have
// not made that stack allocated lookahead adapter, so this is a
// compromise where it can be easily swapped out.
impl<I: Iterator, const N: usize> NLookahead<N> for PeekNth<I> {
    fn peek_nth(&mut self, n: usize) -> Option<&Self::Item> {
        PeekNth::peek_nth(self, n)
    }
}

/// If the peeked element is an error, it immediately returns it.
/// The error does not need to be Clone. Best usage is probably
/// peeking 0, then 1, then 2, and so on, not skipping any.
///
/// I imagine this could be implemented with the either type
/// in a similar and more generic fashion
pub trait ErrorBubbledNLookahead<const LOOKAHEAD: usize, T, E>:
    NLookahead<LOOKAHEAD, Item = Result<T, E>>
{
    fn bubble_peek_nth<'a>(&'a mut self, n: usize) -> Result<Option<&'a T>, E>
    where
        E: 'a,
    {
        if matches!(self.peek_nth(n), Some(Err(_))) {
            return Err(self.nth(n).unwrap().err().unwrap());
        }
        match self.peek_nth(n) {
            None => Ok(None),
            Some(Ok(x)) => Ok(Some(x)),
            Some(Err(_)) => unreachable!(),
        }
    }
    fn bubble_peek<'a>(&'a mut self) -> Result<Option<&'a T>, E>
    where
        E: 'a,
    {
        self.bubble_peek_nth(0)
    }
    fn bubble_next_if(&mut self, f: impl FnOnce(&T) -> bool) -> Result<Option<T>, E> {
        if let Some(x) = self.bubble_peek()?
            && f(x)
        {
            self.next().transpose()
        } else {
            Ok(None)
        }
    }
}

impl<const LOOKAHEAD: usize, T, E, I> ErrorBubbledNLookahead<LOOKAHEAD, T, E> for I where
    I: NLookahead<LOOKAHEAD, Item = Result<T, E>>
{
}
