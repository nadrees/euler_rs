use std::collections::HashSet;

/// An iterator that supports resolving is a value has been or
/// will be produced by the given iterator. It does this by consuming
/// the given iterator until the last produced value is
/// >= the searched value. This implies that the types iterated
/// on must be Orderable (implement Ord).
pub struct LookupIterator<TIter, TItem>
where
    TIter: Iterator<Item = TItem>,
{
    iter: TIter,
    items: HashSet<TItem>,
}

impl<TIter, TItem> LookupIterator<TIter, TItem>
where
    TIter: Iterator<Item = TItem>,
{
    /// Creates as new LookupIterator by wrapping the provided iterator
    pub fn new(iter: TIter) -> Self {
        Self {
            iter,
            items: HashSet::new(),
        }
    }
}

impl<TIter, TItem> Iterator for LookupIterator<TIter, TItem>
where
    TItem: Eq + core::hash::Hash + Copy,
    TIter: Iterator<Item = TItem>,
{
    type Item = TItem;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        if let Some(next) = next {
            self.items.insert(next);
        }
        return next;
    }
}
