/// A class that can compactly track a series of individual bits
pub struct BitArray {
    buckets: Vec<u128>,
    size: u128,
}

impl BitArray {
    /// Creates a new BitArray of the specified size initialized to all zero
    pub fn new(size: u128) -> Self {
        let needs_extra = size % 128 != 0;
        let mut vec_len = size / 128;
        if needs_extra {
            vec_len += 1;
        }
        Self {
            buckets: vec![0; (vec_len).try_into().unwrap()],
            size,
        }
    }

    /// Gets the value of the bit at the index. panics if index >= size
    pub fn get(&self, index: u128) -> bool {
        assert!(index < self.size);

        let bucket_idx = self.get_bucket_index(index);
        let bucket = self.buckets[bucket_idx];

        let idx = index % 128;
        return (bucket >> idx & 1) == 1;
    }

    /// Returns the logic length of the BitArray, i.e. the number of bits this BitArray represents
    pub fn len(&self) -> u128 {
        self.size
    }

    /// Sets the specified bit to the value. panics if index >= size.
    pub fn set(&mut self, index: u128, value: bool) -> () {
        if self.get(index) != value {
            self.toggle_index(index);
        }
    }

    /// Toggle the entire BitArray
    pub fn toggle(&mut self) -> () {
        for i in 0..self.buckets.len() {
            self.buckets[i] = !self.buckets[i];
        }
    }

    /// Toggle the bit at the given index. panics if index >= size.
    pub fn toggle_index(&mut self, index: u128) -> () {
        assert!(index < self.size);

        let bucket_idx = self.get_bucket_index(index);
        let mut bucket = self.buckets[bucket_idx];

        let idx = index % 128;
        bucket ^= 1 << idx;
        self.buckets[bucket_idx] = bucket;
    }

    fn get_bucket_index(&self, index: u128) -> usize {
        (index / 128).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocates_right_size() {
        let mut bs = BitArray::new(10);
        assert_eq!(1, bs.buckets.len());

        bs = BitArray::new(128);
        assert_eq!(1, bs.buckets.len());

        bs = BitArray::new(129);
        assert_eq!(2, bs.buckets.len());
    }

    #[test]
    fn test_get() {
        let mut bs = BitArray::new(2);
        bs.buckets[0] = 2; // [0, 1]

        assert_eq!(false, bs.get(0));
        assert_eq!(true, bs.get(1));
    }

    #[test]
    #[should_panic]
    fn test_get_eq_size() {
        let bs = BitArray::new(1);
        bs.get(1);
    }

    #[test]
    #[should_panic]
    fn test_get_gt_size() {
        let bs = BitArray::new(1);
        bs.get(2);
    }

    #[test]
    fn test_set() {
        let mut bs = BitArray::new(2);
        bs.set(0, true);
        assert_eq!(1, bs.buckets[0]);

        bs.set(0, true);
        assert_eq!(1, bs.buckets[0]);

        bs.set(1, true);
        assert_eq!(3, bs.buckets[0]);
    }

    #[test]
    #[should_panic]
    fn test_set_index_eq_size() {
        let mut bs = BitArray::new(1);
        bs.set(1, true);
    }

    #[test]
    #[should_panic]
    fn test_set_index_gt_size() {
        let mut bs = BitArray::new(1);
        bs.set(1, true);
    }

    #[test]
    fn test_toggle() {
        let mut bs = BitArray::new(1);
        assert_eq!(0, bs.buckets[0]);

        bs.toggle();
        assert_eq!(!0, bs.buckets[0]);

        bs.toggle();
        assert_eq!(0, bs.buckets[0]);
    }
}
