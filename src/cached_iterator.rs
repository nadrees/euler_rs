/// An iterator that supports being restarted. It will remember
/// the previously seen values and continue consuming the wrapped
/// iterator when needed to produce new values.
pub struct CachedIterator<TIter, TItem>
where
    TItem: Copy,
    TIter: Iterator<Item = TItem>,
{
    iter: TIter,
    items: Vec<TItem>,
}

impl<TIter, TItem> CachedIterator<TIter, TItem>
where
    TItem: Copy,
    TIter: Iterator<Item = TItem>,
{
    /// Creates a new CachedIterator by wrapping the provided iterator
    pub fn new(iter: TIter) -> Self {
        Self {
            iter,
            items: vec![],
        }
    }

    /// Returns an iterator that can be used to retrieve the values of the wrapped iterator
    pub fn iter(&mut self) -> CachedIteratorIterator<TIter, TItem> {
        CachedIteratorIterator {
            cached_iter: self,
            index: 0,
        }
    }
}

pub struct CachedIteratorIterator<'a, TIter, TItem>
where
    TItem: Copy,
    TIter: Iterator<Item = TItem>,
{
    cached_iter: &'a mut CachedIterator<TIter, TItem>,
    index: usize,
}

impl<'a, TIter, TItem> Iterator for CachedIteratorIterator<'a, TIter, TItem>
where
    TItem: Copy,
    TIter: Iterator<Item = TItem>,
{
    type Item = TItem;

    fn next(&mut self) -> Option<Self::Item> {
        let result;
        if self.index < self.cached_iter.items.len() {
            result = Some(self.cached_iter.items[self.index]);
        } else {
            result = self.cached_iter.iter.next();
            if result.is_some() {
                self.cached_iter.items.push(result.unwrap())
            }
        }
        self.index += 1;
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restart() {
        let mut iter = CachedIterator::new(0..10);
        assert_eq!(iter.items.len(), 0);

        // consume the values
        iter.iter().for_each(drop);
        assert_eq!(iter.items.len(), 10);

        // we should be able to iterate again
        iter.iter().for_each(drop);
    }
}
