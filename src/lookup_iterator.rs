use std::collections::HashSet;

pub trait LookupIteratorItem: Ord + Eq + core::hash::Hash + Copy {}

impl<T> LookupIteratorItem for T where T: Ord + Eq + core::hash::Hash + Copy {}

/// An iterator that supports resolving is a value has been or
/// will be produced by the given iterator. It does this by consuming
/// the given iterator until the last produced value is
/// >= the searched value. This implies that the types iterated
/// on must be Orderable (implement Ord).
pub struct LookupIterator<TIter, TItem>
where
    TIter: Iterator<Item = TItem>,
    TItem: LookupIteratorItem,
{
    iter: TIter,
    items: HashSet<TItem>,
    last_item_seen: Option<TItem>,
}

impl<TIter, TItem> LookupIterator<TIter, TItem>
where
    TIter: Iterator<Item = TItem>,
    TItem: LookupIteratorItem,
{
    /// Creates as new LookupIterator by wrapping the provided iterator
    pub fn new(iter: TIter) -> Self {
        Self {
            iter,
            items: HashSet::new(),
            last_item_seen: None,
        }
    }

    /// Returns true if the wrapped iterator has or ever will contain the item.
    /// This function may cause more elements to be consumed from the wrapped iterator
    /// if `item` is < the last seen item so far.
    pub fn contains(&mut self, item: TItem) -> bool {
        while self.last_item_seen.is_none()
            || self
                .last_item_seen
                .is_some_and(|last_seen_item| last_seen_item < item)
        {
            self.next();
        }
        self.items.contains(&item)
    }
}

impl<TIter, TItem> Iterator for LookupIterator<TIter, TItem>
where
    TItem: LookupIteratorItem,
    TIter: Iterator<Item = TItem>,
{
    type Item = TItem;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        if let Some(next) = next {
            self.last_item_seen = Some(next.clone());
            self.items.insert(next);
        }
        return next;
    }
}
