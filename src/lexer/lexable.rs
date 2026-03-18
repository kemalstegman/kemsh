use std::iter::Peekable;

pub trait LexableCharBuffer {
    /// a view of the unconsumed `char`s. Iterating over this
    /// does not mean consuming the `char`s.
    fn chars(&self) -> impl Iterator<Item = char>;
    /// consume a number of `char`s from the buffer.
    fn consume(&mut self, chars: usize);
}

/// counts the number of times `.next()` is called but not `.peek()`.
/// this is useful for lexing because you need to count the number of
/// characters actually consumed, but sometimes there is a need for a
/// oneahead peek without consuming.
pub struct TrackingPeekable<I>
where
    I: Iterator,
{
    iter: Peekable<I>,
    next_consumed: usize,
}

impl<I> Iterator for TrackingPeekable<I>
where
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next();
        if item.is_some() {
            self.next_consumed += 1;
        }
        item
    }
}
impl<I> TrackingPeekable<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
            next_consumed: 0,
        }
    }
    pub fn next_consumed(&self) -> usize {
        self.next_consumed
    }
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.iter.peek()
    }
    pub fn peek_mut(&mut self) -> Option<&mut I::Item> {
        self.iter.peek_mut()
    }
    pub fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        let item = self.iter.next_if(func);
        if item.is_some() {
            self.next_consumed += 1;
        }
        item
    }
    pub fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        let item = self.iter.next_if_eq(expected);
        if item.is_some() {
            self.next_consumed += 1;
        }
        item
    }
}
